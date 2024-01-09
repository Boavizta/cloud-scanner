//!  Formatting impacts into Prometheus metrics
use anyhow::{Context, Result};
use std::sync::atomic::AtomicU64;

use crate::cloud_resource::{InstanceState, ResourceDetails};
use crate::impact_provider::CloudResourceWithImpacts;
use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::{EncodeLabelSet, EncodeLabelValue};
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::*;
use prometheus_client::registry::Registry;

use crate::ImpactsSummary;

// Define a type representing a metric label set, i.e. a key value pair.
#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet, Debug)]
pub struct SummaryLabels {
    pub awsregion: String,
    pub country: String,
}
#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet, Debug)]
pub struct ResourceLabels {
    pub awsregion: String,
    pub country: String,
    pub resource_type: ResourceType,
    pub resource_id: String,
    pub resource_tags: String,
    pub resource_state: ResourceState,
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

fn build_resource_labels(resource: &CloudResourceWithImpacts) -> ResourceLabels {
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
    // TODO: convert tags to a better format instead of using the default debug string
    warn!("Tag strings are not exported as metric labels");
    // TODO escape tag strings properly before exporting in label
    //let tags_string = format!("{:?}", resource.cloud_resource.tags);
    let tags_string = "".into();
    
    ResourceLabels {
        awsregion: resource.cloud_resource.location.aws_region.clone(),
        country: resource.cloud_resource.location.iso_country_code.clone(),
        resource_type,
        resource_id: resource.cloud_resource.id.clone(),
        resource_tags: tags_string,
        resource_state,
    }
}

pub fn register_resource_metrics(
    registry: &mut Registry,
    resources_with_impacts: Vec<CloudResourceWithImpacts>,
) {
    // Register metrics
    let boavizta_resource_duration_of_use_hours = Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
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
    let boavizta_resource_pe_use_megajoules = Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "boavizta_resource_pe_use_megajoules",
        "Energy consumed during use",
        boavizta_resource_pe_use_megajoules.clone(),
    );
    let boavizta_resource_adp_embodied_kgsbeq = Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "boavizta_resource_adp_embodied_kgsbeq",
        "Abiotic resources depletion potential of embodied impacts",
        boavizta_resource_adp_embodied_kgsbeq.clone(),
    );
    let boavizta_resource_adp_use_kgsbeq = Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
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
    let boavizta_resource_gwp_use_kgco2eq = Family::<ResourceLabels, Gauge<f64, AtomicU64>>::default();
    registry.register(
        "boavizta_resource_gwp_use_kgco2eq",
        "Global Warming Potential of use",
        boavizta_resource_gwp_use_kgco2eq.clone(),
    );

    // Fill up metrics values
    for resource in resources_with_impacts.iter() {
        let resource_labels = build_resource_labels(resource);
        let impacts = resource.resource_impacts.as_ref().unwrap();

        boavizta_resource_duration_of_use_hours
            .get_or_create(&resource_labels)
            .set(resource.impacts_duration_hours.into());
        boavizta_resource_pe_use_megajoules
            .get_or_create(&resource_labels)
            .set(impacts.pe_use_megajoules);
        boavizta_resource_pe_embodied_megajoules
            .get_or_create(&resource_labels)
            .set(impacts.pe_manufacture_megajoules);
        boavizta_resource_adp_use_kgsbeq
            .get_or_create(&resource_labels)
            .set(impacts.adp_use_kgsbeq);
        boavizta_resource_adp_embodied_kgsbeq
            .get_or_create(&resource_labels)
            .set(impacts.adp_manufacture_kgsbeq);
        boavizta_resource_gwp_use_kgco2eq
            .get_or_create(&resource_labels)
            .set(impacts.gwp_use_kgco2eq);
        boavizta_resource_gwp_embodied_kgco2eq
            .get_or_create(&resource_labels)
            .set(impacts.gwp_manufacture_kgco2eq);
    }
}
///
pub fn get_resources_metrics(
    resources_with_impacts: Vec<CloudResourceWithImpacts>,
) -> Result<String> {
    let mut registry = <Registry>::default();
    register_resource_metrics(&mut registry, resources_with_impacts);
    let mut buffer = String::new();
    encode(&mut buffer, &registry).context("Fails to encode resources impacts into metrics")?;
    let metrics = buffer;

    Ok(metrics)
}

