use crate::model::AwsInstanceWithImpacts;
/// Get impacts of cloud resources through Boavizta API
use boavizta_api_sdk::apis::cloud_api;
use boavizta_api_sdk::apis::configuration;
use boavizta_api_sdk::models::Allocation;
use boavizta_api_sdk::models::UsageCloud;

/// Returns instance information aggregated with Boavizta impacts for this type of instance.
///
/// Uses the standard AWS instance object and associated workload as an input to query Boavizta API.
pub async fn get_instance_impacts(
    instance: &aws_sdk_ec2::model::Instance,
    usage_data: UsageCloud,
    api_url: &str,
) -> AwsInstanceWithImpacts {
    let instance_id = instance.instance_id.as_ref().unwrap().to_string();
    let instance_type = instance_type_as_string(instance);
    let impacts = get_impacts(instance, usage_data.clone(), api_url).await;

    AwsInstanceWithImpacts {
        instance_id,
        instance_type,
        impacts,
        usage_data,
    }
}

/// Returns the default impacts of an instance from Boavizta API
///
///  The manufacture impacts returned represent the entire lifecycle of instance (i.e. Allocation TOTAL )
async fn get_impacts(
    instance: &aws_sdk_ec2::model::Instance,
    usage_data: UsageCloud,
    api_url: &str,
) -> Option<serde_json::Value> {
    let instance_type = instance_type_as_string(instance);

    let mut configuration = configuration::Configuration::new();
    configuration.base_path = String::from(api_url);

    let opt_instance_type = Some(instance_type.as_str());
    let verbose = Some(false);
    let usage_cloud: Option<UsageCloud> = Some(usage_data);

    let res = cloud_api::instance_cloud_impact_v1_cloud_aws_post(
        &configuration,
        opt_instance_type,
        verbose,
        Some(Allocation::TOTAL),
        usage_cloud,
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

/// Returns the instance type as a new String
fn instance_type_as_string(instance: &aws_sdk_ec2::model::Instance) -> String {
    instance.instance_type().unwrap().as_str().to_owned()
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_API_URL: &str = "https://dev.api.boavizta.org";

    const DEFAULT_IMPACT_OF_M6XLARGE: &str = r#"   
    {
        "adp": {
            "manufacture": 0.0083, 
            "unit":"kgSbeq", 
            "use": 8.41e-6
        }, 
        "gwp": {
            "manufacture": 83.0,
            "unit": "kgCO2eq", 
            "use": 50.0
            },
        "pe": {
            "manufacture": 1100.0,
            "unit": "MJ",
            "use": 1686.0
            }
    }
    "#;

    const DEFAULT_IMPACT_OF_M6XLARGE_FR: &str = r#"   
    {
        "adp": {
            "manufacture": 0.0083, 
            "unit":"kgSbeq", 
            "use": 6.36e-6
        }, 
        "gwp": {
            "manufacture": 83.0,
            "unit": "kgCO2eq", 
            "use": 13.0
            },
        "pe": {
            "manufacture": 1100.0,
            "unit": "MJ",
            "use": 1478.0
            }
    }
    "#;

    #[tokio::test]
    async fn retrieve_instance_types_through_sdk_works() {
        let mut configuration = configuration::Configuration::new();
        configuration.base_path = String::from(TEST_API_URL);

        let res =
            cloud_api::server_get_all_archetype_name_v1_cloud_aws_all_instances_get(&configuration)
                .await
                .unwrap();
        println!("{:?}", res);
    }
    #[tokio::test]
    async fn get_instance_default_impacts_through_sdk_works() {
        let mut configuration = configuration::Configuration::new();
        configuration.base_path = String::from(TEST_API_URL);
        let instance_type = Some("m6g.xlarge");
        let verbose = Some(false);
        let usage_cloud: Option<UsageCloud> = Some(UsageCloud::new());

        let res = cloud_api::instance_cloud_impact_v1_cloud_aws_post(
            &configuration,
            instance_type,
            verbose,
            Some(Allocation::TOTAL),
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
                Some(Allocation::TOTAL),
                usage_cloud,
            )
            .await;

            assert!(res.is_err());
        }
    }
}
