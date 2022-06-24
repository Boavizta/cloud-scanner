use aws_sdk_cloudwatch::model::{Dimension, StandardUnit, Statistic};
use aws_sdk_cloudwatch::output::GetMetricStatisticsOutput;
// use aws_sdk_cloudwatch::output::ListMetricsOutput;
use aws_sdk_cloudwatch::Client as CW_client;
use aws_sdk_ec2::model::Instance;
use aws_sdk_ec2::{Client, Error, Region};
use chrono::Duration;
use isocountry::CountryCode;

//use aws_smithy_types_convert::date_time::DateTimeExt;
use chrono::Utc;

/// Returns 3 letters ISO code for the country corresponding to the aws region we are logged into
pub async fn get_current_iso_country() -> &'static str {
    let aws_region = get_current_aws_region().await;
    let cc = get_country_from_aws_region(aws_region.as_str());
    cc.alpha3()
}

/// Converts aws region into country
fn get_country_from_aws_region(aws_region: &str) -> CountryCode {
    let cc: CountryCode = match aws_region {
        "eu-west-1" => CountryCode::IRL,
        "eu-west-3" => CountryCode::FRA,
        "eu-east-1" => CountryCode::IRL,
        "eu-west-2" => CountryCode::GBR,
        _ => CountryCode::FRA,
    };
    cc
}

