use crate::UsageLocation;
//use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

///  A cloud resource (could be an instance, function or any other resource)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CloudResource {
    pub id: String,
    pub location: UsageLocation,
    pub resource_type: String,
    pub usage: Option<CloudResourceUsage>,
    pub tags: HashMap<String, CloudResourceTag>,
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

/// A tag (just a mandatory key + optional value)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CloudResourceTag {
    pub key: String,
    pub value: Option<String>,
}

impl TryFrom<String> for CloudResourceTag {
    type Error = &'static str;

    fn try_from(key_value: String) -> Result<Self, Self::Error> {
        let t: Vec<&str> = key_value.split("=").collect();
        if t.len() < 1 {
            Err("Cannot split the tag")
        } else {
            let key = t.get(0).unwrap().to_string();
            if let Some(val) = t.get(1) {
                Ok(CloudResourceTag {
                    key,
                    value: Some(val.to_string()),
                })
            } else {
                Ok(CloudResourceTag { key, value: None })
            }
        }
    }
}

impl CloudResource {
    /// Returns true it _all_ the tags passed in argument are defined and have the same values on the cloud resource
    fn has_matching_tagmap(&self, tags: &HashMap<String, CloudResourceTag>) -> bool {
        tags.iter()
            .all(|(filter_key, filter_tag)| self.tags.get(filter_key) == Some(filter_tag))
    }

    pub fn has_matching_tags(&self, tags: &[String]) -> bool {
        let mut filter = HashMap::new();
        tags.into_iter().for_each(|f| {
            let res = CloudResourceTag::try_from(f.to_owned());
            if res.is_ok() {
                let crt = res.unwrap();
                filter.insert(crt.key.clone(), crt.clone());
            } else {
                error!("Skipped filter");
            }
        });
        self.has_matching_tagmap(&filter)
    }
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
            tags: HashMap::new(),
        };

        assert_eq!("CloudResource { id: \"inst-1\", location: UsageLocation { aws_region: \"eu-west-1\", iso_country_code: \"IRL\" }, resource_type: \"t2.fictive\", usage: None, tags: {} }", format!("{:?}", instance1));
    }

    #[test]
    pub fn a_cloud_resource_without_usage_data_is_allowed() {
        let instance1: CloudResource = CloudResource {
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-1"),
            resource_type: "t2.fictive".to_string(),
            usage: None,
            tags: HashMap::new(),
        };
        assert_eq!(None, instance1.usage);
    }
    #[test]
    pub fn parse_tags() {
        let tag_string = "name1=val1".to_string();
        let res = CloudResourceTag::try_from(tag_string).unwrap();
        assert_eq!(res.key, "name1", "Wrong key");
        assert_eq!(res.value.unwrap(), "val1", "Wrong value");

        let tag_string = "name1".to_string();
        let res = CloudResourceTag::try_from(tag_string).unwrap();
        assert_eq!(res.key, "name1", "Wrong key");
        assert_eq!(res.value, None, "Wrong value");
    }

    #[test]
    pub fn match_tags() {
        let mut filtertags = HashMap::new();
        filtertags.insert(
            "Name".to_string(),
            CloudResourceTag {
                key: "Name".to_string(),
                value: Some("App1".to_string()),
            },
        );

        let instance1tags = filtertags.clone();
        let instance1: CloudResource = CloudResource {
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-1"),
            resource_type: "t2.fictive".to_string(),
            usage: None,
            tags: instance1tags,
        };
        assert_eq!(
            true,
            instance1.has_matching_tagmap(&filtertags),
            "Tags should match"
        );

        let mut other_name_tag = filtertags.clone();
        // Changing the content of Name tag
        other_name_tag.insert(
            "Name".to_string(),
            CloudResourceTag {
                key: "Name".to_string(),
                value: Some("OtherApp".to_string()),
            },
        );
        assert_eq!(
            false,
            instance1.has_matching_tagmap(&other_name_tag),
            "Tags should not match"
        );

        let mut more_tags = filtertags.clone();
        // Adding an extra tag that is not on the instance
        more_tags.insert(
            "Env".to_string(),
            CloudResourceTag {
                key: "Env".to_string(),
                value: Some("PROD".to_string()),
            },
        );
        assert_eq!(
            false,
            instance1.has_matching_tagmap(&more_tags),
            "Tags should not match"
        );

        let mut tag_without_val = filtertags.clone();
        // Adding an extra tag that is not on the instance
        tag_without_val.insert(
            "Name".to_string(),
            CloudResourceTag {
                key: "Name".to_string(),
                value: None,
            },
        );
        assert_eq!(
            false,
            instance1.has_matching_tagmap(&tag_without_val),
            "Tags should not match"
        );

        // Trying an empty filter
        let empty_filter = HashMap::new();
        assert_eq!(
            true,
            instance1.has_matching_tagmap(&empty_filter),
            "Tags should not match"
        );
    }
}
