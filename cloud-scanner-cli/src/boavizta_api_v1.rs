//!  Provide access to Boavizta API cloud impacts functions
use crate::cloud_resource::*;
use crate::impact_provider::{CloudResourceWithImpacts, ImpactProvider, ResourceImpacts};
use anyhow::Result;
/// Get impacts of cloud resources through Boavizta API
use boavizta_api_sdk::apis::cloud_api;
use boavizta_api_sdk::apis::component_api;
use boavizta_api_sdk::apis::configuration;

use boavizta_api_sdk::models::{Cloud, Disk, UsageCloud};

/// Access data of Boavizta API
pub struct BoaviztaApiV1 {
    configuration: boavizta_api_sdk::apis::configuration::Configuration,
}

impl BoaviztaApiV1 {
    pub fn new(api_url: &str) -> Self {
        let mut configuration = configuration::Configuration::new();
        configuration.base_path = api_url.to_string();
        BoaviztaApiV1 { configuration }
    }

    // Returns the raw impacts (json) of an instance from Boavizta API
    ///
    /// The manufacture impacts returned represent the entire lifecycle of instance (i.e. it is using the 'Allocation' TOTAL )
    async fn get_raws_impacts(
        &self,
        cr: CloudResource,
        usage_duration_hours: &f32,
    ) -> Option<serde_json::Value> {
        let resource_details = cr.resource_details;
        let verbose = Some(false);
        let criteria = vec!["gwp".to_owned(), "adp".to_owned(), "pe".to_owned()];

        match resource_details {
            ResourceDetails::Instance {
                instance_type,
                usage,
            } => {
                let mut usage_cloud: UsageCloud = UsageCloud::new();

                //usage_cloud.hours_life_time = Some(usage_duration_hours.to_owned());
                usage_cloud.usage_location = Some(cr.location.iso_country_code.to_owned());

                if let Some(instance_usage) = usage {
                    usage_cloud.time_workload = Some(instance_usage.average_cpu_load as f32);
                }

                let mut cloud: Cloud = Cloud::new();
                cloud.provider = Some(String::from("aws"));
                cloud.instance_type = Some(instance_type.clone());
                cloud.usage = Some(Box::new(usage_cloud));

                let res = cloud_api::instance_cloud_impact_v1_cloud_instance_post(
                    &self.configuration,
                    verbose,
                    Some(usage_duration_hours.to_owned()),
                    Some(criteria),
                    Some(cloud),
                )
                .await;

                match res {
                    Ok(res) => Some(res),
                    Err(e) => {
                        warn!(
                            "Warning: Cannot get impacts from API for instance type {}: {}",
                            instance_type, e
                        );
                        None
                    }
                }
            }

            ResourceDetails::BlockStorage {
                storage_type,
                usage,
            } => {
                //let duration: f32 = usage.unwrap().usage_duration_seconds.into();

                let disk = Disk {
                    capacity: Some(usage.unwrap().size_gb),
                    units: Some(1),
                    usage: None,
                    r#type: None,
                    density: None,
                    manufacturer: None,
                    model: None,
                    layers: None,
                };

                let res = component_api::disk_impact_bottom_up_v1_component_hdd_post(
                    &self.configuration,
                    verbose,
                    None,
                    None,
                    Some(criteria),
                    Some(disk),
                )
                .await;

                match res {
                    Ok(res) => Some(res),
                    Err(e) => {
                        warn!(
                            "Warning: Cannot get storage impact from API for type {}: {}",
                            storage_type, e
                        );
                        None
                    }
                }
            }
            _ => {
                warn!("Warning: This type of cloud resource is not supported.");
                None
            }
        }
    }

    /// Get the impacts of a single CloudResource
    async fn get_resource_with_impacts(
        &self,
        resource: &CloudResource,
        usage_duration_hours: &f32,
    ) -> CloudResourceWithImpacts {
        let raw_impacts = self
            .get_raws_impacts(resource.clone(), usage_duration_hours)
            .await;
        boa_impacts_to_cloud_resource_with_impacts(resource, &raw_impacts, usage_duration_hours)
    }
}

