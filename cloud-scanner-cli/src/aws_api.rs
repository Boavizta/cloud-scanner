//! Provide access to AWS inventory APIs
use aws_sdk_cloudwatch::model::{Dimension, StandardUnit, Statistic};
use aws_sdk_cloudwatch::output::GetMetricStatisticsOutput;
// use aws_sdk_cloudwatch::output::ListMetricsOutput;
use anyhow::{Context, Result};
use aws_sdk_cloudwatch::Client as CW_client;
use aws_sdk_ec2::model::Instance;
use aws_sdk_ec2::{Client, Region};
use chrono::Duration;

//use aws_smithy_types_convert::date_time::DateTimeExt;
use chrono::Utc;

/// Init a sdk config with default credentials and given region.
/// If region is empty, uses a default region (but will return no error even if the region is invalid)
pub async fn init_aws_config(aws_region: &str) -> aws_types::sdk_config::SdkConfig {
    if aws_region.is_empty() {
        // Use default region (from env)
        let sdk_config = aws_config::from_env().load().await;
        warn!(
            "Cannot parse region, using default region [{}]",
            sdk_config.region().unwrap()
        );
        sdk_config
    } else {
        let sdk_config = aws_config::from_env()
            .region(Region::new(String::from(aws_region)))
            .load()
            .await;
        info!("Using region {}", aws_region);
        sdk_config
    }
}

/// List all instances of the current account
///
/// Filtering instance on tags is not yet implemented.
pub async fn list_instances(tags: &Vec<String>, aws_region: &str) -> Result<Vec<Instance>> {
    warn!("Warning: skipping tag filer {:?}", tags);

    let shared_config = init_aws_config(aws_region).await;
    let client = Client::new(&shared_config);

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
        println!("Instance ID: {:?}", instance.instance_id());
        println!("Type:       {:?}", instance.instance_type());
        println!("Tags:  {:?}", instance.tags());
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
pub async fn display_instances_as_text(tags: &Vec<String>, aws_region: &str) -> Result<()> {
    let instances = list_instances(tags, aws_region).await.with_context(|| {
        format!(
            "Failed to list instances. region:{}, tags:{:?}",
            aws_region, tags
        )
    })?;
    print_instances(instances.clone());

    for instance in &instances {
        let id = instance.instance_id().unwrap();
        let load = get_average_cpu(id).await?;
        println! {"id: {}, cpuload: {}", &id, &load};
    }
    Ok(())
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
/// duration seconds seems to be the sampling period
async fn get_average_cpu_usage_of_last_5_minutes(
    instance_id: &str,
) -> Result<GetMetricStatisticsOutput, aws_sdk_cloudwatch::Error> {
    let shared_config = aws_config::from_env().load().await;
    let client = CW_client::new(&shared_config);

    let now: chrono::DateTime<Utc> = Utc::now();
    let now_aws = aws_sdk_cloudwatch::types::DateTime::from_secs(now.timestamp());

    //let one_day = Duration::days(1);

    // We want statistics about the last 5 minutes
    let last_minutes = Duration::minutes(5);
    // We want only one value for this duration (the last 5 minutes)  so we set the sampling period to the duration of the last 5 minute to analyse.
    let sample_period_seconds = 60;
    //let period = Duration::minutes(5).num_seconds() as i32;
    let start_time: chrono::DateTime<Utc> = now - last_minutes;

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
        .period(sample_period_seconds)
        .set_dimensions(Some(dimensions))
        .start_time(start_time_aws)
        .statistics(Statistic::Average)
        .unit(StandardUnit::Percent)
        .send()
        .await?;

    Ok(resp)
}

/// Returns average CPU load of an instance
pub async fn get_average_cpu(instance_id: &str) -> Result<f64> {
    let res = get_average_cpu_usage_of_last_5_minutes(instance_id)
        .await
        .with_context(|| {
            format!(
                "Cannot retrieve average CPU load of instance: {}",
                instance_id
            )
        })?;

    if let Some(points) = res.datapoints {
        //  dbg!(points.clone());
        if !points.is_empty() {
            if points.len() > 1 {
                warn!("Some datapoints were skipped when getting instance CPU usage, whe expected a single result but received {}. Only the first was considered", points.len());
            }
            let first_point = &points[0];
            return Ok(first_point.average.unwrap());
        }
    }
    warn!(
        "No CPU load data was returned for instance {}, it is likely stopped, using 0 as load",
        instance_id
    );
    Ok(0 as f64)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_get_instance_usage_metrics_of_running_instance() {
        // This instance  needs to be running
        let instance_id = "i-0a3e6b8cdb50c49b8";
        let res = get_average_cpu_usage_of_last_5_minutes(instance_id)
            .await
            .unwrap();
        let datapoints = res.datapoints.unwrap();
        println!("{:#?}", datapoints);
        assert_eq!(1, datapoints.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_instance_usage_metrics_of_shutdown_instance() {
        let instance_id = "i-03e0b3b1246001382";
        let res = get_average_cpu_usage_of_last_5_minutes(instance_id)
            .await
            .unwrap();
        let datapoints = res.datapoints.unwrap();
        assert_eq!(0, datapoints.len());
    }

    #[tokio::test]
    async fn test_get_instance_usage_metrics_of_non_existing_instance() {
        let instance_id = "IDONOTEXISTS";
        let res = get_average_cpu_usage_of_last_5_minutes(instance_id)
            .await
            .unwrap();
        let datapoints = res.datapoints.unwrap();
        assert_eq!(0, datapoints.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_average_cpu_load_24hrs_of_running_instance() {
        // This instance  needs to be running for the test to pass
        let instance_id = "i-03c8f84a6318a8186";
        let avg_cpu_load = get_average_cpu(instance_id).await.unwrap();
        assert_ne!(0 as f64, avg_cpu_load);
        println!("{:#?}", avg_cpu_load);
        assert!((0 as f64) < avg_cpu_load);
        assert!((100 as f64) > avg_cpu_load);
    }

    #[tokio::test]
    async fn test_average_cpu_load_24hrs_of_non_existing_instance() {
        let instance_id = "IDONOTEXISTS";
        let res = get_average_cpu(instance_id).await.unwrap();
        assert_eq!(0 as f64, res);
    }

    #[tokio::test]
    async fn test_average_cpu_load_24hrs_of_shutdown_instance() {
        let instance_id = "i-03e0b3b1246001382";
        let res = get_average_cpu(instance_id).await.unwrap();
        assert_eq!(0 as f64, res);
    }

    //#[tokio::test]
    // async fn test_get_current_region() {
    //     let reg: String = get_current_aws_region().await;
    //     assert_eq!("eu-west-1", reg);
    // }

    #[tokio::test]
    async fn test_create_sdk_config() {
        let region: &str = "eu-west-3";
        let config = init_aws_config(region).await;

        assert_eq!(region, config.region().unwrap().to_string());

        let wrong_region: &str = "impossible-region";
        let config = init_aws_config(wrong_region).await;

        assert_eq!(wrong_region, config.region().unwrap().to_string())
    }
}
