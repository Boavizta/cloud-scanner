use serde_derive::{Deserialize, Serialize};

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