/// Return the ImpactsSummary as metrics in the prometheus format
pub fn get_summary_metrics(summary: &ImpactsSummary) -> Result<String> {
    let mut registry = <Registry>::default();
    register_summary_metrics(&mut registry, summary);
    let mut buffer = String::new();
    encode(&mut buffer, &registry).context("Fails to encode impacts summary into metrics")?;
    let metrics = buffer;
    Ok(metrics)
}

pub fn get_all_metrics(
    summary: &ImpactsSummary,
    resources_with_impacts: Vec<CloudResourceWithImpacts>,
) -> Result<String> {
    let mut registry = <Registry>::default();
    register_summary_metrics(&mut registry, summary);
    register_resource_metrics(&mut registry, resources_with_impacts);

    let mut buffer = String::new();
    encode(&mut buffer, &registry).context("Fails to encode impacts into metrics")?;
    let metrics = buffer;

    Ok(metrics)
}

fn register_summary_metrics(registry: &mut Registry, summary: &ImpactsSummary) {
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
    use crate::cloud_resource::{CloudProvider, CloudResource, InstanceUsage};
    use crate::impact_provider::ResourceImpacts;
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

        let metrics = get_summary_metrics(&summary).unwrap();

        println!("{}", metrics);

        let expected = r#"# HELP boavizta_number_of_resources_total Number of resources detected during the inventory.
# TYPE boavizta_number_of_resources_total gauge
boavizta_number_of_resources_total{awsregion="eu-west-1",country="IRL"} 5
# HELP boavizta_number_of_resources_assessed Number of resources that were considered in the estimation of impacts.
# TYPE boavizta_number_of_resources_assessed gauge
boavizta_number_of_resources_assessed{awsregion="eu-west-1",country="IRL"} 2
# HELP boavizta_duration_of_use_hours Use duration considered to estimate impacts.
# TYPE boavizta_duration_of_use_hours gauge
boavizta_duration_of_use_hours{awsregion="eu-west-1",country="IRL"} 1.0
# HELP boavizta_pe_manufacture_megajoules Energy consumed for manufacture.
# TYPE boavizta_pe_manufacture_megajoules gauge
boavizta_pe_manufacture_megajoules{awsregion="eu-west-1",country="IRL"} 0.3
# HELP boavizta_pe_use_megajoules Energy consumed during use.
# TYPE boavizta_pe_use_megajoules gauge
boavizta_pe_use_megajoules{awsregion="eu-west-1",country="IRL"} 0.4
# HELP boavizta_adp_manufacture_kgsbeq Abiotic resources depletion potential of manufacture.
# TYPE boavizta_adp_manufacture_kgsbeq gauge
boavizta_adp_manufacture_kgsbeq{awsregion="eu-west-1",country="IRL"} 0.1
# HELP boavizta_adp_use_kgsbeq Abiotic resources depletion potential of use.
# TYPE boavizta_adp_use_kgsbeq gauge
boavizta_adp_use_kgsbeq{awsregion="eu-west-1",country="IRL"} 0.2
# HELP boavizta_gwp_manufacture_kgco2eq Global Warming Potential of manufacture.
# TYPE boavizta_gwp_manufacture_kgco2eq gauge
boavizta_gwp_manufacture_kgco2eq{awsregion="eu-west-1",country="IRL"} 0.5
# HELP boavizta_gwp_use_kgco2eq Global Warming Potential of use.
# TYPE boavizta_gwp_use_kgco2eq gauge
boavizta_gwp_use_kgco2eq{awsregion="eu-west-1",country="IRL"} 0.6
# EOF
"#;

        assert_eq!(expected, metrics);
    }
    #[tokio::test]
    async fn test_get_all_metrics() {
        let cloud_resource: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    usage_duration_seconds: 3600,
                    state: InstanceState::Running,
                }),
            },
            tags: Vec::new(),
        };

        let cloud_resource_with_impacts = CloudResourceWithImpacts {
            cloud_resource,
            resource_impacts: Some(ResourceImpacts {
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

        let mut crivec: Vec<CloudResourceWithImpacts> = Vec::new();
        crivec.push(cloud_resource_with_impacts);

        let summary: ImpactsSummary = ImpactsSummary {
            number_of_resources_total: 1,
            number_of_resources_assessed: 1,
            number_of_resources_not_assessed: 0,
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

        let metrics = get_all_metrics(&summary, crivec).unwrap();

        println!("{}", metrics);

        let expected = r#"# HELP boavizta_number_of_resources_total Number of resources detected during the inventory.
# TYPE boavizta_number_of_resources_total gauge
boavizta_number_of_resources_total{awsregion="eu-west-1",country="IRL"} 1
# HELP boavizta_number_of_resources_assessed Number of resources that were considered in the estimation of impacts.
# TYPE boavizta_number_of_resources_assessed gauge
boavizta_number_of_resources_assessed{awsregion="eu-west-1",country="IRL"} 1
# HELP boavizta_duration_of_use_hours Use duration considered to estimate impacts.
# TYPE boavizta_duration_of_use_hours gauge
boavizta_duration_of_use_hours{awsregion="eu-west-1",country="IRL"} 1.0
# HELP boavizta_pe_manufacture_megajoules Energy consumed for manufacture.
# TYPE boavizta_pe_manufacture_megajoules gauge
boavizta_pe_manufacture_megajoules{awsregion="eu-west-1",country="IRL"} 0.3
# HELP boavizta_pe_use_megajoules Energy consumed during use.
# TYPE boavizta_pe_use_megajoules gauge
boavizta_pe_use_megajoules{awsregion="eu-west-1",country="IRL"} 0.4
# HELP boavizta_adp_manufacture_kgsbeq Abiotic resources depletion potential of manufacture.
# TYPE boavizta_adp_manufacture_kgsbeq gauge
boavizta_adp_manufacture_kgsbeq{awsregion="eu-west-1",country="IRL"} 0.1
# HELP boavizta_adp_use_kgsbeq Abiotic resources depletion potential of use.
# TYPE boavizta_adp_use_kgsbeq gauge
boavizta_adp_use_kgsbeq{awsregion="eu-west-1",country="IRL"} 0.2
# HELP boavizta_gwp_manufacture_kgco2eq Global Warming Potential of manufacture.
# TYPE boavizta_gwp_manufacture_kgco2eq gauge
boavizta_gwp_manufacture_kgco2eq{awsregion="eu-west-1",country="IRL"} 0.5
# HELP boavizta_gwp_use_kgco2eq Global Warming Potential of use.
# TYPE boavizta_gwp_use_kgco2eq gauge
boavizta_gwp_use_kgco2eq{awsregion="eu-west-1",country="IRL"} 0.6
# HELP boavizta_resource_duration_of_use_hours Use duration considered to estimate impacts.
# TYPE boavizta_resource_duration_of_use_hours gauge
boavizta_resource_duration_of_use_hours{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="[]",resource_state="Running"} 1.0
# HELP boavizta_resource_pe_embodied_megajoules Energy consumed for manufacture.
# TYPE boavizta_resource_pe_embodied_megajoules gauge
boavizta_resource_pe_embodied_megajoules{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="[]",resource_state="Running"} 0.3
# HELP boavizta_resource_pe_use_megajoules Energy consumed during use.
# TYPE boavizta_resource_pe_use_megajoules gauge
boavizta_resource_pe_use_megajoules{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="[]",resource_state="Running"} 0.4
# HELP boavizta_resource_adp_embodied_kgsbeq Abiotic resources depletion potential of embodied impacts.
# TYPE boavizta_resource_adp_embodied_kgsbeq gauge
boavizta_resource_adp_embodied_kgsbeq{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="[]",resource_state="Running"} 0.1
# HELP boavizta_resource_adp_use_kgsbeq Abiotic resources depletion potential of use.
# TYPE boavizta_resource_adp_use_kgsbeq gauge
boavizta_resource_adp_use_kgsbeq{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="[]",resource_state="Running"} 0.2
# HELP boavizta_resource_gwp_embodied_kgco2eq Global Warming Potential of embodied impacts.
# TYPE boavizta_resource_gwp_embodied_kgco2eq gauge
boavizta_resource_gwp_embodied_kgco2eq{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="[]",resource_state="Running"} 0.5
# HELP boavizta_resource_gwp_use_kgco2eq Global Warming Potential of use.
# TYPE boavizta_resource_gwp_use_kgco2eq gauge
boavizta_resource_gwp_use_kgco2eq{awsregion="eu-west-3",country="FRA",resource_type="Instance",resource_id="inst-1",resource_tags="[]",resource_state="Running"} 0.6
# EOF
"#;

        assert_eq!(expected, metrics);
    }
}
