//! A module to abstract the service used to retrieve impacts of cloud resources.
use crate::model::{CloudResource, EstimatedInventory, Inventory};
use anyhow::Result;
use async_trait::async_trait;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A ImpactProvider trait to implement for a specific impact API/Referential.
#[async_trait]
pub trait ImpactProvider {
    /// Returns a list of CloudImpacts.
    /// usage_duration_hours allow to retrieve the impacts for a given duration (i.e. project impacts for a specific duration).
    async fn get_impacts(
        &self,
        inventory: Inventory,
        usage_duration_hours: &f32,
        verbose: bool,
    ) -> Result<EstimatedInventory>;
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct CloudResourceWithImpacts {
    pub cloud_resource: CloudResource,
    /// The impacts
    pub impacts_values: Option<ImpactsValues>,
    /// The duration for which impacts are calculated
    pub impacts_duration_hours: f32,
}

// TODO: shouldn't theses fields be optional ?
/// Impacts of an individual resource
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ImpactsValues {
    pub adp_manufacture_kgsbeq: f64,
    pub adp_use_kgsbeq: f64,
    pub pe_manufacture_megajoules: f64,
    pub pe_use_megajoules: f64,
    pub gwp_manufacture_kgco2eq: f64,
    pub gwp_use_kgco2eq: f64,
    pub raw_data: Option<serde_json::Value>,
}

/// The aggregated impacts and metadata about the scan results
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ImpactsSummary {
    pub number_of_resources_total: usize,
    pub number_of_resources_assessed: usize,
    pub number_of_resources_not_assessed: usize,
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
        resources_with_impacts: EstimatedInventory,
        duration_of_use_hours: f64,
    ) -> Self {
        let resources = resources_with_impacts.impacting_resources;

        let mut summary = ImpactsSummary {
            number_of_resources_total: resources.len(),
            number_of_resources_assessed: 0,
            number_of_resources_not_assessed: 0,
            aws_region,
            country,
            duration_of_use_hours,
            adp_manufacture_kgsbeq: 0.0,
            adp_use_kgsbeq: 0.0,
            pe_manufacture_megajoules: 0.0,
            pe_use_megajoules: 0.0,
            gwp_manufacture_kgco2eq: 0.0,
            gwp_use_kgco2eq: 0.0,
        };

        for resource in resources {
            // Only consider the instances for which we have impact data
            if let Some(impacts) = resource.impacts_values {
                summary.number_of_resources_assessed += 1;
                summary.adp_manufacture_kgsbeq += impacts.adp_manufacture_kgsbeq;
                summary.adp_use_kgsbeq += impacts.adp_use_kgsbeq;
                summary.pe_manufacture_megajoules += impacts.pe_manufacture_megajoules;
                summary.pe_use_megajoules += impacts.pe_use_megajoules;
                summary.gwp_manufacture_kgco2eq += impacts.gwp_manufacture_kgco2eq;
                summary.gwp_use_kgco2eq += impacts.gwp_use_kgco2eq;
            } else {
                // Resource was not counted due to no impact
                debug!("Skipped counting resource: {:#?} while building summary because it has no impact data", resource);
                summary.number_of_resources_not_assessed += 1;
            }
        }
        summary
    }
}