/// List all instances of the current account
///
/// Filtering instance on tags is not yet implemented.
pub async fn list_instances(tags: &Vec<String>) -> Result<Vec<Instance>, Error> {
    warn!("Warning: skipping tag filer {:?}", tags);

    warn!("Always using default region ");
    let shared_config = aws_config::from_env()
        //.region(Region::new("eu-west-1"))
        .load()
        .await;
    let client = Client::new(&shared_config);

    warn!(
        "Always using default region: {}",
        &shared_config.region().unwrap()
    );
    let mut instances: Vec<Instance> = Vec::new();

    // Filter: AND on name, OR on values
    //let filters :std::vec::Vec<aws_sdk_ec2::model::Filter>;

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

/// Returns the AWS region we are connected to
async fn get_current_aws_region() -> String {
    let shared_config = aws_config::from_env().load().await;
    let awr: &Region = shared_config.region().unwrap();

    String::from(awr.as_ref())
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
pub async fn display_instances_as_text(tags: &Vec<String>) {
    let instances = list_instances(tags).await;
    print_instances(instances.unwrap());
}

// List metrics.
// snippet-start:[cloudwatch.rust.list-metrics]
// pub async fn list_instance_metrics() -> Result<ListMetricsOutput, aws_sdk_cloudwatch::Error> {
//     let shared_config = aws_config::from_env().load().await;
//     let client = CW_client::new(&shared_config);

//     let ec2_namespace = Some(String::from("AWS/EC2"));

//     let lmo = client
//         .list_metrics()
//         .set_namespace(ec2_namespace)
//         .send()
//         .await?;
//     let metrics = lmo.metrics().unwrap_or_default();

//     let num_metrics = metrics.len();

//     println!("Found {} metrics.", num_metrics);

//     Ok(lmo)
// }

// pub fn print_metrics(metrics: &[Metric]) {
//     for metric in metrics {
//         println!("Namespace: {}", metric.namespace().unwrap_or_default());
//         println!("Name:      {}", metric.metric_name().unwrap_or_default());
//         println!("Dimensions:");

//         if let Some(dimension) = metric.dimensions.as_ref() {
//             for d in dimension {
//                 println!("  Name:  {}", d.name().unwrap_or_default());
//                 println!("  Value: {}", d.value().unwrap_or_default());
//                 println!();
//             }
//         }
//         println!("Found {} metrics.", metrics.len());
//     }
// }

/// Returns the instance CPU utilization usage on the last 24 hours
async fn get_instance_usage(
    instance_id: &str,
) -> Result<GetMetricStatisticsOutput, aws_sdk_cloudwatch::Error> {
    let shared_config = aws_config::from_env().load().await;
    let client = CW_client::new(&shared_config);

    let now: chrono::DateTime<Utc> = Utc::now();
    let now_aws = aws_sdk_cloudwatch::types::DateTime::from_secs(now.timestamp());

    let one_day = Duration::days(1);
    let period = one_day.num_seconds() as i32;
    //let period = Duration::minutes(5).num_seconds() as i32;
    let start_time: chrono::DateTime<Utc> = now - one_day;

    let start_time_aws: aws_sdk_cloudwatch::types::DateTime =
        aws_sdk_cloudwatch::types::DateTime::from_secs(start_time.timestamp());

    let cpu_metric_name = String::from("CPUUtilization");
    let ec2_namespace = "AWS/EC2";

    let dimensions = vec![Dimension::builder()
        .name("InstanceId")
        .value(instance_id)
        .build()];

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

/// Returns average CPU load of an instance over the last 24 hours or 0 if cannot retrieve the value.
///
pub async fn get_average_cpu_load_24hrs(instance_id: &str) -> f64 {
    let res = get_instance_usage(instance_id).await;
    let res = match res {
        Ok(res) => res,
        Err(e) => {
            warn!(
                "Cannot get cpu usage, returning 0 load. Application error: {}",
                e
            );
            return 0 as f64;
        }
    };

    let datapoints = res.datapoints.unwrap();
    if datapoints.is_empty() {
        warn!(
            "Warning: No load data for instance {}, returning 0 as load",
            instance_id
        );
        0 as f64
    } else {
        let first_point = &datapoints[0];
        first_point.average.unwrap()
    }
}

// #[tokio::test]
// async fn test_get_and_print_metrics() {
//     let lmo: ListMetricsOutput = list_instance_metrics().await.unwrap();
//     let metrics = lmo.metrics().unwrap();

//     assert!(46 <= metrics.len());
//     print_metrics(metrics);
// }

#[tokio::test]
async fn test_get_instance_usage_metrics() {
    let instance_id = "i-001dc0ebbf9cb25c0";
    let res = get_instance_usage(instance_id).await.unwrap();
    let datapoints = res.datapoints.unwrap();
    assert_eq!(1, datapoints.len());
    println!("{:#?}", datapoints);
}

#[tokio::test]
async fn test_get_instance_usage_metrics_of_shutdown_instance() {
    let instance_id = "i-03e0b3b1246001382";
    let res = get_instance_usage(instance_id).await.unwrap();
    let datapoints = res.datapoints.unwrap();
    assert_eq!(0, datapoints.len());
}

#[tokio::test]
async fn test_get_instance_usage_metrics_of_non_existing_instance() {
    let instance_id = "IDONOTEXISTS";
    let res = get_instance_usage(instance_id).await.unwrap();
    let datapoints = res.datapoints.unwrap();
    assert_eq!(0, datapoints.len());
}

#[tokio::test]
async fn test_average_cpu_load_24hrs() {
    let instance_id = "i-001dc0ebbf9cb25c0";
    let res = get_average_cpu_load_24hrs(instance_id).await;
    assert_ne!(0 as f64, res);
    assert!((0.1 as f64) < res);
}

#[tokio::test]
async fn test_average_cpu_load_24hrs_of_non_existing_instance() {
    let instance_id = "IDONOTEXISTS";
    let res = get_average_cpu_load_24hrs(instance_id).await;
    assert_eq!(0 as f64, res);
}

#[tokio::test]
async fn test_average_cpu_load_24hrs_of_shutdown_instance() {
    let instance_id = "i-03e0b3b1246001382";
    let res = get_average_cpu_load_24hrs(instance_id).await;
    assert_eq!(0 as f64, res);
}

#[tokio::test]
async fn test_get_current_region() {
    let reg: String = get_current_aws_region().await;
    assert_eq!("eu-west-1", reg);
}

#[tokio::test]
async fn test_get_country_code_from_region() {
    let region = "eu-west-3";
    let cc = get_country_from_aws_region(region);
    assert_eq!("FRA", cc.alpha3());
}

#[tokio::test]
async fn test_get_current_iso_region() {
    let country_code = get_current_iso_country().await;
    assert_eq!("IRL", country_code);
}
