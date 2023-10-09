//! A module to perform inventory of  AWS cloud resources.
//!
//!  ⚠ Only ec2 instances are supported  today.
use std::time::Instant;

use crate::cloud_inventory::CloudInventory;
use crate::cloud_resource::*;
use crate::usage_location::*;

use anyhow::{Context, Result};
use aws_sdk_cloudwatch::operation::get_metric_statistics::GetMetricStatisticsOutput;
use aws_sdk_cloudwatch::types::{Dimension, StandardUnit, Statistic};
use aws_sdk_ec2::config::Region;
use aws_sdk_ec2::types::Instance;
use aws_sdk_ec2::types::Volume;
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
    /// Creates an AWS inventory.
    /// Initializes it with a specific region and configures the SDK's that will query your account to perform the inventory of resources.
    pub async fn new(aws_region: &str) -> Self {
        let shared_config = Self::load_aws_config(aws_region).await;
        AwsInventory {
            aws_region: String::from(aws_region),
            ec2_client: aws_sdk_ec2::Client::new(&shared_config),
            cloudwatch_client: aws_sdk_cloudwatch::Client::new(&shared_config),
        }
    }

    /// Initialize a AWS SDK config with default credentials from the environment and  a region passed as argument.
    ///
    /// - If region is empty, uses a default region.
    /// - ⚠  If the region is invalid, it does **not** return error.
    async fn load_aws_config(aws_region: &str) -> aws_types::sdk_config::SdkConfig {
        if aws_region.is_empty() {
            // Use default region (from env)
            let sdk_config = aws_config::from_env().load().await;
            warn!(
                "Cannot initialize from empty region, falling back to using default region from environment [{}]",
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

    /// Convert AWS tags into Cloud Scanner tags
    fn cloud_resource_tags_from_aws_tags(
        aws_tags: Option<&[aws_sdk_ec2::types::Tag]>,
    ) -> Vec<CloudResourceTag> {
        let cloud_resource_tags: Vec<CloudResourceTag> = match aws_tags {
            Some(tags) => {
                let mut cs_tags: Vec<CloudResourceTag> = Vec::new();
                for nt in tags.iter() {
                    let k = nt.key.to_owned().unwrap();
                    let v = nt.value.to_owned();
                    cs_tags.push(CloudResourceTag { key: k, value: v });
                }
                cs_tags
            }
            None => {
                let empty: Vec<CloudResourceTag> = Vec::new();
                empty
            }
        };
        cloud_resource_tags
    }

    /// Perform inventory of all aws instances of the region
    async fn get_instances_with_usage_data(&self, tags: &[String]) -> Result<Vec<CloudResource>> {
        let instances: Vec<Instance> = self
            .clone()
            .list_instances(tags)
            .await
            .context("Cannot list instances")
            .unwrap();
        let location = UsageLocation::from(self.aws_region.as_str());

        // Just to display statistics
        let cpu_ram_info_timer = Instant::now();

        let mut inventory: Vec<CloudResource> = Vec::new();
        for instance in instances {
            let instance_id = instance.instance_id().unwrap().to_string();
            let cpuload: f64 = self
                .clone()
                .get_average_cpu(&instance_id)
                .await
                .context("Cannot get CPU load of instance")
                .unwrap();

            let ramload = self
                .clone()
                .get_average_ram(&instance_id)
                .await
                .context("Cannot get RAM load of instance")
                .unwrap();

            let usage: InstanceUsage = InstanceUsage {
                average_cpu_load: cpuload,
                average_ram_load: ramload,
                usage_duration_seconds: 300,
            };

            let cloud_resource_tags = Self::cloud_resource_tags_from_aws_tags(instance.tags());

            info!(
                "Total time spend querying CPU & RAM load of instances: {:?}",
                cpu_ram_info_timer.elapsed()
            );

            let inst = CloudResource {
                provider: CloudProvider::AWS,
                id: instance_id,
                location: location.clone(),
                resource_details: ResourceDetails::Instance {
                    instance_type: instance.instance_type().unwrap().as_str().to_owned(),
                    usage: Some(usage),
                },

                tags: cloud_resource_tags,
            };

            if inst.has_matching_tags(tags) {
                debug!("Resource matched on tags: {:?}", inst.id);
                inventory.push(inst);
            } else {
                debug!("Filtered instance (tags do not match: {:?}", inst);
            }
            //if cs matches the tags passed in param keep it (push it, otherwise skipp it)
        }

        Ok(inventory)
    }

    /// List all ec2 instances of the current account / region
    ///
    /// ⚠  Filtering instance on tags is not yet implemented. All instances (running or stopped) are returned.
    async fn list_instances(self, _tags: &[String]) -> Result<Vec<Instance>> {
        let client = &self.ec2_client;
        let mut instances: Vec<Instance> = Vec::new();
        // Filter: AND on name, OR on values
        //let filters :std::vec::Vec<aws_sdk_ec2::model::Filter>;

        let resp = client
            .describe_instances()
            //set_filters() // Use filters for tags
            .send()
            .await?;

        for reservation in resp.reservations().unwrap_or_default() {
            for instance in reservation.instances().unwrap_or_default() {
                instances.push(instance.clone());
            }
        }
        Ok(instances)
    }

    /// Returns average CPU load of a given instance.
    ///
    async fn get_average_cpu(self, instance_id: &str) -> Result<f64> {
        let res = self
            .get_average_cpu_usage_of_last_10_minutes(instance_id)
            .await
            .with_context(|| {
                format!(
                    "Cannot retrieve average CPU load of instance: {}",
                    instance_id
                )
            })?;
        if let Some(points) = res.datapoints {
            if !points.is_empty() {
                debug!("Averaging cpu load data point: {:#?}", points);
                let mut sum: f64 = 0.0;
                for x in &points {
                    sum += x.average().unwrap();
                }
                let avg = sum / points.len() as f64;
                return Ok(avg);
            }
        }
        warn!(
            "Unable to get CPU load of  instance {}, it is likely stopped, using 0 as load",
            instance_id
        );
        Ok(0 as f64)
    }

    /// Returns the instance CPU utilization usage on the last 10 minutes
    async fn get_average_cpu_usage_of_last_10_minutes(
        self,
        instance_id: &str,
    ) -> Result<GetMetricStatisticsOutput, aws_sdk_cloudwatch::Error> {
        // We want statistics about the last 10 minutes using  5min  sample
        let measure_duration = Duration::minutes(10);
        let sample_period_seconds = 300; // 5*60 (the default granularity of cloudwatch standard CPU metris)
        let now: chrono::DateTime<Utc> = Utc::now();
        let start_time: chrono::DateTime<Utc> = now - measure_duration;

        let cpu_metric_name = String::from("CPUUtilization");
        let ec2_namespace = "AWS/EC2";

        let dimensions = vec![Dimension::builder()
            .name("InstanceId")
            .value(instance_id)
            .build()];

        let end_time_aws: aws_sdk_cloudwatch::primitives::DateTime =
            aws_sdk_cloudwatch::primitives::DateTime::from_secs(now.timestamp());
        let start_time_aws: aws_sdk_cloudwatch::primitives::DateTime =
            aws_sdk_cloudwatch::primitives::DateTime::from_secs(start_time.timestamp());

        let resp: GetMetricStatisticsOutput = self
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

    /// List all Volumes of current account.
    ///
    /// ⚠  Filtering on tags is not yet implemented.
    async fn list_volumes(self, tags: &[String]) -> Result<Vec<Volume>> {
        warn!(
            "Warning: filtering volumes on tags is not implemented {:?}",
            tags
        );
        let client = &self.ec2_client;
        let mut volumes: Vec<Volume> = Vec::new();
        // Filter: AND on name, OR on values
        //let filters :std::vec::Vec<aws_sdk_ec2::model::Filter>;
        let resp = client
            .describe_volumes()
            //set_filters() // Use filters for tags
            .send()
            .await?;
        for v in resp.volumes().unwrap_or_default() {
            volumes.push(v.clone());
        }
        Ok(volumes)
    }

    /// Perform inventory of all aws volumes of the region
    async fn get_volumes_with_usage_data(&self, tags: &[String]) -> Result<Vec<CloudResource>> {
        let location = UsageLocation::from(self.aws_region.as_str());
        let volumes = self.clone().list_volumes(tags).await.unwrap();
        let mut resources: Vec<CloudResource> = Vec::new();

        for volume in volumes {
            let volume_id = volume.volume_id().unwrap();
            let usage: StorageUsage = StorageUsage {
                size_gb: volume.size().unwrap(),
                usage_duration_seconds: 3600,
            };

            let volume_type: String = volume.volume_type().unwrap().as_str().to_string();
            let disk = CloudResource {
                provider: CloudProvider::AWS,
                id: volume_id.into(),
                location: location.clone(),
                resource_details: ResourceDetails::BlockStorage {
                    storage_type: volume_type,
                    usage: Some(usage),
                },
                tags: Self::cloud_resource_tags_from_aws_tags(volume.tags()),
            };
            resources.push(disk);
        }

        Ok(resources)
    }

    /// Returns average RAM load of a given instance.
    ///
    async fn get_average_ram(self, instance_id: &str) -> Result<f64> {
        let res = self
            .get_average_ram_usage_of_last_10_minutes(instance_id)
            .await
            .with_context(|| {
                format!(
                    "Cannot retrieve average RAM load of instance: {}",
                    instance_id
                )
            })?;
        if let Some(points) = res.datapoints {
            if !points.is_empty() {
                debug!("Averaging ram load data point: {:#?}", points);
                let mut sum: f64 = 0.0;
                for x in &points {
                    sum += x.average().unwrap();
                }
                let avg = sum / points.len() as f64;
                return Ok(avg);
            }
        }
        warn!(
            "Unable to get RAM load of  instance {}, it is likely stopped, using 0 as load",
            instance_id
        );
        Ok(0 as f64)
    }

    /// Returns the instance RAM utilization usage on the last 10 minutes
    async fn get_average_ram_usage_of_last_10_minutes(
        self,
        instance_id: &str,
    ) -> Result<GetMetricStatisticsOutput, aws_sdk_cloudwatch::Error> {
        // We want statistics about the last 10 minutes using  5min  sample
        let measure_duration = Duration::minutes(10);
        let sample_period_seconds = 300; // 5*60 (the default granularity of cloudwatch standard CPU metris)
        let now: chrono::DateTime<Utc> = Utc::now();
        let start_time: chrono::DateTime<Utc> = now - measure_duration;

        let ram_metric_name = String::from("MemoryUtilization");
        let ec2_namespace = "AWS/EC2";

        let dimensions = vec![Dimension::builder()
            .name("InstanceId")
            .value(instance_id)
            .build()];

        let end_time_aws: aws_sdk_cloudwatch::primitives::DateTime =
            aws_sdk_cloudwatch::primitives::DateTime::from_secs(now.timestamp());
        let start_time_aws: aws_sdk_cloudwatch::primitives::DateTime =
            aws_sdk_cloudwatch::primitives::DateTime::from_secs(start_time.timestamp());

        let resp: GetMetricStatisticsOutput = self
            .cloudwatch_client
            .get_metric_statistics()
            .end_time(end_time_aws)
            .metric_name(ram_metric_name)
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
    async fn list_resources(
        &self,
        tags: &[String],
        include_block_storage: bool,
    ) -> Result<Vec<CloudResource>> {
        let mut inventory: Vec<CloudResource> = Vec::new();

        let mut instances = self.clone().get_instances_with_usage_data(tags).await?;
        inventory.append(&mut instances);
        if include_block_storage {
            let mut volumes = self.clone().get_volumes_with_usage_data(tags).await?;
            inventory.append(&mut volumes);
        }
        Ok(inventory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static RUNNING_INSTANCE_ID: &str = "i-03c8f84a6318a8186";

    #[tokio::test]
    #[ignore]
    async fn inventory_should_return_correct_number_of_instances() {
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;
        let filtertags: Vec<String> = Vec::new();
        let res: Vec<CloudResource> = inventory
            .get_instances_with_usage_data(&filtertags)
            .await
            .context("Failed to list")
            .unwrap();
        assert_eq!(4, res.len());

        let inst = res.first().unwrap();
        assert_eq!(3, inst.tags.len(), "Wrong number of tags");
        let tag_map = vec_to_map(inst.tags.clone());
        let v = tag_map.get("Name").unwrap();
        assert_eq!(
            Some("test-boapi".to_string()),
            v.to_owned(),
            "Wrong tag value"
        );
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

    #[tokio::test]
    #[ignore]
    async fn get_cpu_usage_metrics_of_running_instance_should_return_right_number_of_data_points() {
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;
        let res = inventory
            .get_average_cpu_usage_of_last_10_minutes(&RUNNING_INSTANCE_ID)
            .await
            .unwrap();
        let datapoints = res.datapoints.unwrap();
        assert!(
            0 < datapoints.len() && datapoints.len() < 3,
            "Strange number of datapoint returned for instance {}, is it really up ?. I was expecting 1 or 2  but got {} .\n {:#?}",
            &RUNNING_INSTANCE_ID,
            datapoints.len(),
            datapoints
        )
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_instance_usage_metrics_of_shutdown_instance() {
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;
        let instance_id = "i-03e0b3b1246001382";
        let res = inventory
            .get_average_cpu_usage_of_last_10_minutes(instance_id)
            .await
            .unwrap();
        let datapoints = res.datapoints.unwrap();
        assert_eq!(0, datapoints.len(), "Wrong number of datapoint returned");
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_instance_usage_metrics_of_non_existing_instance() {
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;
        let instance_id = "IDONOTEXISTS";
        let res = inventory
            .get_average_cpu_usage_of_last_10_minutes(instance_id)
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

        let avg_cpu_load = inventory
            .get_average_cpu(&RUNNING_INSTANCE_ID)
            .await
            .unwrap();
        assert_ne!(
            0 as f64, avg_cpu_load,
            "CPU load of instance {} is zero, is it really running ?",
            &RUNNING_INSTANCE_ID
        );
        println!("{:#?}", avg_cpu_load);
        assert!((0 as f64) < avg_cpu_load);
        assert!((100 as f64) > avg_cpu_load);
    }

    #[tokio::test]
    #[ignore]
    async fn test_average_cpu_load_of_non_existing_instance_is_zero() {
        let instance_id = "IDONOTEXISTS";
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;
        let res = inventory.get_average_cpu(instance_id).await.unwrap();
        assert_eq!(0 as f64, res);
    }

    #[tokio::test]
    #[ignore]
    async fn test_average_cpu_load_of_shutdown_instance_is_zero() {
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;
        let instance_id = "i-03e0b3b1246001382";
        let res = inventory.get_average_cpu(instance_id).await.unwrap();
        assert_eq!(0 as f64, res);
    }

    #[tokio::test]
    #[ignore]
    async fn returns_the_right_number_of_volumes() {
        let inventory: AwsInventory = AwsInventory::new("eu-west-1").await;
        let filtertags: Vec<String> = Vec::new();
        let res = inventory.list_volumes(&filtertags).await.unwrap();
        assert_eq!(4, res.len());
    }
}
