//!  # cloud_scanner_cli
//!
//!  A module that returns an estimation of environmental impact of the resources used in a cloud account.
//!
//! It performs inventory of resources of the account and combines it with Boavizta API to return impact data.
//!

use crate::model::{EstimatedInventory, ExecutionStatistics};
use crate::usage_location::*;
use aws_cloud_provider::*;
use boavizta_api_v1::*;
use cloud_provider::*;
use impact_provider::ImpactProvider;
use impact_provider::ImpactsSummary;
use metric_exporter::*;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;
use model::Inventory;
use pkg_version::*;
use std::time::{Duration, Instant};
pub mod aws_cloud_provider;
pub mod boavizta_api_v1;
pub mod cloud_provider;
pub mod impact_provider;
pub mod metric_exporter;
pub mod model;
pub mod standalone_server;
pub mod usage_location;

use anyhow::{Context, Result};

async fn estimate_impacts(
    use_duration_hours: &f32,
    tags: &[String],
    aws_region: &str,
    api_url: &str,
    verbose: bool,
    include_block_storage: bool,
) -> Result<EstimatedInventory> {
    let aws_provider: AwsCloudProvider = AwsCloudProvider::new(aws_region).await;
    let inventory: Inventory = aws_provider
        .list_resources(tags, include_block_storage)
        .await
        .context("Cannot perform resources inventory")?;

    let api: BoaviztaApiV1 = BoaviztaApiV1::new(api_url);
    let estimated_inventory = api
        .get_impacts(inventory, use_duration_hours, verbose)
        .await
        .context("Failure while retrieving impacts")?;

    Ok(estimated_inventory)
}

/// Returns default impacts as json string
pub async fn get_impacts_as_json_string(
    use_duration_hours: &f32,
    tags: &[String],
    aws_region: &str,
    api_url: &str,
    verbose: bool,
    include_block_storage: bool,
    summary_only: bool,
) -> Result<String> {
    let inventory_with_impacts = estimate_impacts(
        use_duration_hours,
        tags,
        aws_region,
        api_url,
        verbose,
        include_block_storage,
    )
    .await
    .context("Cannot perform standard scan")?;

    if summary_only {
        let usage_location: UsageLocation = UsageLocation::try_from(aws_region)?;
        let summary: ImpactsSummary = ImpactsSummary::new(
            String::from(aws_region),
            usage_location.iso_country_code,
            inventory_with_impacts.clone(),
            (*use_duration_hours).into(),
        );

        return Ok(serde_json::to_string(&summary)?);
    }

    Ok(serde_json::to_string(&inventory_with_impacts)?)
}

/// Returns  impacts as metrics
pub async fn get_impacts_as_metrics(
    use_duration_hours: &f32,
    tags: &[String],
    aws_region: &str,
    api_url: &str,
    include_storage: bool,
) -> Result<String> {
    let resources_with_impacts = estimate_impacts(
        use_duration_hours,
        tags,
        aws_region,
        api_url,
        false,
        include_storage,
    )
    .await
    .context("Cannot perform standard scan")?;

    let usage_location: UsageLocation = UsageLocation::try_from(aws_region)?;
    let summary: ImpactsSummary = ImpactsSummary::new(
        String::from(aws_region),
        usage_location.iso_country_code,
        resources_with_impacts.clone(),
        (*use_duration_hours).into(),
    );
    debug!("Summary: {:#?}", summary);

    let all_metrics = get_all_metrics(&summary, resources_with_impacts).with_context(|| {
        format!(
            "Unable to get resource impacts as metrics for region {}",
            aws_region
        )
    })?;

    Ok(all_metrics)
}

/// Prints  impacts to standard output in json format
pub async fn print_default_impacts_as_json(
    use_duration_hours: &f32,
    tags: &[String],
    aws_region: &str,
    api_url: &str,
    verbose: bool,
    include_storage: bool,
    summary_only: bool,
) -> Result<()> {
    let j = get_impacts_as_json_string(
        use_duration_hours,
        tags,
        aws_region,
        api_url,
        verbose,
        include_storage,
        summary_only,
    )
    .await?;
    println!("{}", j);
    Ok(())
}

/// Prints impacts to standard output as metrics in prometheus format
pub async fn print_default_impacts_as_metrics(
    use_duration_hours: &f32,
    tags: &[String],
    aws_region: &str,
    api_url: &str,
    include_block_storage: bool,
) -> Result<()> {
    let metrics = get_impacts_as_metrics(
        use_duration_hours,
        tags,
        aws_region,
        api_url,
        include_block_storage,
    )
    .await?;
    println!("{}", metrics);
    Ok(())
}

/// Returns the inventory of cloud resources a as json String
pub async fn get_inventory_as_json(
    tags: &[String],
    aws_region: &str,
    include_block_storage: bool,
) -> Result<String> {
    let start = Instant::now();
    let aws_inventory: AwsCloudProvider = AwsCloudProvider::new(aws_region).await;
    let inventory: Inventory = aws_inventory
        .list_resources(tags, include_block_storage)
        .await
        .context("Cannot perform inventory.")?;
    let stats = ExecutionStatistics {
        inventory_duration: start.elapsed(),
        impact_estimation_duration: Duration::from_millis(0),
        total_duration: start.elapsed(),
    };
    warn!("{:?}", stats);
    serde_json::to_string(&inventory.resources).context("Cannot format inventory as json")
}

/// Returns the inventory of cloud resources
pub async fn get_inventory(
    tags: &[String],
    aws_region: &str,
    include_block_storage: bool,
) -> Result<Inventory> {
    let aws_inventory: AwsCloudProvider = AwsCloudProvider::new(aws_region).await;
    let inventory: Inventory = aws_inventory
        .list_resources(tags, include_block_storage)
        .await
        .context("Cannot perform inventory.")?;
    Ok(inventory)
}

/// List instances and metadata to standard output
pub async fn show_inventory(
    tags: &[String],
    aws_region: &str,
    include_block_storage: bool,
) -> Result<()> {
    let json_inventory: String =
        get_inventory_as_json(tags, aws_region, include_block_storage).await?;
    println!("{}", json_inventory);
    Ok(())
}

/// Starts a server that exposes metrics http like <http://localhost:8000/metrics?aws-region=eu-west-1>
pub async fn serve_metrics(api_url: &str) -> Result<()> {
    let config = standalone_server::Config {
        boavizta_url: api_url.to_string(),
    };
    warn!("Starting server.");
    standalone_server::run(config).await?;
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
    use crate::impact_provider::CloudResourceWithImpacts;

    let resources: Vec<CloudResourceWithImpacts> = Vec::new();

    let resources_with_impacts: EstimatedInventory = EstimatedInventory {
        impacting_resources: resources,
        execution_statistics: None,
    };

    let usage_duration_hours = 1.5;

    let summary: ImpactsSummary = ImpactsSummary::new(
        String::from("eu-west-1"),
        String::from("IRL"),
        resources_with_impacts,
        usage_duration_hours,
    );

    assert_eq!(
        summary.duration_of_use_hours, usage_duration_hours,
        "Duration of summary should match"
    );
}
