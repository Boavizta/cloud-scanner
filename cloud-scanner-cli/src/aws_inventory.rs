//! A module to perform inventory of cloud resources

use crate::cloud_inventory::CloudInventory;
use crate::cloud_resource::*;
use crate::usage_location::*;
use anyhow::{Context, Result};
use aws_sdk_cloudwatch::model::{Dimension, StandardUnit, Statistic};
use aws_sdk_cloudwatch::output::GetMetricStatisticsOutput;
use aws_sdk_ec2::model::Instance;
use aws_sdk_ec2::Region;
use chrono::Duration;
use chrono::Utc;

use async_trait::async_trait;

///  An inventory of AWS resources
#[derive(Clone, Debug)]
pub struct AwsInventory {
    aws_region: String,
    ec2_client: aws_sdk_ec2::Client,
    cloudwatch_client: aws_sdk_cloudwatch::Client,
}

impl AwsInventory {
    /// Creates an AWS inventory
    /// Initialize it with a specific region an configure the SDK's taht will do the inventory.
    pub async fn new(aws_region: &str) -> Self {
        let shared_config = Self::load_aws_config(aws_region).await;
        AwsInventory {
            aws_region: String::from(aws_region),
            ec2_client: aws_sdk_ec2::Client::new(&shared_config),
            cloudwatch_client: aws_sdk_cloudwatch::Client::new(&shared_config),
        }
    }

    /// Initialize a sdk config with default credentials and  region passed as argument
    /// If region is empty, uses a default region (but will return no error even if the region is invalid)
    async fn load_aws_config(aws_region: &str) -> aws_types::sdk_config::SdkConfig {
        if aws_region.is_empty() {
            // Use default region (from env)
            let sdk_config = aws_config::from_env().load().await;
            warn!(
                "Cannot initialize from empty region, fallng back to using default region from environement [{}]",
                sdk_config.region().unwrap()
            );
            sdk_config
        } else {
            let sdk_config = aws_config::from_env()
                .region(Region::new(String::from(aws_region)))
                .load()
                .await;
            info!("Initializing SDK with with region [{}]", aws_region);
            sdk_config
        }
    }

