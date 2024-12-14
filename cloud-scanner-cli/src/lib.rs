//!  # cloud_scanner_cli
//!
//!  A module that returns an estimation of environmental impact of the resources used in a cloud account.
//!
//! It performs inventory of resources of the account and combines it with Boavizta API to return impact data.
//!

use crate::model::{load_inventory_from_file, EstimatedInventory};
use aws_cloud_provider::*;
use boavizta_api_v1::*;
use cloud_provider::*;
use impact_provider::ImpactProvider;
use impact_provider::ImpactsSummary;
use metric_exporter::*;
use std::path::Path;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;
use model::Inventory;
use pkg_version::*;

pub mod aws_cloud_provider;
pub mod boavizta_api_v1;
pub mod cloud_provider;
pub mod estimated_inventory_exporter;
pub mod impact_provider;
pub mod inventory_exporter;
pub mod metric_exporter;
pub mod model;
pub mod standalone_server;
pub mod usage_location;

use crate::estimated_inventory_exporter::build_impact_summary;
use anyhow::{Context, Result};

/// Returns estimated impacts as the result of a live inventory.
pub async fn estimate_impacts(
    use_duration_hours: &f32,
    tags: &[String],
    aws_region: &str,
    api_url: &str,
    verbose: bool,
    include_block_storage: bool,
) -> Result<EstimatedInventory> {
    let aws_provider: AwsCloudProvider = AwsCloudProvider::new(aws_region).await?;
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

/// Returns impacts for and existing inventory file
pub async fn estimate_impacts_of_inventory_file(
    use_duration_hours: &f32,
    api_url: &str,
    verbose: bool,
    inventory_file: &Path,
) -> Result<EstimatedInventory> {
    let inventory = load_inventory_from_file(inventory_file)
        .await
        .context("Failed to load inventory file.")?;
    let estimated_inventory =
        estimate_impacts_of_inventory(use_duration_hours, api_url, verbose, inventory)
            .await
            .context("Cannot get estimations")?;
    Ok(estimated_inventory)
}

/// Returns impacts for and existing inventory
pub async fn estimate_impacts_of_inventory(
    use_duration_hours: &f32,
    api_url: &str,
    verbose: bool,
    inventory: Inventory,
) -> Result<EstimatedInventory> {
    let api: BoaviztaApiV1 = BoaviztaApiV1::new(api_url);
    let estimated_inventory = api
        .get_impacts(inventory, use_duration_hours, verbose)
        .await
        .context("Failure while retrieving impacts")?;

    Ok(estimated_inventory)
}

/// Returns impacts of a live inventory as metrics
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

    let summary: ImpactsSummary =
        build_impact_summary(&resources_with_impacts, aws_region, use_duration_hours).await?;
    debug!("Summary: {:#?}", summary);

    let all_metrics = get_all_metrics(&summary, resources_with_impacts).with_context(|| {
        format!(
            "Unable to get resource impacts as metrics for region {}",
            aws_region
        )
    })?;

    Ok(all_metrics)
}

/// Returns a live inventory of cloud resources
pub async fn get_inventory(
    tags: &[String],
    aws_region: &str,
    include_block_storage: bool,
) -> Result<Inventory> {
    let aws_inventory: AwsCloudProvider = AwsCloudProvider::new(aws_region).await?;
    let inventory: Inventory = aws_inventory
        .list_resources(tags, include_block_storage)
        .await
        .context("Cannot perform inventory.")?;
    Ok(inventory)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::EstimationMetadata;

    #[tokio::test]
    async fn summary_has_to_contain_a_usage_duration() {
        use crate::impact_provider::CloudResourceWithImpacts;

        let resources: Vec<CloudResourceWithImpacts> = Vec::new();

        let resources_with_impacts: EstimatedInventory = EstimatedInventory {
            impacting_resources: resources,
            metadata: EstimationMetadata {
                description: None,
                boavizta_api_version: Some("v1.2.3".to_owned()),
                cloud_scanner_version: Some("acb".to_owned()),
                estimation_date: None,
                execution_statistics: None,
            },
        };

        let usage_duration_hours = 1.5;

        let summary: ImpactsSummary = ImpactsSummary::new(
            String::from("eu-west-1"),
            String::from("IRL"),
            &resources_with_impacts,
            usage_duration_hours,
        );

        assert_eq!(
            summary.duration_of_use_hours, usage_duration_hours,
            "Duration of summary should match"
        );
    }
}
