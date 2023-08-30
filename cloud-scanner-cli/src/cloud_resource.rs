use crate::UsageLocation;
//use anyhow::{Context, Result};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

///  A cloud resource (could be an instance, function or any other resource)
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct CloudResource {
    pub provider: CloudProvider,
    pub id: String,
    pub location: UsageLocation,
    pub resource_details: ResourceDetails,
    pub tags: Vec<CloudResourceTag>,
}

impl fmt::Display for CloudResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum CloudProvider {
    AWS,
    OVH,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum ResourceDetails {
    Instance {
        instance_type: String,
        usage: Option<InstanceUsage>,
    },
    BlockStorage {
        storage_type: String,
        usage: Option<StorageUsage>,
    },
    ObjectStorage,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct InstanceUsage {
    pub average_cpu_load: f64,
    pub usage_duration_seconds: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct StorageUsage {
    pub size_gb: f64,
    pub usage_duration_seconds: u32,
}

/// A tag (just a mandatory key + optional value)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CloudResourceTag {
    pub key: String,
    pub value: Option<String>,
}

///  Parse a  tag kay and value from a String (coming from CLI or query strings) .
///  Tags are expected to be int the form "Tag name=Tag value"
impl TryFrom<String> for CloudResourceTag {
    type Error = &'static str;

    fn try_from(key_value: String) -> Result<Self, Self::Error> {
        let t: Vec<&str> = key_value.split('=').collect();
        if t.is_empty() {
            Err("Cannot split the tag name from value (missing equal sign?)")
        } else {
            let key = t.first().unwrap().to_string();
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
        tags.iter().all(|(filter_key, filter_tag)| {
            let tag_map: HashMap<String, Option<String>> = vec_to_map(self.tags.clone());
            tag_map.get(filter_key) == Some(&filter_tag.value)
        })
    }

    pub fn has_matching_tags(&self, filter_tags: &[String]) -> bool {
        let mut filter = HashMap::new();
        filter_tags.iter().for_each(|f| {
            let res = CloudResourceTag::try_from(f.to_owned());
            if let Ok(crt) = res {
                filter.insert(crt.key.clone(), crt);
            } else {
                error!("Skipped filter");
            }
        });
        self.has_matching_tagmap(&filter)
    }
}

pub fn vec_to_map(tagv: Vec<CloudResourceTag>) -> HashMap<String, Option<String>> {
    let mut tagh: HashMap<String, Option<String>> = HashMap::new();
    tagv.iter().for_each(|t| {
        tagh.insert(t.key.clone(), t.value.clone());
    });
    tagh
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
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-1"),
            resource_details: ResourceDetails::Instance {
                instance_type: "t2.fictive".to_string(),
                usage: None,
            },
            tags: Vec::new(),
        };

        assert_eq!("CloudResource { provider: AWS, id: \"inst-1\", location: UsageLocation { aws_region: \"eu-west-1\", iso_country_code: \"IRL\" }, resource_details: Instance { instance_type: \"t2.fictive\", usage: None }, tags: [] }", format!("{:?}", instance1));
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

        let mut instance1tags: Vec<CloudResourceTag> = Vec::new();
        instance1tags.push(CloudResourceTag {
            key: "Name".to_string(),
            value: Some("App1".to_string()),
        });

        let instance1: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-1"),
            resource_details: ResourceDetails::Instance {
                instance_type: "t2.fictive".to_string(),
                usage: None,
            },
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
            "Tag without a value should not match"
        );

        // Trying an empty filter
        let empty_filter = HashMap::new();
        assert_eq!(
            true,
            instance1.has_matching_tagmap(&empty_filter),
            "Tags should match"
        );

        // When the name of tag used to filter is an empty string....
        let mut empty_tag_name_in_filter = HashMap::new();

        let empty_key: String = "".to_string();
        empty_tag_name_in_filter.insert(
            empty_key.clone(),
            CloudResourceTag {
                key: empty_key,
                value: Some("whatever".to_string()),
            },
        );
        assert_eq!(
            true,
            instance1.has_matching_tagmap(&empty_filter),
            "Tags should match (i.e. we should ignore this invalid filter"
        );
    }
}
