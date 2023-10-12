//! A module to allow inventory of cloud resources.
//!
//!  It define s a CloudInventory trait that you should use when implementing vendor specific inventory .

use crate::cloud_resource::*;
use anyhow::Result;
use async_trait::async_trait;

/// A that you should implement to support vendor-specific inventory of cloud resources.
///
/// For example, you may want to implement it to ensure that cloud-scanner is able to support an additional cloud provider.
#[async_trait]
pub trait CloudInventory {
    /// Returns a list list of cloud resources
    async fn list_resources(
        &self,
        tags: &[String],
        include_block_storage: bool,
    ) -> Result<Vec<CloudResource>>;
}
