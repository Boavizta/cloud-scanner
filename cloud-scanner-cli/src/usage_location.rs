//! The location where cloud resources are running.

use isocountry::CountryCode;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

impl From<&str> for UsageLocation {
    fn from(aws_region: &str) -> Self {
        UsageLocation {
            aws_region: String::from(aws_region),
            iso_country_code: get_country_from_aws_region(aws_region).alpha3().to_owned(),
        }
    }
}

/// Converts AWS region as String into an ISO country code, returns FRA if not found
///
/// TODO! : do not convert to FRA by default, should rather fail explicitly if region is not found.
fn get_country_from_aws_region(aws_region: &str) -> CountryCode {
    let cc: CountryCode = match aws_region {
        "eu-central-1" => CountryCode::DEU,
        "eu-east-1" => CountryCode::IRL,
        "eu-north-1" => CountryCode::SWE,
        "eu-south-1" => CountryCode::ITA,
        "eu-west-1" => CountryCode::IRL,
        "eu-west-2" => CountryCode::GBR,
        "eu-west-3" => CountryCode::FRA,
        "us-east-1" => CountryCode::USA,
        "us-east-2" => CountryCode::USA,
        "us-west-1" => CountryCode::USA,
        "us-west-2" => CountryCode::USA,
        _ => {
            error!("Unsupported region: Unable to match aws region [{}] to country code, defaulting to FRA !", aws_region);
            CountryCode::FRA
        }
    };
    cc
}

#[cfg(test)]
mod tests {
    //use super::*;
    use super::UsageLocation;

    #[test]
    fn test_get_country_code_for_supported_aws_region() {
        let location = UsageLocation::from("eu-west-1");
        assert_eq!("IRL", location.iso_country_code);

        let location = UsageLocation::from("eu-west-2");
        assert_eq!("GBR", location.iso_country_code);

        let location = UsageLocation::from("eu-west-3");
        assert_eq!("FRA", location.iso_country_code);
    }

    #[test]
    fn test_get_country_code_of_unsupported_aws_region_returns_fra() {
        let location = UsageLocation::from("ap-south-1");
        assert_eq!("FRA", location.iso_country_code);

        let location = UsageLocation::from("");
        assert_eq!("FRA", location.iso_country_code);
    }
}
