use crate::countries::*;
use crate::metrics::get_metrics;
use crate::model::AwsInstanceWithImpacts;
use crate::model::ScanResultSummary;
use boavizta_api_sdk::models::UsageCloud;
#[macro_use]
extern crate log;
use pkg_version::*;
mod aws_api;
mod boavizta_api;
mod countries;
mod metrics;
mod model;
use anyhow::{Context, Result};

/// Returns a summary (summing/aggregating data where possible) of the scan results.
pub async fn build_summary(
    instances_with_impacts: &Vec<AwsInstanceWithImpacts>,
    aws_region: &str,
    duration_of_use_hours: f64,
) -> Result<ScanResultSummary> {
    let number_of_instances_total = u32::try_from(instances_with_impacts.len())?;

    let mut summary = ScanResultSummary {
        number_of_instances_total,
        aws_region: aws_region.to_owned(),
        country: countries::get_iso_country(aws_region).to_owned(),
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
    let country_code = get_iso_country(aws_region);

    let mut instances_with_impacts: Vec<AwsInstanceWithImpacts> = Vec::new();

    for instance in &instances {
        let mut usage_cloud: UsageCloud = UsageCloud::new();
        usage_cloud.usage_location = Some(String::from(country_code));
        usage_cloud.hours_use_time = Some(hours_use_time.to_owned());

        let value = boavizta_api::get_instance_impacts(instance, usage_cloud, api_url).await;
        instances_with_impacts.push(value);
    }
    Ok(instances_with_impacts)
}

/// Returns default impacts as json
pub async fn get_default_impacts(
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

// Returns default impacts as metrics
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

/// Prints impacts without using use time and default Boavizta impacts
pub async fn print_default_impacts_as_json(
    hours_use_time: &f32,
    tags: &Vec<String>,
    aws_region: &str,
    api_url: &str,
) -> Result<()> {
    let j = get_default_impacts(hours_use_time, tags, aws_region, api_url).await?;
    println!("{}", j);
    Ok(())
}

/// Prints impacts without using use time and default Boavizta impacts
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

/// Prints impacts considering the instance workload / CPU load
pub async fn print_cpu_load_impacts_as_json(
    tags: &Vec<String>,
    aws_region: &str,
    api_url: &str,
) -> Result<()> {
    warn!("Warning: getting impacts of precise CPU load is not yet implemented, will just display instances and average load of 24 hours");
    let instances = aws_api::list_instances(tags, aws_region)
        .await
        .context("Failed to list instances")?;

    for instance in &instances {
        let instance_id: &str = instance.instance_id.as_ref().unwrap();
        let cpu_load = aws_api::get_average_cpu_load_24hrs(instance_id)
            .await
            .context("Failed to get average cpu load")?;
        println!("Instance ID: {}", instance.instance_id().unwrap());
        println!("Type:       {:?}", instance.instance_type().unwrap());
        println!(
            "AZ of use:  {:?}",
            instance.placement().unwrap().availability_zone().unwrap()
        );
        println!("Tags:  {:?}", instance.tags().unwrap());
        println!("Average CPU load:  {}", cpu_load);
        println!("Not implemented: query  API at {}", api_url);
        println!();
    }
    Ok(())
}

/// List instances as text
pub async fn show_instances(tags: &Vec<String>, aws_region: &str) -> Result<()> {
    aws_api::display_instances_as_text(tags, aws_region).await?;
    Ok(())
}

/// Return current version of the cloud-scanner-cli crate
pub fn get_version() -> String {
    const MAJOR: u32 = pkg_version_major!();
    const MINOR: u32 = pkg_version_minor!();
    const PATCH: u32 = pkg_version_patch!();
    format!("{}.{}.{}", MAJOR, MINOR, PATCH)
}
