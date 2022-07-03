use std::sync::atomic::AtomicU64;

use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::text::Encode;
//use prometheus_client::metrics::counter::{Atomic, Counter};
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::*;
use prometheus_client::registry::Registry;

use crate::model::ScanResultSummary;

// Define a type representing a metric label set, i.e. a key value pair.
//
// You could as well use `(String, String)` to represent a label set,
// instead of the custom type below.
#[derive(Clone, Hash, PartialEq, Eq, Encode)]
pub struct Labels {
    pub awsregion: String,
    // Or just a plain string.
    pub country: String,
}

/// Returns the metrics
pub fn get_metrics(summary: &ScanResultSummary) -> String {
    let label_set: Labels = Labels {
        awsregion: summary.aws_region.to_string(),
        country: summary.country.to_string(),
    };

    let registry = register_all_metrics(summary, label_set);

    let mut buffer = vec![];
    encode(&mut buffer, &registry).unwrap();

    String::from_utf8(buffer).unwrap()
}

fn register_all_metrics(summary: &ScanResultSummary, label_set: Labels) -> Registry {
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
        "Number of instances detected during the scan",
        Box::new(boavizta_number_of_instances_total.clone()),
    );

    let boavizta_number_of_instances_assessed = Family::<Labels, Gauge>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_number_of_instances_assessed",
        // And the metric help text.
        "Number of instances that were considered in the measure",
        Box::new(boavizta_number_of_instances_assessed.clone()),
    );

    let boavizta_duration_of_use_hours = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_duration_of_use_hours",
        // And the metric help text.
        "Number of instances detected during the scan",
        Box::new(boavizta_duration_of_use_hours.clone()),
    );

    let boavizta_pe_manufacture_megajoules = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_pe_manufacture_megajoules",
        // And the metric help text.
        "Power consumed for manufacture",
        Box::new(boavizta_pe_manufacture_megajoules.clone()),
    );

    let boavizta_pe_use_megajoules = Family::<Labels, Gauge<f64, AtomicU64>>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_pe_use_megajoules",
        // And the metric help text.
        "Power consumed during usage",
        Box::new(boavizta_pe_use_megajoules.clone()),
    );

    // Set the values
    boavizta_number_of_instances_total
        .get_or_create(&label_set)
        .set(summary.number_of_instances_total.into());
    boavizta_number_of_instances_assessed
        .get_or_create(&label_set)
        .set(summary.number_of_instances_not_assessed.into());

    boavizta_duration_of_use_hours
        .get_or_create(&label_set)
        .set(summary.duration_of_use_hours);

    boavizta_pe_manufacture_megajoules
        .get_or_create(&label_set)
        .set(summary.pe_manufacture_megajoules);

    boavizta_pe_use_megajoules
        .get_or_create(&label_set)
        .set(summary.pe_use_megajoules);

    registry
}

#[tokio::test]
async fn test_get_get_metrics() {
    let summary: ScanResultSummary = ScanResultSummary {
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

    let metrics = get_metrics(&summary);

    //println!("{}", metrics);

    let expected = r#"# HELP boavizta_number_of_instances_total Number of instances detected during the scan.
# TYPE boavizta_number_of_instances_total gauge
boavizta_number_of_instances_total{awsregion="eu-west-1",country="IRL"} 5
# HELP boavizta_number_of_instances_assessed Number of instances that were considered in the measure.
# TYPE boavizta_number_of_instances_assessed gauge
boavizta_number_of_instances_assessed{awsregion="eu-west-1",country="IRL"} 3
# HELP boavizta_duration_of_use_hours Number of instances detected during the scan.
# TYPE boavizta_duration_of_use_hours gauge
boavizta_duration_of_use_hours{awsregion="eu-west-1",country="IRL"} 1.0
# HELP boavizta_pe_manufacture_megajoules Power consumed for manufacture.
# TYPE boavizta_pe_manufacture_megajoules gauge
boavizta_pe_manufacture_megajoules{awsregion="eu-west-1",country="IRL"} 0.3
# HELP boavizta_pe_use_megajoules Power consumed during usage.
# TYPE boavizta_pe_use_megajoules gauge
boavizta_pe_use_megajoules{awsregion="eu-west-1",country="IRL"} 0.4
# EOF
"#;

    assert_eq!(expected, metrics);
}
