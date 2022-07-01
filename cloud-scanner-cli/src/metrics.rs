use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::text::Encode;
//use prometheus_client::metrics::counter::{Atomic, Counter};
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::*;
use prometheus_client::registry::Registry;

/// Returns the metrics
fn get_metrics() -> String {
    // Create a metric registry.
    //
    // Note the angle brackets to make sure to use the default (dynamic
    // dispatched boxed metric) for the generic type parameter.
    let mut registry = <Registry>::default();

    // Define a type representing a metric label set, i.e. a key value pair.
    //
    // You could as well use `(String, String)` to represent a label set,
    // instead of the custom type below.
    #[derive(Clone, Hash, PartialEq, Eq, Encode)]
    struct Labels {
        awsregion: String,
        // Or just a plain string.
        country: String,
    }

    // Create a sample counter metric family utilizing the above custom label
    // type, representing the number of HTTP requests received.
    let boavizta_number_of_instances_total = Family::<Labels, Gauge>::default();
    // Register the metric family with the registry.
    registry.register(
        // With the metric name.
        "boavizta_number_of_instances_total",
        // And the metric help text.
        "Number of instances detected during the scan.",
        Box::new(boavizta_number_of_instances_total.clone()),
    );

    // Somewhere in your business logic record a single HTTP GET request.
    boavizta_number_of_instances_total
        .get_or_create(&Labels {
            awsregion: "eu-west-1".to_string(),
            country: "IRL".to_string(),
        })
        .set(15);

    // When a monitoring system like Prometheus scrapes the local node, encode
    // all metrics in the registry in the text format, and send the encoded
    // metrics back.
    let mut buffer = vec![];
    encode(&mut buffer, &registry).unwrap();

    String::from_utf8(buffer).unwrap()
}

#[tokio::test]
async fn test_get_get_metrics() {
    let metrics = get_metrics();

    let expected =
        "# HELP boavizta_number_of_instances_total Number of instances detected during the scan.\n"
            .to_owned()
            + "# TYPE boavizta_number_of_instances_total gauge\n"
            + "boavizta_number_of_instances_total{awsregion=\"eu-west-1\",country=\"IRL\"} 15\n"
            + "# EOF\n";
    assert_eq!(expected, metrics);
}