#[async_trait]
impl ImpactProvider for BoaviztaApiV1 {
    /// Get cloud resources impacts from the Boavizta API
    /// The usage_duration_hours parameters allow to retrieve the impacts for a given duration.
    async fn get_impacts(
        &self,
        resources: Vec<CloudResource>,
        usage_duration_hours: &f32,
    ) -> Result<Vec<CloudResourceWithImpacts>> {
        let mut v: Vec<CloudResourceWithImpacts> = Vec::new();
        for resource in resources.iter() {
            let cri = self
                .get_resource_with_impacts(resource, usage_duration_hours)
                .await;
            v.push(cri.clone());
        }
        Ok(v)
    }
}

/// Convert raw results from boavizta API into model objects
pub fn boa_impacts_to_cloud_resource_with_impacts(
    cloud_resource: &CloudResource,
    raw_result: &Option<serde_json::Value>,
    impacts_duration_hours: &f32,
) -> CloudResourceWithImpacts {
    let resource_impacts: Option<ResourceImpacts>;
    if let Some(results) = raw_result {
        debug!("This cloud resource has impacts data: {}", results);

        let resource_details = cloud_resource.resource_details.clone();

        match resource_details {
            ResourceDetails::Instance {
                instance_type: _,
                usage: _,
            } => {
                resource_impacts = Some(ResourceImpacts {
                    adp_manufacture_kgsbeq: results["adp"]["embedded"]["value"].as_f64().unwrap(),
                    adp_use_kgsbeq: results["adp"]["use"]["value"].as_f64().unwrap(),
                    pe_manufacture_megajoules: results["pe"]["embedded"]["value"].as_f64().unwrap(),
                    pe_use_megajoules: results["pe"]["use"]["value"].as_f64().unwrap(),
                    gwp_manufacture_kgco2eq: results["gwp"]["embedded"]["value"].as_f64().unwrap(),
                    gwp_use_kgco2eq: results["gwp"]["use"]["value"].as_f64().unwrap(),
                });
            }
            ResourceDetails::BlockStorage {
                storage_type: _,
                usage: _,
            } => {
                // TODO: handle empty values differently
                resource_impacts = Some(ResourceImpacts {
                    adp_manufacture_kgsbeq: results["adp"]["embedded"]["value"].as_f64().unwrap(),
                    adp_use_kgsbeq: 0 as f64,
                    pe_manufacture_megajoules: results["pe"]["embedded"]["value"].as_f64().unwrap(),
                    pe_use_megajoules: 0 as f64,
                    gwp_manufacture_kgco2eq: results["gwp"]["embedded"]["value"].as_f64().unwrap(),
                    gwp_use_kgco2eq: 0 as f64,
                });
            }
            _ => {
                resource_impacts = None;
            }
        }
    } else {
        debug!(
            "Skipped resource: {:#?} while building impacts, it has no impact data",
            cloud_resource
        );
        resource_impacts = None;
    };
    CloudResourceWithImpacts {
        cloud_resource: cloud_resource.clone(),
        resource_impacts,
        impacts_duration_hours: impacts_duration_hours.to_owned(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::UsageLocation;
    use assert_json_diff::assert_json_include;

    // const TEST_API_URL: &str = "https://api.boavizta.org";
    // Test against local  version of Boavizta API
    // const TEST_API_URL: &str = "http:/localhost:5000";
    // Test against dev version of Boavizta API
    const TEST_API_URL: &str = "https://dev.api.boavizta.org";

    const DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR: &str =
        include_str!("../test-data/DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR.json");

    const DEFAULT_RAW_IMPACTS_OF_HDD_1HRS_FR: &str =
        include_str!("../test-data/DEFAULT_RAW_IMPACTS_OF_HDD_1HRS_FR.json");

    #[tokio::test]
    async fn retrieve_instance_types_through_sdk_works() {
        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let provider = Some("aws");

        let res = cloud_api::server_get_all_archetype_name_v1_cloud_instance_all_instances_get(
            &api.configuration,
            provider,
        )
        .await
        .unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn should_retrieve_raw_default_impacts_aws_fr() {
        let instance1: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    usage_duration_seconds: 3600,
                }),
            },
            tags: Vec::new(),
        };
        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let one_hour = 1.0 as f32;
        let res = api.get_raws_impacts(instance1, &one_hour).await.unwrap();

        let expected: serde_json::Value =
            serde_json::from_str(DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR).unwrap();
        assert_json_include!(actual: res, expected: expected);
    }

    #[tokio::test]
    async fn get_raw_impacts_of_a_hdd() {
        let hdd: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "disk-1".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_details: ResourceDetails::BlockStorage {
                storage_type: "gp2".to_string(),
                usage: Some(StorageUsage {
                    size_gb: 10000000,
                    usage_duration_seconds: 0,
                }),
            },
            tags: Vec::new(),
        };

        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let one_hour = 1.0 as f32;
        let res = api.get_raws_impacts(hdd, &one_hour).await.unwrap();

        let expected: serde_json::Value =
            serde_json::from_str(DEFAULT_RAW_IMPACTS_OF_HDD_1HRS_FR).unwrap();
        assert_json_include!(actual: res, expected: expected);
    }

    #[tokio::test]
    async fn returns_different_pe_impacts_for_different_cpu_load() {
        let instance1: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    usage_duration_seconds: 3600,
                }),
            },
            tags: Vec::new(),
        };

        let instance1_1percent = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-2".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 1.0,
                    usage_duration_seconds: 3600,
                }),
            },
            tags: Vec::new(),
        };

        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let one_hour = 1.0 as f32;

        let mut instances: Vec<CloudResource> = Vec::new();
        instances.push(instance1);
        instances.push(instance1_1percent);

        let res = api.get_impacts(instances, &one_hour).await.unwrap();

        let r0 = res[0].resource_impacts.clone().unwrap();
        let r1 = res[1].resource_impacts.clone().unwrap();
        assert_eq!(0.21321, r0.pe_use_megajoules);
        assert_eq!(0.08903, r1.pe_use_megajoules);
    }

    #[tokio::test]
    async fn should_retrieve_multiple_default_impacts_fr() {
        let instance1: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    usage_duration_seconds: 3600,
                }),
            },
            tags: Vec::new(),
        };

        let instance2: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-2".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    usage_duration_seconds: 3600,
                }),
            },
            tags: Vec::new(),
        };

        let instance3: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-3".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_details: ResourceDetails::Instance {
                instance_type: "type-not-in-boa".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    usage_duration_seconds: 3600,
                }),
            },
            tags: Vec::new(),
        };

        let mut instances: Vec<CloudResource> = Vec::new();
        instances.push(instance1);
        instances.push(instance2);
        instances.push(instance3);
        let one_hour = 1.0 as f32;

        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let res = api.get_impacts(instances, &one_hour).await.unwrap();

        assert_eq!(3, res.len());
        assert_eq!(res[0].cloud_resource.id, "inst-1");
        assert_eq!(res[1].cloud_resource.id, "inst-2");

        let r0 = res[0].resource_impacts.clone().unwrap();
        let r1 = res[1].resource_impacts.clone().unwrap();

        assert_eq!(0.21321, r0.pe_use_megajoules);
        assert_eq!(0.21321, r1.pe_use_megajoules);
        assert!(
            res[2].resource_impacts.clone().is_none(),
            "This instance should return None impacts because it's type is unknown from API"
        );
    }

    #[test]
    fn should_convert_results_to_impacts() {
        let instance1: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    usage_duration_seconds: 3600,
                }),
            },
            tags: Vec::new(),
        };

        let raw_impacts =
            Some(serde_json::from_str(DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR).unwrap());
        let one_hour: f32 = 1 as f32;
        let cloud_resource_with_impacts: CloudResourceWithImpacts =
            boa_impacts_to_cloud_resource_with_impacts(&instance1, &raw_impacts, &one_hour);
        assert!(
            cloud_resource_with_impacts.resource_impacts.is_some(),
            "Emtpy impacts"
        );

        assert_eq!(
            0.21321,
            cloud_resource_with_impacts
                .resource_impacts
                .unwrap()
                .pe_use_megajoules
        );
    }
}