    /// List all instances of the current account
    ///
    /// Filtering instance on tags is not yet implemented.
    async fn list_instances(self, tags: &Vec<String>) -> Result<Vec<Instance>> {
        warn!("Warning: filtering on tags is not implemented {:?}", tags);

        let client = &self.ec2_client;
        let mut instances: Vec<Instance> = Vec::new();
        // Filter: AND on name, OR on values
        //let filters :std::vec::Vec<aws_sdk_ec2::model::Filter>;
        let resp = client
            .describe_instances()
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

    /// Returns average CPU load of an instance
    async fn get_average_cpu(self, instance_id: &str) -> Result<f64> {
        let res = self
            .get_average_cpu_usage_of_last_5_minutes(instance_id)
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

    /// Returns the instance CPU utilization usage on the last 24 hours
    /// duration seconds seems to be the sampling period
    async fn get_average_cpu_usage_of_last_5_minutes(
        self,
        instance_id: &str,
    ) -> Result<GetMetricStatisticsOutput, aws_sdk_cloudwatch::Error> {
        // We want statistics about the last 5 minutes using 60 sec sample
        let measure_duration = Duration::minutes(5);
        let sample_period_seconds = 60;
        let now: chrono::DateTime<Utc> = Utc::now();
        let start_time: chrono::DateTime<Utc> = now - measure_duration;

        let cpu_metric_name = String::from("CPUUtilization");
        let ec2_namespace = "AWS/EC2";

        let dimensions = vec![Dimension::builder()
            .name("InstanceId")
            .value(instance_id)
            .build()];

        let end_time_aws: aws_sdk_cloudwatch::types::DateTime =
            aws_sdk_cloudwatch::types::DateTime::from_secs(now.timestamp());
        let start_time_aws: aws_sdk_cloudwatch::types::DateTime =
            aws_sdk_cloudwatch::types::DateTime::from_secs(start_time.timestamp());

        let resp = self
            .cloudwatch_client
            .get_metric_statistics()
            .end_time(end_time_aws)
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
}

#[async_trait]
impl CloudInventory for AwsInventory {
    /// list resources
    async fn list_resources(&self, tags: &Vec<String>) -> Result<Vec<CloudResource>> {
        let instances: Vec<Instance> = self
            .clone()
            .list_instances(tags)
            .await
            .context("Cannot list instances")
            .unwrap();
        // let usages = instances.iter().map(|i| self.get_average_cpu( i.instance_id().unwrap()).unwrap()).collect();
        let location = UsageLocation::from(self.aws_region.as_str());

        let mut res: Vec<CloudResource> = Vec::new();
        for instance in instances {
            let instance_id = instance.instance_id().unwrap().to_string();
            let cpuload: f64 = self
                .clone()
                .get_average_cpu(&instance_id)
                .await
                .context("Cannot get CPU load of instance")
                .unwrap();

            let usage: CloudResourceUsage = CloudResourceUsage {
                average_cpu_load: cpuload,
                usage_duration_seconds: 300,
            };

            let cs = CloudResource {
                id: instance_id,
                location: location.clone(),
                resource_type: String::from("ec2"),
                usage: Some(usage),
            };
            res.push(cs);
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_list_resources() {
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;
        let tags: Vec<String> = vec!["".to_string()];
        let res: Vec<CloudResource> = inventory
            .list_resources(&tags)
            .await
            .context("Failed to list")
            .unwrap();

        assert_eq!(4, res.len());
    }

    #[tokio::test]
    async fn test_create_sdk_config_works_with_wrong_region() {
        let region: &str = "eu-west-3";
        let config = AwsInventory::load_aws_config(region).await;
        assert_eq!(region, config.region().unwrap().to_string());

        let wrong_region: &str = "impossible-region";
        let config = AwsInventory::load_aws_config(wrong_region).await;
        assert_eq!(wrong_region, config.region().unwrap().to_string())
    }

    // Verify tests from here
    #[tokio::test]
    #[ignore]
    async fn test_get_instance_usage_metrics_of_running_instance() {
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;

        // This instance  needs to be running
        let instance_id = "i-0a3e6b8cdb50c49b8";
        let res = inventory
            .get_average_cpu_usage_of_last_5_minutes(instance_id)
            .await
            .unwrap();
        let datapoints = res.datapoints.unwrap();
        println!("{:#?}", datapoints);
        assert_eq!(1, datapoints.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_instance_usage_metrics_of_shutdown_instance() {
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;
        let instance_id = "i-03e0b3b1246001382";
        let res = inventory
            .get_average_cpu_usage_of_last_5_minutes(instance_id)
            .await
            .unwrap();
        let datapoints = res.datapoints.unwrap();
        assert_eq!(0, datapoints.len());
    }

    #[tokio::test]
    async fn test_get_instance_usage_metrics_of_non_existing_instance() {
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;
        let instance_id = "IDONOTEXISTS";
        let res = inventory
            .get_average_cpu_usage_of_last_5_minutes(instance_id)
            .await
            .unwrap();
        let datapoints = res.datapoints.unwrap();
        assert_eq!(0, datapoints.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_average_cpu_load_of_running_instance_is_not_zero() {
        // This instance  needs to be running for the test to pass
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;

        let instance_id = "i-03c8f84a6318a8186";
        let avg_cpu_load = inventory.get_average_cpu(instance_id).await.unwrap();
        assert_ne!(0 as f64, avg_cpu_load);
        println!("{:#?}", avg_cpu_load);
        assert!((0 as f64) < avg_cpu_load);
        assert!((100 as f64) > avg_cpu_load);
    }

    #[tokio::test]
    async fn test_average_cpu_load_of_non_existing_instance_is_zero() {
        let instance_id = "IDONOTEXISTS";
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;
        let res = inventory.get_average_cpu(instance_id).await.unwrap();
        assert_eq!(0 as f64, res);
    }

    #[tokio::test]
    async fn test_average_cpu_load_of_shutdown_instance_is_zero() {
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;
        let instance_id = "i-03e0b3b1246001382";
        let res = inventory.get_average_cpu(instance_id).await.unwrap();
        assert_eq!(0 as f64, res);
    }
}
