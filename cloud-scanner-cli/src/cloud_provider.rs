//! A module to abstract a service to list resources of a cloud account.
use crate::model::Inventory;
use anyhow::Result;
use async_trait::async_trait;

/// A trait that you should implement to support vendor-specific inventory of cloud resources.
#[async_trait]
pub trait Inventoriable {
    /// Returns an inventory of cloud resources
    async fn list_resources(
        &self,
        tags: &[String],
        include_block_storage: bool,
    ) -> Result<Inventory>;
}
