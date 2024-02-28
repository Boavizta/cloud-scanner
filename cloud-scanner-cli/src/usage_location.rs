//! The location where cloud resources are running.

use isocountry::CountryCode;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
            iso_country_code: cc.alpha3().to_owned(),
        })
    }
    type Error = RegionError;
}

/// Converts AWS region as String into an ISO country code, returns FRA if not found
///
/// TODO! : do not convert to FRA by default, should rather fail explicitly if region is not found.
fn get_country_from_aws_region(aws_region: &str) -> Result<CountryCode, RegionError> {
    let cc: CountryCode = match aws_region {
        "af-south-1" => CountryCode::ZAF,
        "ap-east-1" => CountryCode::HKG,
        "ap-northeast-1" => CountryCode::JPN,
        "ap-northeast-2" => CountryCode::KOR,
        "ap-northeast-3" => CountryCode::JPN,
        "ap-south-1" => CountryCode::IND,
        "ap-south-2" => CountryCode::IND,
        "ap-southeast-1" => CountryCode::SGP,
        "ap-southeast-2" => CountryCode::AUS,
        "ap-southeast-3" => CountryCode::IDN,
        "ap-southeast-4" => CountryCode::AUS,
        "ca-central-1" => CountryCode::CAN,
        "ca-west-1" => CountryCode::CAN,
        "cn-north-1" => CountryCode::CHN,
        "cn-northwest-1" => CountryCode::CHN,
        "eu-central-1" => CountryCode::DEU,
        "eu-central-2" => CountryCode::CHE,
        "eu-east-1" => CountryCode::IRL,
        "eu-north-1" => CountryCode::SWE,
        "eu-south-1" => CountryCode::ITA,
        "eu-south-2" => CountryCode::ESP,
        "eu-west-1" => CountryCode::IRL,
        "eu-west-2" => CountryCode::GBR,
        "eu-west-3" => CountryCode::FRA,
        "il-central-1" => CountryCode::ISR,
        "me-central-1" => CountryCode::ARE,
        "me-south-1" => CountryCode::BHR,
        "sa-east-1" => CountryCode::BRA,
        "us-east-1" => CountryCode::USA,
        "us-east-2" => CountryCode::USA,
        "us-west-1" => CountryCode::USA,
        "us-west-2" => CountryCode::USA,
        _ => {
            error!(
                "Unsupported region: unable to match aws region [{}] to country code",
                aws_region
            );
            return Err(RegionError::UnsupportedRegion(String::from(aws_region)));
        }
    };
    Ok(cc)
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
