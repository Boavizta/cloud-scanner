use crate::UsageLocation;
use serde::{Deserialize, Serialize};
use std::fmt;

///  A cloud resource (could be an instance, function or any other resource)
#[derive(Clone, Debug, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CloudResourceUsage {
    pub average_cpu_load: f64,
    pub usage_duration_seconds: u32,
}

/// Define how to allocate the manufacturing impacts of a resource
pub enum ManufacturingAllocation {
    /// Amortized allocation (prorata of usage duration)
    LinearAllocation,
    /// Total (Full impact regardless of usage duration)
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

    #[test]
    pub fn a_cloud_resource_without_usage_data_is_allowed() {
        let instance1: CloudResource = CloudResource {
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-1"),
            resource_type: "t2.fictive".to_string(),
            usage: None,
        };
        assert_eq!(None, instance1.usage);
    }
}
