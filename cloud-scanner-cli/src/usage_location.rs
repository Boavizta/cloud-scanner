//! The location where cloud resources are running.
use csv::ReaderBuilder;
use log::error;
use once_cell::sync::Lazy;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;
use thiserror::Error;

// Use of static to load the region-country map once
static REGION_COUNTRY_MAP: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let map = load_region_country_map().unwrap_or_default();
    Mutex::new(map)
});

#[derive(Error, Debug)]
pub enum RegionError {
    #[error("Unsupported region ({0})")]
    UnsupportedRegion(String),
}

///  The location where cloud resources are running.
///
/// TODO! the usage location should be abstracted and vendor specific implementation should be part of the cloud_provider model (region names are tied to a specific cloud provider)
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct UsageLocation {
    /// The AWS region (like eu-west-1)
    pub aws_region: String,
    /// The 3-letters ISO country code corresponding to the country of the aws_region
    pub iso_country_code: String,
}

impl TryFrom<&str> for UsageLocation {
    fn try_from(aws_region: &str) -> Result<Self, RegionError> {
        let cc = get_country_from_aws_region(aws_region)?;
        Ok(UsageLocation {
            aws_region: String::from(aws_region),
            iso_country_code: cc,
        })
    }
    type Error = RegionError;
}

/// Load the region-country map from a CSV file
fn load_region_country_map() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let csv_content = include_str!("../csv/cloud_providers_regions.csv");

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_content.as_bytes());

    let mut region_country_map = HashMap::new();

    for result in reader.records() {
        let record = result?;
        let region = &record[1]; // AWS region
        let country_code = &record[2]; // country code
        region_country_map.insert(region.to_string(), country_code.to_string());
    }

    Ok(region_country_map)
}

/// Converts AWS region as String into an ISO country code, returns FRA if not found
fn get_country_from_aws_region(aws_region: &str) -> Result<String, RegionError> {
    let map = REGION_COUNTRY_MAP.lock().unwrap();
    match map.get(aws_region) {
        Some(country_code) => Ok(country_code.to_string()),
        None => {
            error!(
                "Unsupported region: unable to match aws region [{}] to country code",
                aws_region
            );
            return Err(RegionError::UnsupportedRegion(String::from(aws_region)));
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    use super::UsageLocation;

    #[test]
    fn test_get_country_code_for_supported_aws_region() {
        let location = UsageLocation::try_from("eu-west-1").unwrap();
        assert_eq!("IRL", location.iso_country_code);

        let location = UsageLocation::try_from("eu-west-2").unwrap();
        assert_eq!("GBR", location.iso_country_code);

        let location = UsageLocation::try_from("eu-west-3").unwrap();
        assert_eq!("FRA", location.iso_country_code);
    }

    #[test]
    fn test_get_country_code_of_unsupported_aws_region_returns_error() {
        // this one is not supported
        let res = UsageLocation::try_from("us-gov-east-1");
        assert!(res.is_err());

        // this one is not supported
        let res = UsageLocation::try_from("whatever");
        assert!(res.is_err());

        let res = UsageLocation::try_from("");
        assert!(res.is_err());
    }
}
