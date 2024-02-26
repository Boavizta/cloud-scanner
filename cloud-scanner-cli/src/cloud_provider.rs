//! A module to allow inventory of resources of cloud account.
//!
//!  It defines an Inventoriable trait that you should use when implementing vendor specific inventory .

use crate::model::Inventory;
use anyhow::Result;
use async_trait::async_trait;

/// A trait that you should implement to support vendor-specific inventory of cloud resources.
///
/// Implement it to support an additional cloud provider.
#[async_trait]
pub trait Inventoriable {
    /// Returns an inventory of cloud resources
    async fn list_resources(
        &self,
        tags: &[String],
        include_block_storage: bool,
    ) -> Result<Inventory>;
}
