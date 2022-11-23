//! A module to allow retrieving impacts of cloud resources
//!  It defines an Impact provider  trait that you should use when implementing vendor specific inventory .
///
/// The model of impacts goes here (scan result summary ?)
///
/// The model of allocation should be internal to boa API
///
use crate::cloud_resource::*;
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// A ImpactProvider trait that yu should implement for a specific impact API
///
/// Implementing this trait when creating a new ImpactProvider (for example to support a different version of Boavizata db) ensures that cloud-scanner will be able to use it.
#[async_trait]
pub trait ImpactProvider {
    /// Returns a list list of CloudImpacts.
    /// The usage_duration_hours parameters allow to retrieve the impacts for a given duration (i.e. project impacts for a specific duration).
    async fn get_impacts(
        &self,
        resources: Vec<CloudResource>,
        usage_duration_hours: &f32,
    ) -> Result<Vec<CloudResourceWithImpacts>>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CloudResourceWithImpacts {
    pub cloud_resource: CloudResource,
    /// The impacts
    pub resource_impacts: Option<ResourceImpacts>,
    /// The duration for which impacts are calculated
    pub impacts_duration_hours: f32,
}

/// Impacts of an individual resource
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResourceImpacts {
    pub adp_manufacture_kgsbeq: f64,
    pub adp_use_kgsbeq: f64,
    pub pe_manufacture_megajoules: f64,
    pub pe_use_megajoules: f64,
    pub gwp_manufacture_kgco2eq: f64,
    pub gwp_use_kgco2eq: f64,
}

/// The aggregated impacts and meta data about the scan results
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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

impl ImpactsSummary {
    /// Returns a Summary of impacts for a list of Cloud Resources
    pub fn new(
        aws_region: String,
        country: String,
        resources: Vec<CloudResourceWithImpacts>,
    ) -> Self {
        let mut summary = ImpactsSummary {
            number_of_instances_total: u32::try_from(resources.len()).unwrap(),
            aws_region,
            country,
            ..Default::default()
        };

        for resource in resources {
            // Only consider the instances for which we have impact data
            if let Some(impacts) = resource.resource_impacts {
                summary.number_of_instances_assessed += 1;
                summary.adp_manufacture_kgsbeq += impacts.adp_manufacture_kgsbeq;
                summary.adp_use_kgsbeq += impacts.adp_use_kgsbeq;
                summary.pe_manufacture_megajoules += impacts.pe_manufacture_megajoules;
                summary.pe_use_megajoules += impacts.pe_use_megajoules;
                summary.gwp_manufacture_kgco2eq += impacts.gwp_manufacture_kgco2eq;
                summary.gwp_use_kgco2eq += impacts.gwp_use_kgco2eq;
            } else {
                // Resource was not counted due to no impact
                debug!("Skipped counting resource: {:#?} while building summary because it has no impact data", resource);
                summary.number_of_instances_not_assessed += 1;
            }
        }
        summary
    }
}
