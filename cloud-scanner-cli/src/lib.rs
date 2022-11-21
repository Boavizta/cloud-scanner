//!  # cloud_scanner_cli
//!
//!  A command line application to combine Environmental impacts of your Cloud instances using Boavizta API.
//!

use crate::metric_exporter::get_metrics;
use crate::model::AwsInstanceWithImpacts;
use crate::model::ScanResultSummary;
use crate::usage_location::*;
use boavizta_api_sdk::models::UsageCloud;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;
use pkg_version::*;
pub mod aws_api;
pub mod aws_inventory;
pub mod boavizta_api;
pub mod cloud_inventory;
pub mod cloud_resource;
pub mod impact_provider;
pub mod metric_exporter;
pub mod metric_server;
pub mod model;
pub mod usage_location;

use anyhow::{Context, Result};

/// Returns a summary (summing/aggregating data where possible) of the scan results.
pub async fn build_summary(
    instances_with_impacts: &Vec<AwsInstanceWithImpacts>,
    aws_region: &str,
    duration_of_use_hours: f64,
) -> Result<ScanResultSummary> {
    let number_of_instances_total = u32::try_from(instances_with_impacts.len())?;

    let usage_location: UsageLocation = UsageLocation::from(aws_region);

    let mut summary = ScanResultSummary {
        number_of_instances_total,
        aws_region: aws_region.to_owned(),
        country: usage_location.iso_country_code,
        duration_of_use_hours,
        ..Default::default()
    };

    for instance in instances_with_impacts {
        // Only consider the instances for which we have impact data
        if let Some(impacts) = &instance.impacts {
            debug!("This instance has impacts data: {}", impacts);
            summary.number_of_instances_assessed += 1;
            summary.adp_manufacture_kgsbeq += impacts["adp"]["manufacture"].as_f64().unwrap();
            summary.adp_use_kgsbeq += impacts["adp"]["use"].as_f64().unwrap();
            summary.pe_manufacture_megajoules += impacts["pe"]["manufacture"].as_f64().unwrap();
            summary.pe_use_megajoules += impacts["pe"]["use"].as_f64().unwrap();
            summary.gwp_manufacture_kgco2eq += impacts["gwp"]["manufacture"].as_f64().unwrap();
            summary.gwp_use_kgco2eq += impacts["gwp"]["use"].as_f64().unwrap();
        } else {
            debug!("Skipped instance: {:#?} while building summary because instance has no impact data", instance);
        }
    }

    summary.number_of_instances_not_assessed =
        summary.number_of_instances_total - summary.number_of_instances_assessed;

    Ok(summary)
}

/// Standard scan (using standard/default workload)
async fn standard_scan(
    hours_use_time: &f32,
    tags: &Vec<String>,
    aws_region: &str,
    api_url: &str,
) -> Result<Vec<AwsInstanceWithImpacts>> {
    let instances = aws_api::list_instances(tags, aws_region)
        .await
        .context("Cannot perform standard scan")?;

    let usage_location = UsageLocation::from(aws_region);

    let mut instances_with_impacts: Vec<AwsInstanceWithImpacts> = Vec::new();

    for instance in &instances {
        let mut usage_cloud: UsageCloud = UsageCloud::new();
        usage_cloud.usage_location = Some(usage_location.iso_country_code.to_owned());
        usage_cloud.hours_use_time = Some(hours_use_time.to_owned());

        let value = boavizta_api::get_instance_impacts(instance, usage_cloud, api_url).await;
        instances_with_impacts.push(value);
    }
    Ok(instances_with_impacts)
}

/// CPU usage scan (using standard/default workload)
async fn get_load_based_impacts(
    _hours_use_time: &f32,
    _tags: &Vec<String>,
    _aws_region: &str,
    _api_url: &str,
) -> Result<Vec<AwsInstanceWithImpacts>> {
    unimplemented!("Getting impacts of a measured CPU load is unsupported in Boavizta API v0.1.x");
    //     let instances = aws_api::list_instances(tags, aws_region)
    //         .await
    //         .context("Cannot list the instances.")?;

    //     let usage_location = UsageLocation::from(aws_region);

    //     let mut instances_with_impacts: Vec<AwsInstanceWithImpacts> = Vec::new();

    //     for instance in &instances {
    //         let instance_id: &str = instance.instance_id.as_ref().unwrap();
    //         let duration_seconds: i32 = (*hours_use_time * 3600_f32) as i32;
    //         let _cpu_load = aws_api::get_average_cpu(instance_id, duration_seconds)
    //             .await
    //             .context(format!(
    //                 "Failed to get  average cpu load of instance {}. ",
    //                 instance_id
    //             ))?;

    //         let mut usage_cloud: UsageCloud = UsageCloud::new();
    //         usage_cloud.usage_location = Some(usage_location.iso_country_code.to_owned());
    //         usage_cloud.hours_use_time = Some(hours_use_time.to_owned());

    //         let value = boavizta_api::get_instance_impacts(instance, usage_cloud, api_url).await;
    //         instances_with_impacts.push(value);
    //     }
    //     Ok(instances_with_impacts)
}

