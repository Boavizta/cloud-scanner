//! cloud scanner entities
use crate::UsageLocation;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

/// Describes an instance with its impacts
#[derive(Debug, Serialize, Deserialize)]
pub struct AwsInstanceWithImpacts {
    pub instance_id: String,
    pub instance_type: String,
    pub usage_data: boavizta_api_sdk::models::UsageCloud,
    pub impacts: Option<serde_json::Value>,
}

/// The aggregated impacts and meta data about the scan results
#[derive(Debug, Default)]
pub struct ScanResultSummary {
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

///  A cloud resource (instance, function)
#[derive(Debug)]
pub struct CloudResource {
    pub id: String,
    pub location: UsageLocation,
    pub resource_type: String,
    pub usage: Option<CloudResourceUsage>,
}

impl fmt::Display for CloudResource {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:?}", self)
    }
}

/// Usage of a cloud resource
#[derive(Debug, Default)]
pub struct CloudResourceUsage {
    pub average_cpu_load: f32,
    pub usage_duration_seconds: u32,
}

pub enum ManufacturingAllocation {
    LinearAllocation,
    TotalAllocation,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn a_cloud_resource_can_be_displayed() {
        let instance1: CloudResource = CloudResource {
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-1"),
            resource_type: "t2.fictive".to_string(),
            usage: None,
        };

        assert_eq!("CloudResource { id: \"inst-1\", location: UsageLocation { aws_region: \"eu-west-1\", iso_country_code: \"IRL\" }, resource_type: \"t2.fictive\", usage: None }", format!("{:?}", instance1));
    }
}
