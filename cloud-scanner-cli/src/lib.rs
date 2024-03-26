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
use prometheus_http_query::Client;

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
            &inventory_with_impacts.clone(),
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
        &resources_with_impacts,
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

/// Returns deployment impacts as metrics
pub async fn get_deployment_impacts_as_metrics(
    use_duration_hours: &f32,
    tags: &[String],
    aws_region: &str,
    api_url: &str,
    prometheus_input_url: &str,
    include_storage: bool,
    namespace_to_scan:&str,
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

    let client: Client = Client::try_from(prometheus_input_url).unwrap();

    let mut cpu_pod_consumming_by_node =
        "sum by (node, pod)
        (kube_pod_container_resource_requests{resource=\"cpu\",node=~\".*\",namespace=~\"".to_owned();

    if namespace_to_scan.is_empty() {
        warn!("Warning: namespace_to_scan is empty.");
        cpu_pod_consumming_by_node.push_str(".*\"})");
    } else {
        cpu_pod_consumming_by_node.push_str(namespace_to_scan);
        cpu_pod_consumming_by_node.push_str("\"})");
    }

    let response = client.query(cpu_pod_consumming_by_node).get().await?;
    let mut impacts_copy = resources_with_impacts.impacting_resources.clone();

    let mut owned_string: String = "".to_owned();
    let mut boavizta_resource_pe_embodied_megajoules = "# HELP boavizta_resource_pe_embodied_megajoules Energy consumed for manufacture.\n# TYPE boavizta_resource_pe_embodied_megajoules gauge\n".to_owned();
    let mut boavizta_resource_pe_use_megajoules = "# HELP boavizta_pe_use_megajoules Energy consumed during use.\n# TYPE boavizta_pe_use_megajoules gauge\n".to_owned();
    let mut boavizta_resource_adp_embodied_kgsbeq = "# HELP boavizta_resource_adp_embodied_kgsbeq Abiotic resources depletion potential of embodied impacts.\n# TYPE boavizta_resource_adp_embodied_kgsbeq gauge\n".to_owned();
    let mut boavizta_resource_adp_use_kgsbeq = "# HELP boavizta_resource_adp_use_kgsbeq Abiotic resources depletion potential of use.\n# TYPE boavizta_resource_adp_use_kgsbeq gauge\n".to_owned();
    let mut boavizta_resource_gwp_embodied_kgco2eq = "# HELP boavizta_resource_gwp_embodied_kgco2eq Global Warming Potential of embodied impacts.\n# TYPE boavizta_resource_gwp_embodied_kgco2eq gauge\n".to_owned();
    let mut boavizta_resource_gwp_use_kgco2eq = "# HELP boavizta_resource_gwp_use_kgco2eq Global Warming Potential of use.\n# TYPE boavizta_resource_gwp_use_kgco2eq gauge\n".to_owned();

    if response.data().is_empty() {
        warn!("Warning: No data found in the response of cpu_pod_consumming_by_node.");
    } else {
        warn!("&response.data().as_vector(): {:#?}", &response.data().as_vector());
    }

    for vec_instant_vector in &response.data().as_vector() {
        for instant_vector in vec_instant_vector.into_iter() {
            match instant_vector.metric().get("node") {
                Some(private_dns_name) => {
                    for impact in &mut impacts_copy {
                        match &impact.cloud_resource.resource_details {
                            model::ResourceDetails::Instance {
                                instance_type,
                                private_ip_dns_name,
                                usage,
                            } => {
                                if private_dns_name.eq(private_ip_dns_name) {
                                    // (GWP use par pod) = (GWP use par node) x (usage cpu pod par node) / usage cpu par node
                                    if let Some(impacts) = &impact.impacts_values {
                                        let usage_temp = usage.as_ref().unwrap();

                                        let pe_use_par_pod =
                                        &impacts.pe_use_megajoules // GWP use par node
                                        * &instant_vector.sample().value() // usage cpu pod par node
                                        / &usage_temp.average_cpu_load; // usage cpu par node

                                        let pe_manufacture_par_pod =
                                        &impacts.pe_manufacture_megajoules // GWP use par node
                                        * &instant_vector.sample().value() // usage cpu pod par node
                                        / &usage_temp.average_cpu_load; // usage cpu par node

                                        let adp_use_par_pod =
                                        &impacts.adp_use_kgsbeq // GWP use par node
                                        * &instant_vector.sample().value() // usage cpu pod par node
                                        / &usage_temp.average_cpu_load; // usage cpu par node

                                        let adp_manufacture_par_pod =
                                        &impacts.adp_manufacture_kgsbeq // GWP use par node
                                        * &instant_vector.sample().value() // usage cpu pod par node
                                        / &usage_temp.average_cpu_load; // usage cpu par node

                                        let gwp_use_par_pod =
                                            &impacts.adp_use_kgsbeq // GWP use par node
                                            * &instant_vector.sample().value() // usage cpu pod par node
                                            / &usage_temp.average_cpu_load; // usage cpu par node

                                        let gwp_manufacture_par_pod =
                                        &impacts.gwp_manufacture_kgco2eq // GWP use par node
                                        * &instant_vector.sample().value() // usage cpu pod par node
                                        / &usage_temp.average_cpu_load; // usage cpu par node

                                        impact.impacts_values = Some(impact_provider::ImpactsValues {
                                            adp_manufacture_kgsbeq: adp_manufacture_par_pod,
                                            adp_use_kgsbeq: adp_use_par_pod,
                                            pe_manufacture_megajoules: pe_manufacture_par_pod,
                                            pe_use_megajoules: pe_use_par_pod,
                                            gwp_manufacture_kgco2eq: gwp_manufacture_par_pod,
                                            gwp_use_kgco2eq: gwp_use_par_pod,
                                            raw_data: impacts.raw_data.clone(),
                                        });

                                        if let Some(pod_id) = instant_vector.metric().get("pod") {
                                            impact.cloud_resource.tags = <Vec<model::CloudResourceTag>>::new();
                                            impact.cloud_resource.id = pod_id.to_string();


                                            let mut boavizta_resource_pe_embodied_megajoules_temp =
                                            "boavizta_resource_pe_embodied_megajoules{awsregion=\"eu-west-1\",country=\"IRL\",resource_type=\"Pod\",resource_id=\"".to_owned();
                                            boavizta_resource_pe_embodied_megajoules_temp.push_str(pod_id);
                                            boavizta_resource_pe_embodied_megajoules_temp.push_str("\",resource_tags=\"");
                                            boavizta_resource_pe_embodied_megajoules_temp.push_str("private_ip_dns_name:");
                                            boavizta_resource_pe_embodied_megajoules_temp.push_str(private_ip_dns_name);
                                            boavizta_resource_pe_embodied_megajoules_temp.push_str(";");
                                            boavizta_resource_pe_embodied_megajoules_temp.push_str("\",resource_state=\"Running\"} ");
                                            boavizta_resource_pe_embodied_megajoules_temp.push_str(&pe_manufacture_par_pod.to_string());
                                            boavizta_resource_pe_embodied_megajoules_temp.push_str("\n");
                                            boavizta_resource_pe_embodied_megajoules.push_str(&boavizta_resource_pe_embodied_megajoules_temp);

                                            let mut boavizta_resource_pe_use_megajoules_temp =
                                            "boavizta_resource_pe_use_megajoules{awsregion=\"eu-west-1\",country=\"IRL\",resource_type=\"Pod\",resource_id=\"".to_owned();
                                            boavizta_resource_pe_use_megajoules_temp.push_str(pod_id);
                                            boavizta_resource_pe_use_megajoules_temp.push_str("\",resource_tags=\"");
                                            boavizta_resource_pe_use_megajoules_temp.push_str("private_ip_dns_name:");
                                            boavizta_resource_pe_use_megajoules_temp.push_str(private_ip_dns_name);
                                            boavizta_resource_pe_use_megajoules_temp.push_str(";");
                                            boavizta_resource_pe_use_megajoules_temp.push_str("\",resource_state=\"Running\"} ");
                                            boavizta_resource_pe_use_megajoules_temp.push_str(&pe_use_par_pod.to_string());
                                            boavizta_resource_pe_use_megajoules_temp.push_str("\n");
                                            boavizta_resource_pe_use_megajoules.push_str(&boavizta_resource_pe_use_megajoules_temp);

                                            let mut boavizta_resource_adp_embodied_kgsbeq_temp =
                                            "boavizta_resource_adp_embodied_kgsbeq{awsregion=\"eu-west-1\",country=\"IRL\",resource_type=\"Pod\",resource_id=\"".to_owned();
                                            boavizta_resource_adp_embodied_kgsbeq_temp.push_str(pod_id);
                                            boavizta_resource_adp_embodied_kgsbeq_temp.push_str("\",resource_tags=\"");
                                            boavizta_resource_adp_embodied_kgsbeq_temp.push_str("private_ip_dns_name:");
                                            boavizta_resource_adp_embodied_kgsbeq_temp.push_str(private_ip_dns_name);
                                            boavizta_resource_adp_embodied_kgsbeq_temp.push_str(";");
                                            boavizta_resource_adp_embodied_kgsbeq_temp.push_str("\",resource_state=\"Running\"} ");
                                            boavizta_resource_adp_embodied_kgsbeq_temp.push_str(&adp_manufacture_par_pod.to_string());
                                            boavizta_resource_adp_embodied_kgsbeq_temp.push_str("\n");
                                            boavizta_resource_adp_embodied_kgsbeq.push_str(&boavizta_resource_adp_embodied_kgsbeq_temp);

                                            let mut boavizta_resource_adp_use_kgsbeq_temp =
                                            "boavizta_resource_adp_use_kgsbeq{awsregion=\"eu-west-1\",country=\"IRL\",resource_type=\"Pod\",resource_id=\"".to_owned();
                                            boavizta_resource_adp_use_kgsbeq_temp.push_str(pod_id);
                                            boavizta_resource_adp_use_kgsbeq_temp.push_str("\",resource_tags=\"");
                                            boavizta_resource_adp_use_kgsbeq_temp.push_str("private_ip_dns_name:");
                                            boavizta_resource_adp_use_kgsbeq_temp.push_str(private_ip_dns_name);
                                            boavizta_resource_adp_use_kgsbeq_temp.push_str(";");
                                            boavizta_resource_adp_use_kgsbeq_temp.push_str("\",resource_state=\"Running\"} ");
                                            boavizta_resource_adp_use_kgsbeq_temp.push_str(&adp_use_par_pod.to_string());
                                            boavizta_resource_adp_use_kgsbeq_temp.push_str("\n");
                                            boavizta_resource_adp_use_kgsbeq.push_str(&boavizta_resource_adp_use_kgsbeq_temp);

                                            let mut boavizta_resource_gwp_embodied_kgco2eq_temp =
                                            "boavizta_resource_gwp_embodied_kgco2eq{awsregion=\"eu-west-1\",country=\"IRL\",resource_type=\"Pod\",resource_id=\"".to_owned();
                                            boavizta_resource_gwp_embodied_kgco2eq_temp.push_str(pod_id);
                                            boavizta_resource_gwp_embodied_kgco2eq_temp.push_str("\",resource_tags=\"");
                                            boavizta_resource_gwp_embodied_kgco2eq_temp.push_str("private_ip_dns_name:");
                                            boavizta_resource_gwp_embodied_kgco2eq_temp.push_str(private_ip_dns_name);
                                            boavizta_resource_gwp_embodied_kgco2eq_temp.push_str(";");
                                            boavizta_resource_gwp_embodied_kgco2eq_temp.push_str("\",resource_state=\"Running\"} ");
                                            boavizta_resource_gwp_embodied_kgco2eq_temp.push_str(&gwp_manufacture_par_pod.to_string());
                                            boavizta_resource_gwp_embodied_kgco2eq_temp.push_str("\n");
                                            boavizta_resource_gwp_embodied_kgco2eq.push_str(&boavizta_resource_gwp_embodied_kgco2eq_temp);

                                            let mut boavizta_resource_gwp_use_kgco2eq_temp =
                                            "boavizta_resource_gwp_use_kgco2eq{awsregion=\"eu-west-1\",country=\"IRL\",resource_type=\"Pod\",resource_id=\"".to_owned();
                                            boavizta_resource_gwp_use_kgco2eq_temp.push_str(pod_id);
                                            boavizta_resource_gwp_use_kgco2eq_temp.push_str("\",resource_tags=\"");
                                            boavizta_resource_gwp_use_kgco2eq_temp.push_str("private_ip_dns_name:");
                                            boavizta_resource_gwp_use_kgco2eq_temp.push_str(private_ip_dns_name);
                                            boavizta_resource_gwp_use_kgco2eq_temp.push_str(";");
                                            boavizta_resource_gwp_use_kgco2eq_temp.push_str("\",resource_state=\"Running\"} ");
                                            boavizta_resource_gwp_use_kgco2eq_temp.push_str(&gwp_use_par_pod.to_string());
                                            boavizta_resource_gwp_use_kgco2eq_temp.push_str("\n");
                                            boavizta_resource_gwp_use_kgco2eq.push_str(&boavizta_resource_gwp_use_kgco2eq_temp);

                                        }
                                    };

                                }
                            }

                            model::ResourceDetails::BlockStorage {
                                storage_type,
                                usage,
                                attached_instances: _,
                            } => {
                                warn!("Warning: This type of cloud resource is not supported.");
                            }

                            _ => {
                                warn!("Warning: This type of cloud resource is not supported.");
                            }
                        }
                    }
                }
                _ => warn!("no match private_ip_dns_name: {:#?}", "private_ip_dns_name"),
            }
        }
    }

   owned_string.push_str(&boavizta_resource_pe_embodied_megajoules);
   owned_string.push_str(&boavizta_resource_pe_use_megajoules);
   owned_string.push_str(&boavizta_resource_adp_embodied_kgsbeq);
   owned_string.push_str(&boavizta_resource_adp_use_kgsbeq);
   owned_string.push_str(&boavizta_resource_gwp_embodied_kgco2eq);
   owned_string.push_str(&boavizta_resource_gwp_use_kgco2eq);
   owned_string.push_str("# EOF\n");

   Ok(owned_string)
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
pub async fn serve_metrics(api_url: &str, prometheus_input_url:&str, namespace_to_scan:&str) -> Result<()> {
    let config = standalone_server::Config {
        boavizta_url: api_url.to_string(),
        prometheus_input_url: prometheus_input_url.to_string(),
        namespace_to_scan: namespace_to_scan.to_string(),
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
        &resources_with_impacts,
        usage_duration_hours,
    );

    assert_eq!(
        summary.duration_of_use_hours, usage_duration_hours,
        "Duration of summary should match"
    );
}
