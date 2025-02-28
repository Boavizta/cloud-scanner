//!  A service to retrieve cloud resource impacts from Boavizta API.
use crate::impact_provider::{CloudResourceWithImpacts, ImpactProvider, ImpactsValues};
use anyhow::Result;
use boavizta_api_sdk::apis::cloud_api;
use boavizta_api_sdk::apis::component_api;
use boavizta_api_sdk::apis::configuration;
use boavizta_api_sdk::apis::utils_api;
use chrono::Utc;
use std::time::{Duration, Instant};

use crate::model::{
    CloudResource, EstimatedInventory, EstimationMetadata, ExecutionStatistics, Inventory,
    ResourceDetails,
};
use boavizta_api_sdk::models::{Cloud, Disk, UsageCloud};

/// Access data of Boavizta API
pub struct BoaviztaApiV1 {
    configuration: boavizta_api_sdk::apis::configuration::Configuration,
}

/// Create a new instance of service to access Boavizta API by passing API URL.
impl BoaviztaApiV1 {
    pub fn new(api_url: &str) -> Self {
        let mut configuration = configuration::Configuration::new();
        configuration.base_path = api_url.to_string();
        BoaviztaApiV1 { configuration }
    }

    // Returns the version of Boavizta API
    async fn get_api_version(&self) -> Option<String> {
        let res = utils_api::version_v1_utils_version_get(&self.configuration).await;
        if let Ok(serde_json::Value::String(v)) = res {
            Some(v)
        } else {
            error!("Cannot fetch API version");
            None
        }
    }

