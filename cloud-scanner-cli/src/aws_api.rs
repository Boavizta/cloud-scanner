//use std::time::SystemTime;

use aws_sdk_cloudwatch::output::GetMetricStatisticsOutput;
use chrono::prelude::*;
use chrono::Duration;
//use chrono::{DateTime, TimeZone, Utc};
//use std::time::Duration;

//use aws_sdk_cloudwatch::;
//use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cloudwatch;
//use aws_sdk_cloudwatch::types::{DateTime};
use aws_sdk_cloudwatch::model::{Dimension, Metric, StandardUnit, Statistic};
use aws_sdk_cloudwatch::output::ListMetricsOutput;
use aws_sdk_cloudwatch::Client as CW_client;
use aws_sdk_ec2::model::Instance;
use aws_sdk_ec2::{Client, Error /*Region*/};

use aws_smithy_types_convert::date_time::DateTimeExt;
use chrono::Utc;

// struct InstanceUsage {
//     instance: aws_sdk_ec2::model::Instance,
//     timestamp: f64,
//     duration: f64,
//     usage: Vec<Usage>,
// }

// struct Usage {
//     percentile: u8,
//     cpu_usage: u8,
//     mem_consumption: u8,
// }

/// List all instances of the current account
///
/// Filtering instance on tags is not yet implemented.
pub async fn list_instances(tags: Vec<String>) -> Result<Vec<Instance>, Error> {
    eprintln!("Warning: skipping tag filer {:?}", tags);

    let shared_config = aws_config::from_env()
        //.region(Region::new("eu-west-1"))
        .load()
        .await;
    let client = Client::new(&shared_config);

    let mut instances: Vec<Instance> = Vec::new();

    let resp = client
        .describe_instances()
        //.set_instance_ids(Some(ids))
        //.set_filters() // Use filters for tags
        .send()
        .await?;

    for reservation in resp.reservations().unwrap_or_default() {
        for instance in reservation.instances().unwrap_or_default() {
            instances.push(instance.clone());
        }
    }
    Ok(instances)
}

/// Prints some instance details
fn print_instances(instances: Vec<Instance>) {
    for instance in &instances {
        println!("Instance ID: {}", instance.instance_id().unwrap());
        println!("Type:       {:?}", instance.instance_type().unwrap());
        println!("Tags:  {:?}", instance.tags().unwrap());
        println!();
    }
}

// async fn show_regions(client: &Client) -> Result<(), Error> {
//     let rsp = client.describe_regions().senduse chrono::prelude::*;
//     println!("Regions:");
//     for region in rsp.regions().unwrap_or_default() {
//         println!("  {}", region.region_name().unwrap());
//     }
//     Ok(())
// }

/// Query account for instances and display as text
pub async fn display_instances_as_text(tags: Vec<String>) {
    let instances = list_instances(tags).await;
    print_instances(instances.unwrap());
}

// List metrics.
// snippet-start:[cloudwatch.rust.list-metrics]
pub async fn list_instance_metrics() -> Result<ListMetricsOutput, aws_sdk_cloudwatch::Error> {
    let shared_config = aws_config::from_env().load().await;
    let client = CW_client::new(&shared_config);

    let ec2_namespace = Some(String::from("AWS/EC2"));

    let lmo = client
        .list_metrics()
        .set_namespace(ec2_namespace)
        .send()
        .await?;
    let metrics = lmo.metrics().unwrap_or_default();

    let num_metrics = metrics.len();

    println!("Found {} metrics.", num_metrics);

    Ok(lmo)
}

pub fn print_metrics(metrics: &[Metric]) {
    for metric in metrics {
        println!("Namespace: {}", metric.namespace().unwrap_or_default());
        println!("Name:      {}", metric.metric_name().unwrap_or_default());
        println!("Dimensions:");

        if let Some(dimension) = metric.dimensions.as_ref() {
            for d in dimension {
                println!("  Name:  {}", d.name().unwrap_or_default());
                println!("  Value: {}", d.value().unwrap_or_default());
                println!();
            }
        }
        println!("Found {} metrics.", metrics.len());
    }
}

