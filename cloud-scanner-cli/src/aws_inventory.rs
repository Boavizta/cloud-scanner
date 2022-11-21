//! A module to perform inventory of cloud resources

use crate::cloud_inventory::CloudInventory;
use crate::cloud_resource::*;
use crate::usage_location::*;
use anyhow::{Context, Result};
use aws_sdk_ec2::Region;
use aws_sdk_ec2::model::Instance;

///  An inventory of AWS resources
pub struct AwsInventory {
    aws_region: String,
    ec2_client: aws_sdk_ec2::Client,
    cloudwatch_client: aws_sdk_cloudwatch::Client,
}

impl AwsInventory {
    pub async fn new(aws_region: &str) -> Self {
        let shared_config = Self::load_aws_config(aws_region).await;
        AwsInventory {
            aws_region: String::from(aws_region),
            ec2_client: aws_sdk_ec2::Client::new(&shared_config),
            cloudwatch_client: aws_sdk_cloudwatch::Client::new(&shared_config),
        }
    }

    // /// Initialize the  AWS sdk
    // pub async fn init(&mut self) {
    //     let shared_config = Self::load_aws_config(&self.aws_region).await;
    //     self.ec2_client = Some(aws_sdk_ec2::Client::new(&shared_config));
    //     self.cloudwatch_client = Some(aws_sdk_cloudwatch::Client::new(&shared_config));
    // }

    /// Initialize a sdk config with default credentials and  region passed as argument
    /// If region is empty, uses a default region (but will return no error even if the region is invalid)
    async fn load_aws_config(aws_region: &str) -> aws_types::sdk_config::SdkConfig {
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
    async fn list_instances(& self, tags: &Vec<String>) -> Result<Vec<Instance>> {
        warn!("Warning: filtering on tag not implemented {:?}", tags);

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


}

impl CloudInventory for AwsInventory {
    /// list resources
    fn list_resources(self: &Self) -> Result<Vec<CloudResource>> {
        let mut res: Vec<CloudResource> = Vec::new();

        Ok(res)
    }
}


/// Cannot work, need more metadata (region and usage)
// /// Not sure this is the right idea as usage will always be None
// impl From<aws_sdk_ec2::model::Instance> for CloudResource {
//     fn from(instance: aws_sdk_ec2::model::Instance) -> Self {
//         CloudResource {
//             id: instance.instance_id(),
//             location: instance
//             resource_type: instance.instance_type(),
//             usage: None,
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;
    //use anyhow::{Context, Result};

    #[tokio::test]
    async fn test_list_resources() {
        let inventory: AwsInventory = AwsInventory ::new("eu-west-1").await;
        let res: Vec<CloudResource> = inventory.list_resources().expect("Failed to list");

        assert_eq!(0, res.len());
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
}
