//!  Provide access to Boavizta API cloud impacts functions
use crate::cloud_resource::*;
use crate::impact_provider::{CloudResourceWithImpacts, ImpactProvider, ResourceImpacts};
use anyhow::Result;
/// Get impacts of cloud resources through Boavizta API
use boavizta_api_sdk::apis::cloud_api;
use boavizta_api_sdk::apis::configuration;
use boavizta_api_sdk::models::{Allocation, UsageCloud};

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
        let instance_type = cr.resource_type;
        let verbose = Some(false);
        let mut usage_cloud: UsageCloud = UsageCloud::new();
        usage_cloud.hours_use_time = Some(usage_duration_hours.to_owned());
        usage_cloud.usage_location = Some(cr.location.iso_country_code.to_owned());
        if let Some(usage) = cr.usage {
            usage_cloud.time_workload = Some(usage.average_cpu_load as f32);
        }

        let res = cloud_api::instance_cloud_impact_v1_cloud_aws_post(
            &self.configuration,
            Some(instance_type.as_str()),
            verbose,
            Some(Allocation::Total),
            Some(usage_cloud),
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

    // /// Get the impacts of a single CloudResource
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

        resource_impacts = Some(ResourceImpacts {
            adp_manufacture_kgsbeq: results["adp"]["manufacture"].as_f64().unwrap(),
            adp_use_kgsbeq: results["adp"]["use"].as_f64().unwrap(),
            pe_manufacture_megajoules: results["pe"]["manufacture"].as_f64().unwrap(),
            pe_use_megajoules: results["pe"]["use"].as_f64().unwrap(),
            gwp_manufacture_kgco2eq: results["gwp"]["manufacture"].as_f64().unwrap(),
            gwp_use_kgco2eq: results["gwp"]["use"].as_f64().unwrap(),
        });
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

    //  const TEST_API_URL: &str = "https://api.boavizta.org";
    // Test against local  version of Boavizta API
    // const TEST_API_URL: &str = "http:/localhost:5000";
    // Test against dev version of Boavizta API
    const TEST_API_URL: &str = "https://dev.api.boavizta.org";

    const DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR: &str = r#"   
    {
        "adp": {
            "manufacture": 0.0083, 
            "unit":"kgSbeq", 
            "use": 9e-10
        }, 
        "gwp": {
            "manufacture": 83.0,
            "unit": "kgCO2eq", 
            "use": 0.002
            },
        "pe": {
            "manufacture": 1100.0,
            "unit": "MJ",
            "use": 0.2
            }
    }
    "#;

    #[tokio::test]
    async fn retrieve_instance_types_through_sdk_works() {
        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);

        let res = cloud_api::server_get_all_archetype_name_v1_cloud_aws_all_instances_get(
            &api.configuration,
        )
        .await
        .unwrap();
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn should_retrieve_raw_default_impacts_fr() {
        let instance1: CloudResource = CloudResource {
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_type: "m6g.xlarge".to_string(),
            usage: Some(CloudResourceUsage {
                average_cpu_load: 100.0,
                usage_duration_seconds: 3600,
            }),
            tags: vec![],
        };
        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let one_hour = 1.0 as f32;
        let res = api.get_raws_impacts(instance1, &one_hour).await.unwrap();

        let expected: serde_json::Value =
            serde_json::from_str(DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR).unwrap();
        assert_eq!(expected, res);
    }

    #[tokio::test]
    async fn should_retrieve_different_pe_impacts_for_different_cpu_load() {
        let mut tags: Vec<CloudResourceTag> = vec![];

        tags.push(CloudResourceTag {
            key: "appName".to_string(),
            value: Some("myApp".to_string()),
        });
        let instance1: CloudResource = CloudResource {
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_type: "m6g.xlarge".to_string(),
            usage: Some(CloudResourceUsage {
                average_cpu_load: 100.0,
                usage_duration_seconds: 3600,
            }),
            tags: tags,
        };

        let instance1_1percent: CloudResource = CloudResource {
            id: "inst-2".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_type: "m6g.xlarge".to_string(),
            usage: Some(CloudResourceUsage {
                average_cpu_load: 1.0,
                usage_duration_seconds: 3600,
            }),
            tags: vec![],
        };

        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let one_hour = 1.0 as f32;

        let mut instances: Vec<CloudResource> = Vec::new();
        instances.push(instance1);
        instances.push(instance1_1percent);

        let res = api.get_impacts(instances, &one_hour).await.unwrap();

        let r0 = res[0].resource_impacts.clone().unwrap();
        let r1 = res[1].resource_impacts.clone().unwrap();
        assert_eq!(0.2, r0.pe_use_megajoules);
        assert_eq!(0.09, r1.pe_use_megajoules);
    }

    #[tokio::test]
    async fn should_retrieve_multiple_default_impacts_fr() {
        let instance1: CloudResource = CloudResource {
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_type: "m6g.xlarge".to_string(),
            usage: Some(CloudResourceUsage {
                average_cpu_load: 100.0, // Will not be considered in v1
                usage_duration_seconds: 3600,
            }),
            tags: vec![],
        };

        let instance2: CloudResource = CloudResource {
            id: "inst-2".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_type: "m6g.xlarge".to_string(),
            usage: Some(CloudResourceUsage {
                average_cpu_load: 100.0, // Will not be considered in v1
                usage_duration_seconds: 3600,
            }),
            tags: vec![],
        };

        let instance3: CloudResource = CloudResource {
            id: "inst-3".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_type: "type-not-in-boa".to_string(),
            usage: Some(CloudResourceUsage {
                average_cpu_load: 100.0, // Will not be considered in v1
                usage_duration_seconds: 3600,
            }),
            tags: vec![],
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
        let r2 = res[2].resource_impacts.clone().is_none();

        assert_eq!(0.2, r0.pe_use_megajoules);
        assert_eq!(0.2, r1.pe_use_megajoules);
        assert_eq!(true, r2);
    }

    #[test]
    fn should_convert_results_to_impacts() {
        let instance1: CloudResource = CloudResource {
            id: "inst-1".to_string(),
            location: UsageLocation::from("eu-west-3"),
            resource_type: "m6g.xlarge".to_string(),
            usage: Some(CloudResourceUsage {
                average_cpu_load: 100.0, // Will not be considered in v1
                usage_duration_seconds: 3600,
            }),
            tags: vec![],
        };

        let raw_impacts =
            Some(serde_json::from_str(DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR).unwrap());

        let one_hour: f32 = 1 as f32;

        let cloud_resource_with_impacts: CloudResourceWithImpacts =
            boa_impacts_to_cloud_resource_with_impacts(&instance1, &raw_impacts, &one_hour);

        assert_eq!(
            0.2,
            cloud_resource_with_impacts
                .resource_impacts
                .unwrap()
                .pe_use_megajoules
        );
    }
}
