//!  # cloud_scanner_cli
//!
//!  A command line application that performs inventory of your cloud account and combines it with Boavizta API  to return an estimation of its environmental impact.
//!

use crate::model::ExecutionStatistics;
use crate::usage_location::*;
use aws_inventory::*;
use boavizta_api_v1::*;
use cloud_inventory::*;
use cloud_resource::*;
use impact_provider::ImpactProvider;
use impact_provider::{CloudResourceWithImpacts, ImpactsSummary};
use metric_exporter::*;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;
use pkg_version::*;
use std::time::{Instant, Duration};
pub mod aws_inventory;
pub mod boavizta_api_v1;
pub mod cloud_inventory;
pub mod cloud_resource;
pub mod impact_provider;
pub mod metric_exporter;
pub mod metric_server;
pub mod usage_location;
pub mod model;

use anyhow::{Context, Result};

async fn standard_scan(
    hours_use_time: &f32,
    tags: &[String],
    aws_region: &str,
    api_url: &str,
) -> Result<Vec<CloudResourceWithImpacts>> {


    let start = Instant::now();

    let inventory: AwsInventory = AwsInventory::new(aws_region).await;
    let cloud_resources: Vec<CloudResource> = inventory
        .list_resources(tags)
        .await
        .context("Cannot perform resouces inventory")?;

    let inventory_duration = start.elapsed();

    let impact_start = Instant::now();
    let api: BoaviztaApiV1 = BoaviztaApiV1::new(api_url);
    let res = api
        .get_impacts(cloud_resources, hours_use_time)
        .await
        .context("Failure while retrieving impacts")?;
    let impact_duration = impact_start.elapsed();

    let total_duration =  start.elapsed();

    let stats = ExecutionStatistics {
        inventory_duration,
        impact_duration,
        total_duration
    };

    info!("{}",stats);

    Ok(res)
}

/// Returns default impacts as json
pub async fn get_default_impacts_as_json_string(
    hours_use_time: &f32,
    tags: &[String],
    aws_region: &str,
    api_url: &str,
) -> Result<String> {
    let instances_with_impacts = standard_scan(hours_use_time, tags, aws_region, api_url)
        .await
        .context("Cannot perform standard scan")?;

    Ok(serde_json::to_string(&instances_with_impacts)?)
}

/// Returns  impacts as metrics
pub async fn get_default_impacts_as_metrics(
    hours_use_time: &f32,
    tags: &[String],
    aws_region: &str,
    api_url: &str,
) -> Result<String> {
    let instances_with_impacts = standard_scan(hours_use_time, tags, aws_region, api_url)
        .await
        .context("Cannot perform standard scan")?;

    let usage_location = UsageLocation::from(aws_region);
    let summary: ImpactsSummary = ImpactsSummary::new(
        String::from(aws_region),
        usage_location.iso_country_code,
        instances_with_impacts,
        (*hours_use_time).into(),
    );
    debug!("Summary: {:#?}", summary);

    let metrics = get_metrics(&summary).with_context(|| {
        format!(
            "Unable to get default impacts as metrics for {}",
            aws_region
        )
    })?;

    Ok(metrics)
}

/// Prints  impacts  to standard output in json format
pub async fn print_default_impacts_as_json(
    hours_use_time: &f32,
    tags: &[String],
    aws_region: &str,
    api_url: &str,
) -> Result<()> {
    let j = get_default_impacts_as_json_string(hours_use_time, tags, aws_region, api_url).await?;
    println!("{}", j);
    Ok(())
}

/// Prints impacts  to standard output as metrics in prometheus format
pub async fn print_default_impacts_as_metrics(
    hours_use_time: &f32,
    tags: &[String],
    aws_region: &str,
    api_url: &str,
) -> Result<()> {
    let metrics = get_default_impacts_as_metrics(hours_use_time, tags, aws_region, api_url).await?;
    println!("{}", metrics);
    Ok(())
}

/// List instances and metadata to standard output
pub async fn show_inventory(tags: &[String], aws_region: &str) -> Result<()> {
    let start = Instant::now();

    let inventory: AwsInventory = AwsInventory::new(aws_region).await;
    let cloud_resources: Vec<CloudResource> = inventory
        .list_resources(tags)
        .await
        .context("Cannot perform inventory.")?;
    let json_inventory: String =
        serde_json::to_string(&cloud_resources).context("Cannot format inventory as json")?;

    let stats = ExecutionStatistics{
        inventory_duration : start.elapsed(),
        impact_duration : Duration::from_millis(0),
        total_duration : start.elapsed(),
    };
    warn!("{:?}",stats);
    println!("{}", json_inventory);
    Ok(())
}

/// Starts a server that exposes metrics http like <http://localhost:8000/metrics?aws-region=eu-west-1>
pub async fn serve_metrics(api_url: &str) -> Result<()> {
    let config = metric_server::Config {
        boavizta_url: api_url.to_string(),
    };
    warn!("Starting metric server.");
    metric_server::run(config).await?;
    Ok(())
}
/// Return current version of the cloud-scanner-cli crate
pub fn get_version() -> String {
    const MAJOR: u32 = pkg_version_major!();
    const MINOR: u32 = pkg_version_minor!();
    const PATCH: u32 = pkg_version_patch!();
    format!("{}.{}.{}", MAJOR, MINOR, PATCH)
}

#[tokio::test]
async fn summary_has_to_contain_a_usage_duration() {
    let resources: Vec<CloudResourceWithImpacts> = Vec::new();

    let usage_duration_hours = 1.5;

    let summary: ImpactsSummary = ImpactsSummary::new(
        String::from("eu-west-1"),
        String::from("IRL"),
        resources,
        usage_duration_hours,
    );

    assert_eq!(
        summary.duration_of_use_hours, usage_duration_hours,
        "Duration of summary should match"
    );
}
