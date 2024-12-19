//!  A module to format the results of cloud-scanner into OpenMetrics (Prometheus format) metrics
use anyhow::{Context, Result};
use std::sync::atomic::AtomicU64;

use crate::impact_provider::CloudResourceWithImpacts;
use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::{EncodeLabelSet, EncodeLabelValue};
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::*;
use prometheus_client::registry::Registry;

use crate::model::{EstimatedInventory, InstanceState, ResourceDetails};
use crate::ImpactsSummary;

// Define a type representing a metric label set, i.e. a key value pair.
#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet, Debug)]
pub struct SummaryLabels {
    pub awsregion: String,
    pub country: String,
    pub cloud_scanner_version: String,
    pub boaviztapi_version: String,
}
#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet, Debug)]
pub struct ResourceLabels {
    pub awsregion: String,
    pub country: String,
    pub resource_type: ResourceType,
    pub resource_id: String,
    pub resource_tags: String,
    pub resource_state: ResourceState,
    pub cloud_scanner_version: String,
    pub boaviztapi_version: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
pub enum ResourceType {
    BlockStorage,
    Instance,
    ObjectStorage,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
pub enum ResourceState {
    Running,
    Stopped,
    Unknown,
}

fn build_resource_labels(
    resource: &CloudResourceWithImpacts,
    cloud_scanner_version: &str,
    boaviztapi_version: &str,
) -> ResourceLabels {
    let resource_type = match resource.clone().cloud_resource.resource_details {
        ResourceDetails::Instance { .. } => ResourceType::Instance,
        ResourceDetails::BlockStorage { .. } => ResourceType::BlockStorage,
        ResourceDetails::ObjectStorage => ResourceType::ObjectStorage,
    };
    let resource_state = match resource.clone().cloud_resource.resource_details {
        ResourceDetails::Instance {
            instance_type: _,
            usage,
        } => match usage.unwrap().state {
            InstanceState::Running => ResourceState::Running,
            InstanceState::Stopped => ResourceState::Stopped,
        },
        _ => ResourceState::Unknown,
    };

    ResourceLabels {
        awsregion: resource.cloud_resource.location.aws_region.clone(),
        country: resource.cloud_resource.location.iso_country_code.clone(),
        resource_type,
        resource_id: resource.cloud_resource.id.clone(),
        resource_tags: resource.cloud_resource.tags_as_metric_label_value(),
        resource_state,
        cloud_scanner_version: cloud_scanner_version.to_owned(),
        boaviztapi_version: boaviztapi_version.to_owned(),
    }
}

pub fn register_resource_metrics(
    registry: &mut Registry,
    resources_with_impacts: Vec<CloudResourceWithImpacts>,
    cloud_scanner_version: &str,
    boaviztapi_version: &str,
) {
    // Register metrics
    let boavizta_resource_duration_of_use_hours =
        Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "boavizta_resource_duration_of_use_hours",
        "Use duration considered to estimate impacts",
        boavizta_resource_duration_of_use_hours.clone(),
    );
    let boavizta_resource_pe_embodied_megajoules =
        Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "boavizta_resource_pe_embodied_megajoules",
        "Energy consumed for manufacture",
        boavizta_resource_pe_embodied_megajoules.clone(),
    );
    let boavizta_resource_pe_use_megajoules =
        Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "boavizta_resource_pe_use_megajoules",
        "Energy consumed during use",
        boavizta_resource_pe_use_megajoules.clone(),
    );
    let boavizta_resource_adp_embodied_kgsbeq =
        Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "boavizta_resource_adp_embodied_kgsbeq",
        "Abiotic resources depletion potential of embodied impacts",
        boavizta_resource_adp_embodied_kgsbeq.clone(),
    );
    let boavizta_resource_adp_use_kgsbeq =
        Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "boavizta_resource_adp_use_kgsbeq",
        "Abiotic resources depletion potential of use",
        boavizta_resource_adp_use_kgsbeq.clone(),
    );
    let boavizta_resource_gwp_embodied_kgco2eq =
        Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "boavizta_resource_gwp_embodied_kgco2eq",
        "Global Warming Potential of embodied impacts",
        boavizta_resource_gwp_embodied_kgco2eq.clone(),
    );
    let boavizta_resource_gwp_use_kgco2eq =
        Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "boavizta_resource_gwp_use_kgco2eq",
        "Global Warming Potential of use",
        boavizta_resource_gwp_use_kgco2eq.clone(),
    );

    let boavizta_resource_cpu_load = Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "boavizta_resource_cpu_load",
        "CPU load of instance",
        boavizta_resource_cpu_load.clone(),
    );

    let boavizta_storage_size_gb = Family::<ResourceLabels, Gauge>::default();
    registry.register(
        "boavizta_storage_size_gb",
        "Storage size in GB",
        boavizta_storage_size_gb.clone(),
    );

    // Fill up metrics values
    for resource in resources_with_impacts.iter() {
        let resource_labels =
            build_resource_labels(resource, cloud_scanner_version, boaviztapi_version);
        let impacts = resource.impacts_values.as_ref();

        // the impacts can be missing
        if impacts.is_some() {
            let impact_values = impacts.unwrap();
            boavizta_resource_duration_of_use_hours
                .get_or_create(&resource_labels)
                .set(resource.impacts_duration_hours.into());
            boavizta_resource_pe_use_megajoules
                .get_or_create(&resource_labels)
                .set(impact_values.pe_use_megajoules);
            boavizta_resource_pe_embodied_megajoules
                .get_or_create(&resource_labels)
                .set(impact_values.pe_manufacture_megajoules);
            boavizta_resource_adp_use_kgsbeq
                .get_or_create(&resource_labels)
                .set(impact_values.adp_use_kgsbeq);
            boavizta_resource_adp_embodied_kgsbeq
                .get_or_create(&resource_labels)
                .set(impact_values.adp_manufacture_kgsbeq);
            boavizta_resource_gwp_use_kgco2eq
                .get_or_create(&resource_labels)
                .set(impact_values.gwp_use_kgco2eq);
            boavizta_resource_gwp_embodied_kgco2eq
                .get_or_create(&resource_labels)
                .set(impact_values.gwp_manufacture_kgco2eq);
        }

        // Export CPU usage metrics (for instances) and size metrics (for storage)
        match &resource.cloud_resource.resource_details {
            ResourceDetails::Instance {
                usage: Some(instance_usage),
                ..
            } => {
                let cpu_load = instance_usage.average_cpu_load;
                boavizta_resource_cpu_load
                    .get_or_create(&resource_labels)
                    .set(cpu_load);
            }
            ResourceDetails::BlockStorage {
                usage: Some(storage_usage),
                ..
            } => {
                let size_gb = storage_usage.size_gb;
                boavizta_storage_size_gb
                    .get_or_create(&resource_labels)
                    .set(size_gb as i64);
            }
            _ => {}
        }
    }
}
/// Returns metrics related to individual resources as String
///
/// - Individual resource metrics are prefixed with: `boavizta_resource_`
pub fn get_resources_metrics(
    resources_with_impacts: Vec<CloudResourceWithImpacts>,
    cloud_scanner_version: &str,
    boaviztapi_version: &str,
) -> Result<String> {
    let mut registry = <Registry>::default();
    register_resource_metrics(
        &mut registry,
        resources_with_impacts,
        cloud_scanner_version,
        boaviztapi_version,
    );
    let mut buffer = String::new();
    encode(&mut buffer, &registry).context("Fails to encode resources impacts into metrics")?;
    let metrics = buffer;

    Ok(metrics)
}

