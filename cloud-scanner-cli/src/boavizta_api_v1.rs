//!  Provide access to Boavizta API cloud impacts functions
//use crate::model::AwsInstanceWithImpacts;
use crate::cloud_resource::*;
use crate::impact_provider::{CloudResourceWithImpacts, ImpactProvider};
use crate::usage_location::*;
use anyhow::{Context, Result};
/// Get impacts of cloud resources through Boavizta API
use boavizta_api_sdk::apis::cloud_api;
use boavizta_api_sdk::apis::configuration;
use boavizta_api_sdk::models::UsageCloud;

/// Access data of API v1
pub struct BoaviztaApiV1 {
    configuration: boavizta_api_sdk::apis::configuration::Configuration,
}

impl BoaviztaApiV1 {
    pub fn new(api_url: &str) -> Self {
        let mut configuration = configuration::Configuration::new();
        configuration.base_path = api_url.to_string();
        BoaviztaApiV1 {
            configuration: configuration,
        }
    }

    // Returns the impacts of an instance from Boavizta API
    ///
    ///  The manufacture impacts returned represent the entire lifecycle of instance (i.e. Allocation TOTAL )
    async fn get_raws_impacts(self, cr: CloudResource) -> Option<serde_json::Value> {
        let instance_type = cr.resource_type;

        let cru = cr.usage.unwrap();

        let mut usage_cloud: UsageCloud = UsageCloud::new();

        usage_cloud.hours_use_time = Some((cru.usage_duration_seconds / 3600) as f32);
        usage_cloud.usage_location = Some(cr.location.iso_country_code.to_owned());

        let verbose = Some(false);
        let res = cloud_api::instance_cloud_impact_v1_cloud_aws_post(
            &self.configuration,
            Some(instance_type.as_str()),
            verbose,
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

    // /// Creates a CloudResourceWithImpacts from a CloudResource
    // pub fn get_resource_with_impacts(self, cr :CloudResource) ->  CloudResourceWithImpacts {

    //     impacts
    //     cri : CloudResourceWithImpacts = CloudResourceWithImpacts{

    //     }
    //     cri
    // }
}

/*

#[derive(Debug)]
pub struct CloudResourceWithImpacts {
    cloud_resource: CloudResource,
    resource_impacts: Impacts,
}

/// Impacts of an individual resource
#[derive(Debug, Default)]
pub struct Impacts {
    pub adp_manufacture_kgsbeq: f64,
    pub adp_use_kgsbeq: f64,
    pub pe_manufacture_megajoules: f64,
    pub pe_use_megajoules: f64,
    pub gwp_manufacture_kgco2eq: f64,
    pub gwp_use_kgco2eq: f64,
}

*/

#[async_trait]
impl ImpactProvider for BoaviztaApiV1 {
    async fn get_impacts(
        &self,
        resources: Vec<CloudResource>,
    ) -> Result<Vec<CloudResourceWithImpacts>> {
        let v: Vec<CloudResourceWithImpacts> = Vec::new();
        Ok(v)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_API_URL: &str = "https://api.boavizta.org";

    const DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR: &str = r#"   
    {
        "adp": {
            "manufacture": 0.0084, 
            "unit":"kgSbeq", 
            "use": 7.5e-10
        }, 
        "gwp": {
            "manufacture": 87.0,
            "unit": "kgCO2eq", 
            "use": 0.0015
            },
        "pe": {
            "manufacture": 1100.0,
            "unit": "MJ",
            "use": 0.17
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
                average_cpu_load: 100.0, // Will not be considered in v1
                usage_duration_seconds: 3600,
            }),
        };
        let api: BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);
        let res = api.get_raws_impacts(instance1).await.unwrap();

        let expected: serde_json::Value =
            serde_json::from_str(DEFAULT_RAW_IMPACTS_OF_M6GXLARGE_1HRS_FR).unwrap();
        assert_eq!(expected, res);
    }