    // Returns the raw impacts (json) of an instance from Boavizta API for the duration of use (hours)
    async fn get_raws_impacts(
        &self,
        cr: CloudResource,
        usage_duration_hours: &f32,
        verbose: bool,
    ) -> Option<serde_json::Value> {
        let resource_details = cr.resource_details;
        let criteria = vec!["gwp".to_owned(), "adp".to_owned(), "pe".to_owned()];

        match resource_details {
            ResourceDetails::Instance {
                instance_type,
                usage,
            } => {
                let mut usage_cloud: UsageCloud = UsageCloud::new();

                //usage_cloud.hours_life_time = Some(usage_duration_hours.to_owned());
                usage_cloud.usage_location = Some(Some(cr.location.iso_country_code.to_owned()));

                if let Some(instance_usage) = usage {
                    usage_cloud.time_workload = Some(instance_usage.average_cpu_load);
                }

                let mut cloud: Cloud = Cloud::new();
                cloud.provider = Some(Some(String::from("aws")));
                cloud.instance_type = Some(Some(instance_type.clone()));
                cloud.usage = Some(Some(Box::new(usage_cloud)));

                let res = cloud_api::instance_cloud_impact_v1_cloud_instance_post(
                    &self.configuration,
                    Some(verbose),
                    Some(usage_duration_hours.to_owned().into()),
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
                attached_instances: _,
            } => {
                //let duration: f32 = usage.unwrap().usage_duration_seconds.into();
                let disk = Disk {
                    capacity: Some(Some(usage.unwrap().size_gb)),
                    units: None,
                    usage: None,
                    r#type: None,
                    density: None,
                    manufacturer: None,
                    model: None,
                    layers: None,
                };

                match storage_type.as_str() {
                    "st1" | "sc1" | "standard" => {
                        // This is a HDD
                        let res = component_api::disk_impact_bottom_up_v1_component_hdd_post(
                            &self.configuration,
                            Some(verbose),
                            Some(usage_duration_hours.to_owned().into()),
                            Some("DEFAULT"),
                            Some(criteria),
                            Some(disk),
                        )
                        .await;
                        match res {
                            Ok(res) => Some(res),
                            Err(e) => {
                                warn!(
                                    "Warning: Cannot get HHD impact from API for storage type {}: {}",
                                    storage_type, e
                                );
                                None
                            }
                        }
                    }
                    "gp2" | "gp3" | "io1" | "io2" => {
                        // Use impacts of an SSD
                        let res = component_api::disk_impact_bottom_up_v1_component_ssd_post(
                            &self.configuration,
                            Some(verbose),
                            Some(usage_duration_hours.to_owned().into()),
                            Some("DEFAULT"),
                            Some(criteria),
                            Some(disk),
                        )
                        .await;
                        match res {
                            Ok(res) => Some(res),
                            Err(e) => {
                                warn!(
                                    "Warning: Cannot get SSD impact from API for storage type {}: {}",
                                    storage_type, e
                                );
                                None
                            }
                        }
                    }
                    _ => {
                        warn!(
                            "Unknown storage type ({:?}), defaulting to using impacts of an SSD {:?}",
                            storage_type.as_str(),
                            disk
                        );
                        // All other types are considered SSD
                        let res = component_api::disk_impact_bottom_up_v1_component_ssd_post(
                            &self.configuration,
                            Some(verbose),
                            Some(usage_duration_hours.to_owned().into()),
                            Some("DEFAULT"),
                            Some(criteria),
                            Some(disk),
                        )
                        .await;
                        match res {
                            Ok(res) => Some(res),
                            Err(e) => {
                                warn!(
                                    "Warning: Cannot get SSD impact from API for type {}: {}",
                                    storage_type, e
                                );
                                None
                            }
                        }
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
        verbose: bool,
    ) -> CloudResourceWithImpacts {
        let raw_impacts = self
            .get_raws_impacts(resource.clone(), usage_duration_hours, verbose)
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
        inventory: Inventory,
        usage_duration_hours: &f32,
        verbose: bool,
    ) -> Result<EstimatedInventory> {
        let impact_query_start_time = Instant::now();

        let mut v: Vec<CloudResourceWithImpacts> = Vec::new();
        for resource in inventory.resources.iter() {
            let cri = self
                .get_resource_with_impacts(resource, usage_duration_hours, verbose)
                .await;
            v.push(cri.clone());
        }

        let mut inventory_duration = Duration::from_millis(0);
        if let Some(exec_stats) = inventory.metadata.execution_statistics {
            inventory_duration = exec_stats.inventory_duration;
        }
        let impact_estimation_duration = impact_query_start_time.elapsed();
        let execution_statistics = ExecutionStatistics {
            inventory_duration,
            impact_estimation_duration,
            total_duration: inventory_duration + impact_estimation_duration,
        };

        let estimated_inventory: EstimatedInventory = EstimatedInventory {
            impacting_resources: v,
            metadata: EstimationMetadata {
                estimation_date: Some(Utc::now()),
                description: Some("Estimation using Boavizta API".to_string()),
                cloud_scanner_version: Some(crate::get_version()),
                boavizta_api_version: self.get_api_version().await,
                execution_statistics: Some(execution_statistics),
            },
        };
        Ok(estimated_inventory)
    }
}

/// Convert raw results from Boavizta API into model objects
pub fn boa_impacts_to_cloud_resource_with_impacts(
    cloud_resource: &CloudResource,
    raw_result: &Option<serde_json::Value>,
    impacts_duration_hours: &f32,
) -> CloudResourceWithImpacts {
    let resource_impacts: Option<ImpactsValues>;
    if let Some(results) = raw_result {
        debug!("Raw results before conversion: {}", results);

        let impacts = &results["impacts"];

        let resource_details = cloud_resource.resource_details.clone();

        match resource_details {
            ResourceDetails::Instance {
                instance_type: _,
                usage: _,
            } => {
                resource_impacts = Some(ImpactsValues {
                    adp_manufacture_kgsbeq: impacts["adp"]["embedded"]["value"].as_f64().unwrap(),
                    adp_use_kgsbeq: impacts["adp"]["use"]["value"].as_f64().unwrap(),
                    pe_manufacture_megajoules: impacts["pe"]["embedded"]["value"].as_f64().unwrap(),
                    pe_use_megajoules: impacts["pe"]["use"]["value"].as_f64().unwrap(),
                    gwp_manufacture_kgco2eq: impacts["gwp"]["embedded"]["value"].as_f64().unwrap(),
                    gwp_use_kgco2eq: impacts["gwp"]["use"]["value"].as_f64().unwrap(),
                    raw_data: raw_result.clone(),
                });
            }
            ResourceDetails::BlockStorage {
                storage_type: _,
                usage: _,
                attached_instances: _,
            } => {
                // TODO: handle empty values differently, it could be better to have an option to be explicit about null values.
                info!("Impacts of the use phase of storage are not counted (only embedded impacts are counted).");
                resource_impacts = Some(ImpactsValues {
                    adp_manufacture_kgsbeq: impacts["adp"]["embedded"]["value"].as_f64().unwrap(),
                    adp_use_kgsbeq: 0 as f64,
                    pe_manufacture_megajoules: impacts["pe"]["embedded"]["value"].as_f64().unwrap(),
                    pe_use_megajoules: 0 as f64,
                    gwp_manufacture_kgco2eq: impacts["gwp"]["embedded"]["value"].as_f64().unwrap(),
                    gwp_use_kgco2eq: 0 as f64,
                    raw_data: raw_result.clone(),
                });
            }
            _ => {
                resource_impacts = None;
            }
        }
    } else {
        debug!(
            "Skipped resource: {:#?} while converting impacts, it has no impact data",
            cloud_resource
        );
        resource_impacts = None;
    };
    CloudResourceWithImpacts {
        cloud_resource: cloud_resource.clone(),
        impacts_values: resource_impacts,
        impacts_duration_hours: impacts_duration_hours.to_owned(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::get_version;
    use crate::model::{
        CloudProvider, CloudResource, InstanceState, InstanceUsage, InventoryMetadata,
        ResourceDetails, StorageUsage,
    };
    use crate::usage_location::UsageLocation;
    use assert_json_diff::assert_json_include;
    use assert_json_diff::{assert_json_matches, CompareMode, Config, NumericMode};

    const TEST_API_URL: &str = "https://api.boavizta.org";
    // Test against local  version of Boavizta API
    // const TEST_API_URL: &str = "http:/localhost:5000";
    // Test against dev version of Boavizta API
    // const TEST_API_URL: &str = "https://dev.api.boavizta.org";

    const DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR: &str =
        include_str!("../test-data/DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR.json");
    const DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR_VERBOSE: &str =
        include_str!("../test-data/DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR_VERBOSE.json");

    const DEFAULT_RAW_IMPACTS_OF_HDD: &str =
        include_str!("../test-data/DEFAULT_RAW_IMPACTS_OF_HDD.json");

    const DEFAULT_RAW_IMPACTS_OF_SSD_1000GB_1HR: &str =
        include_str!("../test-data/DEFAULT_RAW_IMPACTS_OF_SSD_1000GB_1HR.json");

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
    async fn get_api_version() {
        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let version = api.get_api_version().await;
        let expected = Some("1.3.8".to_owned());
        assert_eq!(version, expected, "Versions do not match");
    }

    #[tokio::test]
    async fn should_retrieve_raw_default_impacts_aws_fr() {
        let instance1: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::try_from("eu-west-3").unwrap(),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    state: InstanceState::Running,
                }),
            },
            tags: Vec::new(),
        };
        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let one_hour = 1.0_f32;
        let res = api
            .get_raws_impacts(instance1, &one_hour, false)
            .await
            .unwrap();

        let expected: serde_json::Value =
            serde_json::from_str(DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR).unwrap();
        assert_json_include!(actual: res, expected: expected);
    }

    #[tokio::test]
    async fn get_verbose_raw_impacts_of_a_hdd() {
        let hdd: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "disk-1".to_string(),

            location: UsageLocation::try_from("eu-west-3").unwrap(),
            resource_details: ResourceDetails::BlockStorage {
                storage_type: "st1".to_string(),
                usage: Some(StorageUsage { size_gb: 1000 }),
                attached_instances: None,
            },
            tags: Vec::new(),
        };

        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let one_hour = 1.0_f32;
        let res = api.get_raws_impacts(hdd, &one_hour, true).await.unwrap();

        let expected: serde_json::Value = serde_json::from_str(DEFAULT_RAW_IMPACTS_OF_HDD).unwrap();

        let config = Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat);
        assert_json_matches!(res, expected, config);
    }

    // not sure why it fails, ignoring it for now
    #[tokio::test]
    async fn get_verbose_raw_impacts_of_a_ssd() {
        let ssd: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "disk-1".to_string(),
            location: UsageLocation::try_from("eu-west-3").unwrap(),
            resource_details: ResourceDetails::BlockStorage {
                storage_type: "gp2".to_string(),
                usage: Some(StorageUsage { size_gb: 1000 }),
                attached_instances: None,
            },
            tags: Vec::new(),
        };

        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let one_hour = 1.0_f32;
        let res = api.get_raws_impacts(ssd, &one_hour, true).await.unwrap();

        let expected: serde_json::Value =
            serde_json::from_str(DEFAULT_RAW_IMPACTS_OF_SSD_1000GB_1HR).unwrap();

        let config = Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat);
        assert_json_matches!(res, expected, config);
    }

    #[tokio::test]
    async fn returns_different_pe_impacts_for_different_cpu_load() {
        let instance1: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::try_from("eu-west-3").unwrap(),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    state: InstanceState::Running,
                }),
            },
            tags: Vec::new(),
        };

        let instance1_1percent = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-2".to_string(),
            location: UsageLocation::try_from("eu-west-3").unwrap(),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 1.0,
                    state: InstanceState::Running,
                }),
            },
            tags: Vec::new(),
        };

        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let one_hour = 1.0_f32;

        let instances: Vec<CloudResource> = vec![instance1, instance1_1percent];

        let inventory = Inventory {
            metadata: InventoryMetadata {
                inventory_date: None,
                description: None,
                cloud_scanner_version: Some(get_version()),
                execution_statistics: None,
            },
            resources: instances,
        };

        let res = api.get_impacts(inventory, &one_hour, false).await.unwrap();

        let r0 = res.impacting_resources[0].impacts_values.clone().unwrap();
        let r1 = res.impacting_resources[1].impacts_values.clone().unwrap();
        assert_eq!(0.212, r0.pe_use_megajoules);
        assert_eq!(0.088, r1.pe_use_megajoules);
    }

    #[tokio::test]
    async fn should_retrieve_multiple_default_impacts_fr() {
        let instance1: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::try_from("eu-west-3").unwrap(),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    state: InstanceState::Running,
                }),
            },
            tags: Vec::new(),
        };

        let instance2: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-2".to_string(),
            location: UsageLocation::try_from("eu-west-3").unwrap(),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    state: InstanceState::Running,
                }),
            },
            tags: Vec::new(),
        };

        let instance3: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-3".to_string(),
            location: UsageLocation::try_from("eu-west-3").unwrap(),
            resource_details: ResourceDetails::Instance {
                instance_type: "type-not-in-boa".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    state: InstanceState::Running,
                }),
            },
            tags: Vec::new(),
        };

        let instances: Vec<CloudResource> = vec![instance1, instance2, instance3];
        let one_hour = 1.0_f32;

        let inventory = Inventory {
            metadata: InventoryMetadata {
                inventory_date: None,
                description: None,
                cloud_scanner_version: Some(get_version()),
                execution_statistics: None,
            },
            resources: instances,
        };

        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let res = api.get_impacts(inventory, &one_hour, false).await.unwrap();

        assert_eq!(3, res.impacting_resources.len());
        assert_eq!(res.impacting_resources[0].cloud_resource.id, "inst-1");
        assert_eq!(res.impacting_resources[1].cloud_resource.id, "inst-2");

        let r0 = res.impacting_resources[0].impacts_values.clone().unwrap();
        let r1 = res.impacting_resources[1].impacts_values.clone().unwrap();

        assert_eq!(0.212, r0.pe_use_megajoules);
        assert_eq!(0.212, r1.pe_use_megajoules);
        assert!(
            res.impacting_resources[2].impacts_values.clone().is_none(),
            "This instance should return None impacts because it's type is unknown from API"
        );
    }

    #[test]
    fn should_convert_basic_results_to_impacts() {
        let instance1: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::try_from("eu-west-3").unwrap(),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    state: InstanceState::Running,
                }),
            },
            tags: Vec::new(),
        };

        let raw_impacts =
            Some(serde_json::from_str(DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR).unwrap());
        let one_hour: f32 = 1_f32;
        let cloud_resource_with_impacts: CloudResourceWithImpacts =
            boa_impacts_to_cloud_resource_with_impacts(&instance1, &raw_impacts, &one_hour);
        assert!(
            cloud_resource_with_impacts.impacts_values.is_some(),
            "Empty impacts"
        );

        assert_eq!(
            0.212,
            cloud_resource_with_impacts
                .impacts_values
                .as_ref()
                .unwrap()
                .pe_use_megajoules
        );

        assert_eq!(
            0.212,
            cloud_resource_with_impacts
                .impacts_values
                .unwrap()
                .raw_data
                .unwrap()["impacts"]["pe"]["use"]["value"]
                .as_f64()
                .unwrap()
        );
    }
    #[test]
    fn convert_verbose_results_to_impacts() {
        let instance1: CloudResource = CloudResource {
            provider: CloudProvider::AWS,
            id: "inst-1".to_string(),
            location: UsageLocation::try_from("eu-west-3").unwrap(),
            resource_details: ResourceDetails::Instance {
                instance_type: "m6g.xlarge".to_string(),
                usage: Some(InstanceUsage {
                    average_cpu_load: 100.0,
                    state: InstanceState::Running,
                }),
            },
            tags: Vec::new(),
        };

        let raw_impacts =
            Some(serde_json::from_str(DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR_VERBOSE).unwrap());
        let one_hour: f32 = 1_f32;
        let cloud_resource_with_impacts: CloudResourceWithImpacts =
            boa_impacts_to_cloud_resource_with_impacts(&instance1, &raw_impacts, &one_hour);
        assert!(
            cloud_resource_with_impacts.impacts_values.is_some(),
            "Emtpy impacts"
        );

        assert_eq!(
            0.0005454,
            cloud_resource_with_impacts
                .impacts_values
                .unwrap()
                .raw_data
                .unwrap()["verbose"]["CPU-1"]["impacts"]["gwp"]["embedded"]["value"]
                .as_f64()
                .unwrap()
        );
    }
}
