//!  Business Entities of cloud Scanner
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use crate::impact_provider::CloudResourceWithImpacts;
use crate::usage_location::UsageLocation;

///  Statistics about program execution
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionStatistics {
    pub inventory_duration: Duration,
    pub impact_estimation_duration: Duration,
    pub total_duration: Duration,
}

impl fmt::Display for ExecutionStatistics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Inventory: a list of resources
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub resources: Vec<CloudResource>,
    pub execution_statistics: Option<ExecutionStatistics>,
}

/// An estimated inventory: impacting resources with their estimated impacts
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedInventory {
    pub impacting_resources: Vec<CloudResourceWithImpacts>,
    pub execution_statistics: Option<ExecutionStatistics>,
}

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

impl CloudResource {
    /// Convert tags into a format supported by prometheus metrics label (like `tag_key_1:tag_value_1;tag_key_2:tag_value_2;`)
    pub fn tags_as_metric_label_value(&self) -> String {
        let mut res = "".to_string();
        for tag in self.tags.iter() {
            let val = tag.value.clone().unwrap_or("".parse().unwrap());
            res.push_str(&tag.key);
            res.push(':');
            res.push_str(&val);
            res.push(';');
        }
        res
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
        attached_instances: Option<Vec<StorageAttachment>>,
    },
    ObjectStorage,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct InstanceUsage {
    pub average_cpu_load: f64,
    pub usage_duration_seconds: u32,
    pub state: InstanceState,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum InstanceState {
    #[default]
    Running,
    Stopped,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct StorageUsage {
    pub size_gb: i32,
    pub usage_duration_seconds: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct StorageAttachment {
    pub instance_id: String,
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

#[cfg(test)]
mod tests {
    use crate::model::{CloudProvider, CloudResource, CloudResourceTag, ResourceDetails};
    use crate::usage_location::UsageLocation;
    use std::collections::HashMap;

    #[test]
    pub fn a_cloud_resource_can_be_displayed() {
        let instance1: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::try_from("eu-west-1").unwrap(),
            resource_details: ResourceDetails::Instance {
                instance_type: "t2.fictive".to_string(),
                usage: None,
            },
            tags: Vec::new(),
        };

        assert_eq!("CloudResource { provider: AWS, id: \"inst-1\", location: UsageLocation { aws_region: \"eu-west-1\", iso_country_code: \"IRL\" }, resource_details: Instance { instance_type: \"t2.fictive\", usage: None }, tags: [] }", format!("{:?}", instance1));
    }

    #[test]
    pub fn parse_tag() {
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
    pub fn parse_tags_with_spaces() {
        let tag_string = "name 1=val 1".to_string();
        let res = CloudResourceTag::try_from(tag_string).unwrap();
        assert_eq!(res.key, "name 1", "Wrong key");
        assert_eq!(res.value.unwrap(), "val 1", "Wrong value");
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
            location: UsageLocation::try_from("eu-west-1").unwrap(),
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
    #[test]
    pub fn format_tags_as_metric_label() {
        let tag1 = CloudResourceTag {
            key: "name1".to_string(),
            value: Some("value1".to_string()),
        };
        let tag2 = CloudResourceTag {
            key: "name2".to_string(),
            value: Some("value2".to_string()),
        };

        let cr = CloudResource {
            provider: CloudProvider::AWS,
            id: "123".to_string(),
            location: UsageLocation {
                aws_region: "eu-west-3".to_string(),
                iso_country_code: "FR".to_string(),
            },
            resource_details: ResourceDetails::ObjectStorage,
            tags: vec![tag1, tag2],
        };

        let tag_label_value = cr.tags_as_metric_label_value();

        assert_eq!(
            "name1:value1;name2:value2;", tag_label_value,
            "could not convert tags to metric label values"
        );
    }
}