/// Return an ImpactsSummary as metrics in the prometheus format
///
/// - Summary metrics are prefixed with: `boavizta_`
pub fn get_summary_metrics(
    summary: &ImpactsSummary,
    cloud_scanner_version: String,
    boaviztapi_version: String,
) -> Result<String> {
    let mut registry = <Registry>::default();
    register_summary_metrics(
        &mut registry,
        summary,
        cloud_scanner_version,
        boaviztapi_version,
    );
    let mut buffer = String::new();
    encode(&mut buffer, &registry).context("Fails to encode impacts summary into metrics")?;
    let metrics = buffer;
    Ok(metrics)
}

/// Returns all metrics as string: both aggregated metrics (summary) as well a metrics of individual resources
///
/// - Summary metrics are prefixed with: `boavizta_`
/// - Individual resource metrics are prefixed with: `boavizta_resource_`
pub fn get_all_metrics(
    summary: &ImpactsSummary,
    estimated_inventory: EstimatedInventory,
) -> Result<String> {
    let cloud_scanner_version = estimated_inventory
        .metadata
        .cloud_scanner_version
        .unwrap_or("".to_string());
    let boaviztapi_version = estimated_inventory
        .metadata
        .boavizta_api_version
        .unwrap_or("".to_string());

    let mut registry = <Registry>::default();
    register_summary_metrics(
        &mut registry,
        summary,
        cloud_scanner_version.clone(),
        boaviztapi_version.clone(),
    );
    register_resource_metrics(
        &mut registry,
        estimated_inventory.impacting_resources,
        &cloud_scanner_version,
        &boaviztapi_version,
    );

    let mut buffer = String::new();
    encode(&mut buffer, &registry).context("Fails to encode impacts into metrics")?;
    let metrics = buffer;

    Ok(metrics)
}

