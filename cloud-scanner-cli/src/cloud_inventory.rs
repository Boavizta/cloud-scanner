//! A module to allow inventory of cloud resources
//!  It define s a CoudInventory trait that you should use when implementing vendor specific inventory .

use crate::cloud_resource::*;
use anyhow::{Context, Result};
use async_trait::async_trait;

/// A cloud inventory trait that vendor specific cloud inventory should implement
///
/// Implementing this trait when creating a new CloudInventory (for example to support another cloud provider) esures that ckloud-scanner will be able to use it.
#[async_trait]
pub trait CloudInventory {
    /// Returns a list list of cloud resources
    async fn list_resources(&self, tags: Vec<String>) -> Result<Vec<CloudResource>>;
}
