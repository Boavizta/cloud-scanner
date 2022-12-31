//!  Formatting results into Prometheus metrics
use anyhow::{Context, Result};
use std::sync::atomic::AtomicU64;

use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::*;
use prometheus_client::registry::Registry;

//use crate::impact_provider::ImpactsSummary;
use crate::ImpactsSummary;

// Define a type representing a metric label set, i.e. a key value pair.
//
// You could as well use `(String, String)` to represent a label set,
// instead of the custom type below.
#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet, Debug)]
pub struct Labels {
    pub awsregion: String,
    // Or just a plain string.
    pub country: String,
}

/// Retursn the ImpactsSumary as metrics in the prometheus format
pub fn get_metrics(summary: &ImpactsSummary) -> Result<String> {
    let label_set: Labels = Labels {
        awsregion: summary.aws_region.to_string(),
        country: summary.country.to_string(),
    };

    let registry = register_all_metrics_new(summary, label_set);

    let mut buffer = String::new();
    encode(&mut buffer, &registry).context("Fails to encode result into metrics")?;
    let metrics = String::from(buffer);

    Ok(metrics)
}

fn register_all_metrics_new(summary: &ImpactsSummary, label_set: Labels) -> Registry {
    // Create a metric registry.
    //
    // Note the angle brackets to make sure to use the default (dynamic
    // dispatched boxed metric) for the generic type parameter.
    let mut registry = <Registry>::default();

    let boavizta_number_of_instances_total = Family::<Labels, Gauge>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_number_of_instances_total",
        // And the metric help text.
        "Number of instances detected during the inventory",
        boavizta_number_of_instances_total.clone(),
    );

    let boavizta_number_of_instances_assessed = Family::<Labels, Gauge>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_number_of_instances_assessed",
        // And the metric help text.
        "Number of instances that were considered in the estimation of impacts",
        boavizta_number_of_instances_assessed.clone(),
    );

    let boavizta_duration_of_use_hours = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_duration_of_use_hours",
        // And the metric help text.
        "Use duration considered to estimate impacts",
        boavizta_duration_of_use_hours.clone(),
    );

    let boavizta_pe_manufacture_megajoules = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_pe_manufacture_megajoules",
        // And the metric help text.
        "Energy consumed for manufacture",
        boavizta_pe_manufacture_megajoules.clone(),
    );

    let boavizta_pe_use_megajoules = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_pe_use_megajoules",
        // And the metric help text.
        "Energy consumed during use",
        boavizta_pe_use_megajoules.clone(),
    );

    let boavizta_adp_manufacture_kgsbeq = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_adp_manufacture_kgsbeq",
        // And the metric help text.
        "Abiotic resources depletion potential of manufacture",
        boavizta_adp_manufacture_kgsbeq.clone(),
    );

    let boavizta_adp_use_kgsbeq = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_adp_use_kgsbeq",
        // And the metric help text.
        "Abiotic resources depletion potential of use",
        boavizta_adp_use_kgsbeq.clone(),
    );

    let boavizta_gwp_manufacture_kgco2eq = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_gwp_manufacture_kgco2eq",
        // And the metric help text.
        "Global Warming Potential of manufacture",
        boavizta_gwp_manufacture_kgco2eq.clone(),
    );

    let boavizta_gwp_use_kgco2eq = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_gwp_use_kgco2eq",
        // And the metric help text.
        "Global Warming Potential of use",
        boavizta_gwp_use_kgco2eq.clone(),
    );

    // Set the values
    boavizta_number_of_instances_total
        .get_or_create(&label_set)
        .set(summary.number_of_instances_total.into());
    boavizta_number_of_instances_assessed
        .get_or_create(&label_set)
        .set(summary.number_of_instances_assessed.into());

    boavizta_duration_of_use_hours
        .get_or_create(&label_set)
        .set(summary.duration_of_use_hours);

    boavizta_pe_manufacture_megajoules
        .get_or_create(&label_set)
        .set(summary.pe_manufacture_megajoules);

    boavizta_pe_use_megajoules
        .get_or_create(&label_set)
        .set(summary.pe_use_megajoules);

    boavizta_adp_manufacture_kgsbeq
        .get_or_create(&label_set)
        .set(summary.adp_manufacture_kgsbeq);

    boavizta_adp_use_kgsbeq
        .get_or_create(&label_set)
        .set(summary.adp_use_kgsbeq);

    boavizta_gwp_manufacture_kgco2eq
        .get_or_create(&label_set)
        .set(summary.gwp_manufacture_kgco2eq);

    boavizta_gwp_use_kgco2eq
        .get_or_create(&label_set)
        .set(summary.gwp_use_kgco2eq);

    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_get_metrics() {
        let summary: ImpactsSummary = ImpactsSummary {
            number_of_instances_total: 5,
            number_of_instances_assessed: 2,
            number_of_instances_not_assessed: 3,
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

        let metrics = get_metrics(&summary).unwrap();

        println!("{}", metrics);

        let expected = r#"# HELP boavizta_number_of_instances_total Number of instances detected during the inventory.
# TYPE boavizta_number_of_instances_total gauge
boavizta_number_of_instances_total{awsregion="eu-west-1",country="IRL"} 5
# HELP boavizta_number_of_instances_assessed Number of instances that were considered in the estimation of impacts.
# TYPE boavizta_number_of_instances_assessed gauge
boavizta_number_of_instances_assessed{awsregion="eu-west-1",country="IRL"} 2
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
}