    /*
    #[tokio::test]
    async fn get_instance_default_impacts_through_sdk_works() {
        let api : BoaviztaApiV1 = BoaviztaApiV1::new(TEST_API_URL);

        let instance_type = Some("m6g.xlarge");
        let verbose = Some(false);
        let usage_cloud: Option<UsageCloud> = Some(UsageCloud::new());

        let res = api.get_individual_impacts(resources)

        let res = cloud_api::instance_cloud_impact_v1_cloud_aws_post(
            &configuration,
            instance_type,
            verbose,
            usage_cloud,
        )
        .await;
        assert!(res.is_ok());
        let json = res.unwrap();
        println!("{:?}", json);
        println!("{}", json);
    }

    #[tokio::test]
    async fn get_default_impact_of_m6gxlarge() {
        // Parse the string of data into serde_json::Value.
        let expected: serde_json::Value = serde_json::from_str(DEFAULT_IMPACT_OF_M6XLARGE).unwrap();

        let usage_cloud: UsageCloud = UsageCloud::new();
        let instance: aws_sdk_ec2::model::Instance = aws_sdk_ec2::model::Instance::builder()
            .set_instance_type(Some(aws_sdk_ec2::model::InstanceType::M6gXlarge))
            .build();
        let impacts = get_impacts(&instance, usage_cloud, TEST_API_URL).await;

        assert_eq!(expected, impacts.unwrap());
    }

    #[tokio::test]
    async fn test_get_impacts_of_m6xlarge_without_region() {
        // Parse the string of data into serde_json::Value.
        let expected: serde_json::Value = serde_json::from_str(DEFAULT_IMPACT_OF_M6XLARGE).unwrap();

        let usage_cloud: UsageCloud = UsageCloud::new();
        //usage_cloud.days_use_time = Some(4 as f32);

        let instance: aws_sdk_ec2::model::Instance = aws_sdk_ec2::model::Instance::builder()
            .set_instance_type(Some(aws_sdk_ec2::model::InstanceType::M6gXlarge))
            .build();
        let impacts = get_impacts(&instance, usage_cloud, TEST_API_URL).await;

        assert_eq!(expected, impacts.unwrap());
    }

    #[tokio::test]
    async fn test_get_impacts_of_m6xlarge_with_fr_region() {
        // Parse the string of data into serde_json::Value.
        let expected: serde_json::Value =
            serde_json::from_str(DEFAULT_IMPACT_OF_M6XLARGE_FR).unwrap();

        let mut usage_cloud: UsageCloud = UsageCloud::new();
        //usage_cloud.days_use_time = Some(4 as f32);
        usage_cloud.usage_location = Some(String::from("FRA"));

        // impl std::convert::From<&str> for InstanceType {
        //     fn from(s: &str) -> Self {
        //         match s {
        //             "a1.2xlarge" => InstanceType::A12xlarge,
        //             "a1.4xlarge" => InstanceType::A14xlarge,
        //             "a1.large" => InstanceType::A1Large,
        //             "a1.medium" => InstanceType::A1Medium,
        //             "a1.metal" => InstanceType::A1Metal,
        //             "a1.xlarge" => InstanceType::A1Xlarge,

        // let itype: aws_sdk_ec2::model::InstanceType =
        //        aws_sdk_ec2::model::InstanceType::from("m6g.xlarge");
        let instance: aws_sdk_ec2::model::Instance = aws_sdk_ec2::model::Instance::builder()
            .set_instance_type(Some(aws_sdk_ec2::model::InstanceType::M6gXlarge))
            .build();

        let impacts = get_impacts(&instance, usage_cloud, TEST_API_URL).await;

        assert_eq!(expected, impacts.unwrap());
    }

    #[tokio::test]
    async fn get_instance_default_impacts_through_sdk_fails_for_some_instance_types() {
        let mut configuration = configuration::Configuration::new();
        configuration.base_path = String::from(TEST_API_URL);

        let known_failing_types = vec!["t2.xlarge", "t2.micro", "t2.small", "g3.4xlarge"];

        for failing_type in known_failing_types {
            let instance_type = Some(failing_type);
            let verbose = Some(false);
            let usage_cloud: Option<UsageCloud> = Some(UsageCloud::new());

            let res = cloud_api::instance_cloud_impact_v1_cloud_aws_post(
                &configuration,
                instance_type,
                verbose,
                usage_cloud,
            )
            .await;

            assert!(res.is_err());
        }
    }
    */
}
