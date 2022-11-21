//! A module to perform inventory of cloud resources

use crate::cloud_resource::*;
use anyhow::{Context, Result};

/// Define a cloud inventory that can be used to list resources and their usage
pub trait CloudInventory {
    /// Returns a list list of cloud resources
    fn list_resources(&self) -> Result<Vec<CloudResource>>;
}