pub async fn get_load_impacts_as_json_string(
    hours_use_time: &f32,
    tags: &Vec<String>,
    aws_region: &str,
    api_url: &str,
) -> Result<String> {
    let instances_with_impacts = get_load_based_impacts(hours_use_time, tags, aws_region, api_url)
        .await
        .context("Cannot perform load scan")?;

    let summary = build_summary(
        &instances_with_impacts,
        aws_region,
        hours_use_time.to_owned().into(),
    )
    .await;

    debug!("Summary: {:#?}", summary);

    Ok(serde_json::to_string(&instances_with_impacts)?)
}

/// Returns default impacts as json
pub async fn get_default_impacts_as_json_string(
    hours_use_time: &f32,
    tags: &Vec<String>,
    aws_region: &str,
    api_url: &str,
) -> Result<String> {
    let instances_with_impacts = standard_scan(hours_use_time, tags, aws_region, api_url)
        .await
        .context("Cannot perform standard scan")?;

    let summary = build_summary(
        &instances_with_impacts,
        aws_region,
        hours_use_time.to_owned().into(),
    )
    .await;

    debug!("Summary: {:#?}", summary);

    Ok(serde_json::to_string(&instances_with_impacts)?)
}

/// Returns default impacts as metrics
pub async fn get_default_impacts_as_metrics(
    hours_use_time: &f32,
    tags: &Vec<String>,
    aws_region: &str,
    api_url: &str,
) -> Result<String> {
    let instances_with_impacts = standard_scan(hours_use_time, tags, aws_region, api_url)
        .await
        .context("Cannot perform standard scan")?;

    let summary = build_summary(
        &instances_with_impacts,
        aws_region,
        hours_use_time.to_owned().into(),
    )
    .await?;

    debug!("Summary: {:#?}", summary);

    let metrics = get_metrics(&summary).with_context(|| {
        format!(
            "Unable to get default impacts as metrics for {}",
            aws_region
        )
    })?;

    Ok(metrics)
}

/// Prints default impacts  to standard output in json format
pub async fn print_default_impacts_as_json(
    hours_use_time: &f32,
    tags: &Vec<String>,
    aws_region: &str,
    api_url: &str,
) -> Result<()> {
    let j = get_default_impacts_as_json_string(hours_use_time, tags, aws_region, api_url).await?;
    println!("{}", j);
    Ok(())
}

/// Prints default impacts  to standard output as metrics in prometheus format
pub async fn print_default_impacts_as_metrics(
    hours_use_time: &f32,
    tags: &Vec<String>,
    aws_region: &str,
    api_url: &str,
) -> Result<()> {
    let metrics = get_default_impacts_as_metrics(hours_use_time, tags, aws_region, api_url).await?;
    println!("{}", metrics);
    Ok(())
}

/// Prints impacts without using use time and default Boavizta impacts
pub async fn print_cpu_load_impacts_as_metrics(
    _hours_use_time: &f32,
    _tags: &Vec<String>,
    _aws_region: &str,
    _api_url: &str,
) -> Result<()> {
    unimplemented!("Getting impacts of a measured CPU load is unsupported in Boavizta API v0.1.x");
}

/// Prints impacts considering the instance workload / CPU load
pub async fn print_cpu_load_impacts_as_json(
    _hours_use_time: &f32,
    _tags: &Vec<String>,
    _aws_region: &str,
    _api_url: &str,
) -> Result<()> {
    unimplemented!("Getting impacts of a measured CPU load is unsupported in Boavizta API v0.1.x");
}

/// List instances and metadata to standard output
pub async fn show_instances(tags: &Vec<String>, aws_region: &str) -> Result<()> {
    aws_api::display_instances_as_text(tags, aws_region).await?;
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
