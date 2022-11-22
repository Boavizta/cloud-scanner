//! A module to allow retrieving impacts of cloud resources
//!  It define s a Impact procvider  trait that you should use when implementing vendor specific inventory .
///
/// The model of impacts goes here (scan result summary ?)
///
/// The model of allocation should be internal to boa API
///
use crate::cloud_resource::*;
use anyhow::{Context, Result};
use async_trait::async_trait;

/// A ImpactProvider trait that yu should implement for a specific impact API
///
/// Implementing this trait when creating a new ImpactProvider (for example to support a different version of Boavizata db) ensures that cloud-scanner will be able to use it.
#[async_trait]
pub trait ImpactProvider {
    /// Returns a list list of CloudImpacts
    async fn get_impacts(
        &self,
        resources: Vec<CloudResource>,
    ) -> Result<Vec<CloudResourceWithImpacts>>;
}

#[derive(Debug)]
pub struct CloudResourceWithImpacts {
    cloud_resource: CloudResource,
    resource_impacts: Impacts,
}

/// Impacts of an individual resource
#[derive(Debug, Default)]
pub struct Impacts {
    pub adp_manufacture_kgsbeq: f64,
    pub adp_use_kgsbeq: f64,
    pub pe_manufacture_megajoules: f64,
    pub pe_use_megajoules: f64,
    pub gwp_manufacture_kgco2eq: f64,
    pub gwp_use_kgco2eq: f64,
}

/// The aggregated impacts and meta data about the scan results
#[derive(Debug, Default)]
pub struct ImpactsSummary {
    pub number_of_instances_total: u32,
    pub number_of_instances_assessed: u32,
    pub number_of_instances_not_assessed: u32,
    pub duration_of_use_hours: f64,
    pub adp_manufacture_kgsbeq: f64,
    pub adp_use_kgsbeq: f64,
    pub pe_manufacture_megajoules: f64,
    pub pe_use_megajoules: f64,
    pub gwp_manufacture_kgco2eq: f64,
    pub gwp_use_kgco2eq: f64,
    pub aws_region: String,
    pub country: String,
}