fn register_summary_metrics(
    registry: &mut Registry,
    summary: &ImpactsSummary,
    cloud_scanner_version: String,
    boaviztapi_version: String,
) {
    let boavizta_number_of_resources_total = Family::<SummaryLabels, Gauge>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_number_of_resources_total",
        // And the metric help text.
        "Number of resources detected during the inventory",
        boavizta_number_of_resources_total.clone(),
    );

    let boavizta_number_of_resources_assessed = Family::<SummaryLabels, Gauge>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_number_of_resources_assessed",
        // And the metric help text.
        "Number of resources that were considered in the estimation of impacts",
        boavizta_number_of_resources_assessed.clone(),
    );

    let boavizta_duration_of_use_hours = Family::<SummaryLabels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_duration_of_use_hours",
        // And the metric help text.
        "Use duration considered to estimate impacts",
        boavizta_duration_of_use_hours.clone(),
    );

    let boavizta_pe_manufacture_megajoules =
        Family::<SummaryLabels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_pe_manufacture_megajoules",
        // And the metric help text.
        "Energy consumed for manufacture",
        boavizta_pe_manufacture_megajoules.clone(),
    );

    let boavizta_pe_use_megajoules = Family::<SummaryLabels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_pe_use_megajoules",
        // And the metric help text.
        "Energy consumed during use",
        boavizta_pe_use_megajoules.clone(),
    );

    let boavizta_adp_manufacture_kgsbeq = Family::<SummaryLabels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_adp_manufacture_kgsbeq",
        // And the metric help text.
        "Abiotic resources depletion potential of manufacture",
        boavizta_adp_manufacture_kgsbeq.clone(),
    );

    let boavizta_adp_use_kgsbeq = Family::<SummaryLabels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_adp_use_kgsbeq",
        // And the metric help text.
        "Abiotic resources depletion potential of use",
        boavizta_adp_use_kgsbeq.clone(),
    );

    let boavizta_gwp_manufacture_kgco2eq =
        Family::<SummaryLabels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_gwp_manufacture_kgco2eq",
        // And the metric help text.
        "Global Warming Potential of manufacture",
        boavizta_gwp_manufacture_kgco2eq.clone(),
    );

    let boavizta_gwp_use_kgco2eq = Family::<SummaryLabels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_gwp_use_kgco2eq",
        // And the metric help text.
        "Global Warming Potential of use",
        boavizta_gwp_use_kgco2eq.clone(),
    );

    let summary_labels: SummaryLabels = SummaryLabels {
        awsregion: summary.aws_region.to_string(),
        country: summary.country.to_string(),
        cloud_scanner_version,
        boaviztapi_version,
    };

    // Set the values
    boavizta_number_of_resources_total
        .get_or_create(&summary_labels)
        .set(summary.number_of_resources_total as i64);
    boavizta_number_of_resources_assessed
        .get_or_create(&summary_labels)
        .set(summary.number_of_resources_assessed as i64);

    boavizta_duration_of_use_hours
        .get_or_create(&summary_labels)
        .set(summary.duration_of_use_hours);

    boavizta_pe_manufacture_megajoules
        .get_or_create(&summary_labels)
        .set(summary.pe_manufacture_megajoules);

    boavizta_pe_use_megajoules
        .get_or_create(&summary_labels)
        .set(summary.pe_use_megajoules);

    boavizta_adp_manufacture_kgsbeq
        .get_or_create(&summary_labels)
        .set(summary.adp_manufacture_kgsbeq);

    boavizta_adp_use_kgsbeq
        .get_or_create(&summary_labels)
        .set(summary.adp_use_kgsbeq);

    boavizta_gwp_manufacture_kgco2eq
        .get_or_create(&summary_labels)
        .set(summary.gwp_manufacture_kgco2eq);

    boavizta_gwp_use_kgco2eq
        .get_or_create(&summary_labels)
        .set(summary.gwp_use_kgco2eq);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impact_provider::ImpactsValues;
    use crate::model::{
        CloudProvider, CloudResource, CloudResourceTag, EstimationMetadata, InstanceUsage,
        StorageUsage,
    };
    use crate::usage_location::UsageLocation;

    #[tokio::test]
    async fn test_get_summary_metrics() {
        let summary: ImpactsSummary = ImpactsSummary {
            number_of_resources_total: 5,
            number_of_resources_assessed: 2,
            number_of_resources_not_assessed: 3,
            duration_of_use_hours: 1.0,
            adp_manufacture_kgsbeq: 0.1,
            adp_use_kgsbeq: 0.2,
            pe_manufacture_megajoules: 0.3,
            pe_use_megajoules: 0.4,
            gwp_manufacture_kgco2eq: 0.5,
            gwp_use_kgco2eq: 0.6,
            aws_region: "eu-west-1".to_string(),
            country: "IRL".to_string(),
        };

        let cloud_scannner_version = "4.5.6".to_string();
        let boaviztapi_version = "v1.2.3".to_string();

        let metrics =
            get_summary_metrics(&summary, cloud_scannner_version, boaviztapi_version).unwrap();

        println!("{}", metrics);

        let expected = r#"# HELP boavizta_number_of_resources_total Number of resources detected during the inventory.
# TYPE boavizta_number_of_resources_total gauge
boavizta_number_of_resources_total{awsregion="eu-west-1",country="IRL",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 5
# HELP boavizta_number_of_resources_assessed Number of resources that were considered in the estimation of impacts.
# TYPE boavizta_number_of_resources_assessed gauge
boavizta_number_of_resources_assessed{awsregion="eu-west-1",country="IRL",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 2
# HELP boavizta_duration_of_use_hours Use duration considered to estimate impacts.
# TYPE boavizta_duration_of_use_hours gauge
boavizta_duration_of_use_hours{awsregion="eu-west-1",country="IRL",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 1.0
# HELP boavizta_pe_manufacture_megajoules Energy consumed for manufacture.
# TYPE boavizta_pe_manufacture_megajoules gauge
boavizta_pe_manufacture_megajoules{awsregion="eu-west-1",country="IRL",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.3
# HELP boavizta_pe_use_megajoules Energy consumed during use.
# TYPE boavizta_pe_use_megajoules gauge
boavizta_pe_use_megajoules{awsregion="eu-west-1",country="IRL",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.4
# HELP boavizta_adp_manufacture_kgsbeq Abiotic resources depletion potential of manufacture.
# TYPE boavizta_adp_manufacture_kgsbeq gauge
boavizta_adp_manufacture_kgsbeq{awsregion="eu-west-1",country="IRL",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.1
# HELP boavizta_adp_use_kgsbeq Abiotic resources depletion potential of use.
# TYPE boavizta_adp_use_kgsbeq gauge
boavizta_adp_use_kgsbeq{awsregion="eu-west-1",country="IRL",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.2
# HELP boavizta_gwp_manufacture_kgco2eq Global Warming Potential of manufacture.
# TYPE boavizta_gwp_manufacture_kgco2eq gauge
boavizta_gwp_manufacture_kgco2eq{awsregion="eu-west-1",country="IRL",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.5
# HELP boavizta_gwp_use_kgco2eq Global Warming Potential of use.
# TYPE boavizta_gwp_use_kgco2eq gauge
boavizta_gwp_use_kgco2eq{awsregion="eu-west-1",country="IRL",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.6
# EOF
"#;

        assert_eq!(expected, metrics);
    }

    #[tokio::test]
    async fn test_get_all_metrics_for_instance() {
        let tag1 = CloudResourceTag {
            key: "tag_key_1".to_string(),
            value: Some("tag_value_1".to_string()),
        };
        let tag2 = CloudResourceTag {
            key: "tag_key_2".to_string(),
            value: Some("tag_value_2".to_string()),
        };

        let cloud_resource: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::try_from("eu-west-3").unwrap(),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    state: InstanceState::Running,
                }),
            },
            tags: vec![tag1, tag2],
        };

        let cloud_resource_with_impacts = CloudResourceWithImpacts {
            cloud_resource,
            impacts_values: Some(ImpactsValues {
                adp_manufacture_kgsbeq: 0.1,
                adp_use_kgsbeq: 0.2,
                pe_manufacture_megajoules: 0.3,
                pe_use_megajoules: 0.4,
                gwp_manufacture_kgco2eq: 0.5,
                gwp_use_kgco2eq: 0.6,
                raw_data: None,
            }),
            impacts_duration_hours: 1.0,
        };

        let estimated_inventory: EstimatedInventory = EstimatedInventory {
            metadata: EstimationMetadata {
                description: None,
                boavizta_api_version: Some("v1.2.3".to_owned()),
                cloud_scanner_version: Some("4.5.6".to_owned()),
                estimation_date: None,
                execution_statistics: None,
            },
            impacting_resources: vec![cloud_resource_with_impacts],
        };

        let summary = ImpactsSummary::new(
            "eu-west-3".to_string(),
            "FRA".to_string(),
            &estimated_inventory,
            1.0,
        );
        let metrics = get_all_metrics(&summary, estimated_inventory).unwrap();

        println!("{}", metrics);

        let expected = r#"# HELP boavizta_number_of_resources_total Number of resources detected during the inventory.
# TYPE boavizta_number_of_resources_total gauge
boavizta_number_of_resources_total{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 1
# HELP boavizta_number_of_resources_assessed Number of resources that were considered in the estimation of impacts.
# TYPE boavizta_number_of_resources_assessed gauge
boavizta_number_of_resources_assessed{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 1
# HELP boavizta_duration_of_use_hours Use duration considered to estimate impacts.
# TYPE boavizta_duration_of_use_hours gauge
boavizta_duration_of_use_hours{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 1.0
# HELP boavizta_pe_manufacture_megajoules Energy consumed for manufacture.
# TYPE boavizta_pe_manufacture_megajoules gauge
boavizta_pe_manufacture_megajoules{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.3
# HELP boavizta_pe_use_megajoules Energy consumed during use.
# TYPE boavizta_pe_use_megajoules gauge
boavizta_pe_use_megajoules{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.4
# HELP boavizta_adp_manufacture_kgsbeq Abiotic resources depletion potential of manufacture.
# TYPE boavizta_adp_manufacture_kgsbeq gauge
boavizta_adp_manufacture_kgsbeq{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.1
# HELP boavizta_adp_use_kgsbeq Abiotic resources depletion potential of use.
# TYPE boavizta_adp_use_kgsbeq gauge
boavizta_adp_use_kgsbeq{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.2
# HELP boavizta_gwp_manufacture_kgco2eq Global Warming Potential of manufacture.
# TYPE boavizta_gwp_manufacture_kgco2eq gauge
boavizta_gwp_manufacture_kgco2eq{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.5
# HELP boavizta_gwp_use_kgco2eq Global Warming Potential of use.
# TYPE boavizta_gwp_use_kgco2eq gauge
boavizta_gwp_use_kgco2eq{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.6
# HELP boavizta_resource_duration_of_use_hours Use duration considered to estimate impacts.
# TYPE boavizta_resource_duration_of_use_hours gauge
boavizta_resource_duration_of_use_hours{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Running",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 1.0
# HELP boavizta_resource_pe_embodied_megajoules Energy consumed for manufacture.
# TYPE boavizta_resource_pe_embodied_megajoules gauge
boavizta_resource_pe_embodied_megajoules{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Running",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.3
# HELP boavizta_resource_pe_use_megajoules Energy consumed during use.
# TYPE boavizta_resource_pe_use_megajoules gauge
boavizta_resource_pe_use_megajoules{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Running",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.4
# HELP boavizta_resource_adp_embodied_kgsbeq Abiotic resources depletion potential of embodied impacts.
# TYPE boavizta_resource_adp_embodied_kgsbeq gauge
boavizta_resource_adp_embodied_kgsbeq{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Running",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.1
# HELP boavizta_resource_adp_use_kgsbeq Abiotic resources depletion potential of use.
# TYPE boavizta_resource_adp_use_kgsbeq gauge
boavizta_resource_adp_use_kgsbeq{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Running",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.2
# HELP boavizta_resource_gwp_embodied_kgco2eq Global Warming Potential of embodied impacts.
# TYPE boavizta_resource_gwp_embodied_kgco2eq gauge
boavizta_resource_gwp_embodied_kgco2eq{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Running",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.5
# HELP boavizta_resource_gwp_use_kgco2eq Global Warming Potential of use.
# TYPE boavizta_resource_gwp_use_kgco2eq gauge
boavizta_resource_gwp_use_kgco2eq{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Running",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.6
# HELP boavizta_resource_cpu_load CPU load of instance.
# TYPE boavizta_resource_cpu_load gauge
boavizta_resource_cpu_load{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Running",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 100.0
# HELP boavizta_storage_size_gb Storage size in GB.
# TYPE boavizta_storage_size_gb gauge
# EOF
"#;

        assert_eq!(expected, metrics);
    }

    #[tokio::test]
    async fn test_get_all_metrics_for_storage() {
        let tag1 = CloudResourceTag {
            key: "tag_key_1".to_string(),
            value: Some("tag_value_1".to_string()),
        };
        let tag2 = CloudResourceTag {
            key: "tag_key_2".to_string(),
            value: Some("tag_value_2".to_string()),
        };

        let cloud_resource: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::try_from("eu-west-3").unwrap(),
            resource_details: ResourceDetails::BlockStorage {
                storage_type: "arbitrary-type".to_string(),
                usage: Some(StorageUsage { size_gb: 42 }),
                attached_instances: None,
            },
            tags: vec![tag1, tag2],
        };

        let cloud_resource_with_impacts = CloudResourceWithImpacts {
            cloud_resource,
            impacts_values: Some(ImpactsValues {
                adp_manufacture_kgsbeq: 0.1,
                adp_use_kgsbeq: 0.2,
                pe_manufacture_megajoules: 0.3,
                pe_use_megajoules: 0.4,
                gwp_manufacture_kgco2eq: 0.5,
                gwp_use_kgco2eq: 0.6,
                raw_data: None,
            }),
            impacts_duration_hours: 1.0,
        };

        let estimated_inventory: EstimatedInventory = EstimatedInventory {
            metadata: EstimationMetadata {
                description: None,
                boavizta_api_version: Some("v1.2.3".to_owned()),
                cloud_scanner_version: Some("4.5.6".to_owned()),
                estimation_date: None,
                execution_statistics: None,
            },
            impacting_resources: vec![cloud_resource_with_impacts],
        };

        let summary = ImpactsSummary::new(
            "eu-west-3".to_string(),
            "FRA".to_string(),
            &estimated_inventory,
            1.0,
        );

        let metrics = get_all_metrics(&summary, estimated_inventory).unwrap();

        println!("{}", metrics);

        let expected = r#"# HELP boavizta_number_of_resources_total Number of resources detected during the inventory.
# TYPE boavizta_number_of_resources_total gauge
boavizta_number_of_resources_total{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 1
# HELP boavizta_number_of_resources_assessed Number of resources that were considered in the estimation of impacts.
# TYPE boavizta_number_of_resources_assessed gauge
boavizta_number_of_resources_assessed{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 1
# HELP boavizta_duration_of_use_hours Use duration considered to estimate impacts.
# TYPE boavizta_duration_of_use_hours gauge
boavizta_duration_of_use_hours{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 1.0
# HELP boavizta_pe_manufacture_megajoules Energy consumed for manufacture.
# TYPE boavizta_pe_manufacture_megajoules gauge
boavizta_pe_manufacture_megajoules{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.3
# HELP boavizta_pe_use_megajoules Energy consumed during use.
# TYPE boavizta_pe_use_megajoules gauge
boavizta_pe_use_megajoules{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.4
# HELP boavizta_adp_manufacture_kgsbeq Abiotic resources depletion potential of manufacture.
# TYPE boavizta_adp_manufacture_kgsbeq gauge
boavizta_adp_manufacture_kgsbeq{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.1
# HELP boavizta_adp_use_kgsbeq Abiotic resources depletion potential of use.
# TYPE boavizta_adp_use_kgsbeq gauge
boavizta_adp_use_kgsbeq{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.2
# HELP boavizta_gwp_manufacture_kgco2eq Global Warming Potential of manufacture.
# TYPE boavizta_gwp_manufacture_kgco2eq gauge
boavizta_gwp_manufacture_kgco2eq{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.5
# HELP boavizta_gwp_use_kgco2eq Global Warming Potential of use.
# TYPE boavizta_gwp_use_kgco2eq gauge
boavizta_gwp_use_kgco2eq{awsregion="eu-west-3",country="FRA",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.6
# HELP boavizta_resource_duration_of_use_hours Use duration considered to estimate impacts.
# TYPE boavizta_resource_duration_of_use_hours gauge
boavizta_resource_duration_of_use_hours{awsregion="eu-west-3",country="FRA",resource_type="BlockStorage",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Unknown",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 1.0
# HELP boavizta_resource_pe_embodied_megajoules Energy consumed for manufacture.
# TYPE boavizta_resource_pe_embodied_megajoules gauge
boavizta_resource_pe_embodied_megajoules{awsregion="eu-west-3",country="FRA",resource_type="BlockStorage",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Unknown",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.3
# HELP boavizta_resource_pe_use_megajoules Energy consumed during use.
# TYPE boavizta_resource_pe_use_megajoules gauge
boavizta_resource_pe_use_megajoules{awsregion="eu-west-3",country="FRA",resource_type="BlockStorage",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Unknown",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.4
# HELP boavizta_resource_adp_embodied_kgsbeq Abiotic resources depletion potential of embodied impacts.
# TYPE boavizta_resource_adp_embodied_kgsbeq gauge
boavizta_resource_adp_embodied_kgsbeq{awsregion="eu-west-3",country="FRA",resource_type="BlockStorage",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Unknown",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.1
# HELP boavizta_resource_adp_use_kgsbeq Abiotic resources depletion potential of use.
# TYPE boavizta_resource_adp_use_kgsbeq gauge
boavizta_resource_adp_use_kgsbeq{awsregion="eu-west-3",country="FRA",resource_type="BlockStorage",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Unknown",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.2
# HELP boavizta_resource_gwp_embodied_kgco2eq Global Warming Potential of embodied impacts.
# TYPE boavizta_resource_gwp_embodied_kgco2eq gauge
boavizta_resource_gwp_embodied_kgco2eq{awsregion="eu-west-3",country="FRA",resource_type="BlockStorage",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Unknown",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.5
# HELP boavizta_resource_gwp_use_kgco2eq Global Warming Potential of use.
# TYPE boavizta_resource_gwp_use_kgco2eq gauge
boavizta_resource_gwp_use_kgco2eq{awsregion="eu-west-3",country="FRA",resource_type="BlockStorage",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Unknown",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 0.6
# HELP boavizta_resource_cpu_load CPU load of instance.
# TYPE boavizta_resource_cpu_load gauge
# HELP boavizta_storage_size_gb Storage size in GB.
# TYPE boavizta_storage_size_gb gauge
boavizta_storage_size_gb{awsregion="eu-west-3",country="FRA",resource_type="BlockStorage",resource_id="inst-1",resource_tags="tag_key_1:tag_value_1;tag_key_2:tag_value_2;",resource_state="Unknown",cloud_scanner_version="4.5.6",boaviztapi_version="v1.2.3"} 42
# EOF
"#;

        assert_eq!(expected, metrics);
    }
}