// {
//     "Metrics": [
//       {
//           "Namespace": "AWS/EC2",
//           "Dimensions": [
//               {
//                   "Name": "InstanceId",
//                   "Value": "i-1234567890abcdef0"
//               }
//           ],
//           "MetricName": "NetworkOut"
//       },
//       {
//           "Namespace": "AWS/EC2",
//           "Dimensions": [
//               {
//                   "Name": "InstanceId",
//                   "Value": "i-1234567890abcdef0"
//               }
//           ],
//           "MetricName": "CPUUtilization"
//       },
//       {
//           "Namespace": "AWS/EC2",
//           "Dimensions": [
//               {
//                   "Name": "InstanceId",
//                   "Value": "i-1234567890abcdef0"
//               }
//           ],
//           "MetricName": "NetworkIn"
//       },
//       ...
//     ]
//   }
async fn get_instance_usage(
    instance_id: String,
) -> Result<GetMetricStatisticsOutput, aws_sdk_cloudwatch::Error> {
    let shared_config = aws_config::from_env().load().await;
    let client = CW_client::new(&shared_config);

    // Prepare some variables used in the query
    //let now = time::Instant::now();
    //let now_as_date_time = DateTime::from(SystemTime::now());

    //SystemTime::from(U)

    // let one_day = Duration::days(1);
    // let period = one_day.num_seconds() as i32;
    // let start_time = (now_as_date_time - DateTime
    //     ::from(one_day.to_std().unwrap() * 2));

    //     let chrono_date_time: chrono::DateTime<Utc> = DateTime::from_secs(5).to_chrono_utc();
    // let date_time: DateTime = DateTime::from_chrono_utc(chrono_date_time);

    let now: chrono::DateTime<Utc> = Utc::now();
    let now_aws = aws_sdk_cloudwatch::types::DateTime::from_secs(now.timestamp());

    let one_day = Duration::days(1);
    let period = one_day.num_seconds() as i32;
    let start_time: chrono::DateTime<Utc> = (now - (one_day * 2)).into();

    //let start_time: aws_smithy_types::DateTime;
    let start_time_aws: aws_sdk_cloudwatch::types::DateTime;
    start_time_aws = aws_sdk_cloudwatch::types::DateTime::from_secs(start_time.timestamp());

    let cpu_metric_name = String::from("CPUUtilization");
    let ec2_namespace = "AWS/EC2";

    println!("{}", instance_id);
    let dimensions = vec![Dimension::builder()
        .name("InstanceId")
        // .value("i-03e0b3b1246001382")
        .value(instance_id)
        .build()];

    println!(
        "{:?} {:?} {:?} {:?}",
        start_time, start_time_aws, now, now_aws
    );

    let resp = client
        .get_metric_statistics()
        .end_time(now_aws)
        .metric_name(cpu_metric_name)
        .namespace(ec2_namespace)
        .period(period)
        .set_dimensions(Some(dimensions))
        .start_time(start_time_aws)
        .statistics(Statistic::Average)
        .unit(StandardUnit::Percent)
        .send()
        .await?;

    Ok(resp)
}

#[tokio::test]
async fn test_get_and_print_metrics() {
    let lmo: ListMetricsOutput = list_instance_metrics().await.unwrap();
    let metrics = lmo.metrics().unwrap();

    assert!(46 <= metrics.len());
    print_metrics(metrics);
}

#[tokio::test]
async fn test_get_instance_usage_metrics() {
    //let instance_id = String::from("i-03e0b3b1246001382");
    let instance_id = String::from("i-001dc0ebbf9cb25c0");

    let res = get_instance_usage(instance_id).await.unwrap();

    let datapoints = res.datapoints.unwrap();
    assert_eq!(2, datapoints.len());
}

#[tokio::test]
async fn test_get_instance_usage_metrics_of_shutdown_instance() {
    let instance_id = String::from("i-03e0b3b1246001382");

    let res = get_instance_usage(instance_id).await.unwrap();

    let datapoints = res.datapoints.unwrap();
    assert_eq!(0, datapoints.len());
}
