/// Get impacts of cloud resources through Boavizta API
use boavizta_api_sdk::apis::cloud_api;
use boavizta_api_sdk::apis::configuration;
use boavizta_api_sdk::models::UsageCloud;
use serde_derive::{Deserialize, Serialize};

/// Describes an instance with it's impacts
#[derive(Debug, Serialize, Deserialize)]
pub struct AwsInstanceWithImpacts {
    instance_id: String,
    instance_type: String,
    impacts: serde_json::Value,
}

/// Returns minimal AWS instance metadata with its impacts
pub async fn get_instance_with_default_impacts(
    instance: &aws_sdk_ec2::model::Instance,
) -> AwsInstanceWithImpacts {
    let instance_id = String::from(instance.instance_id.as_ref().unwrap());
    let instance_type = get_instance_type_as_string(instance);
    let res: serde_json::Value = get_default_impacts_from_instance(instance).await;

    AwsInstanceWithImpacts {
        instance_id,
        instance_type,
        impacts: res,
    }
}

/// Returns the default impacts of an instance from Boavizta API
/// without considering usage pattern (default is 100% usage)
///
/// Returns empty json of impact if any error
async fn get_default_impacts(instance_type: String) -> serde_json::Value {
    // Call boavizta API, passing an instance type, returns a standard impact
    let mut configuration = configuration::Configuration::new();
    configuration.base_path = String::from("https://api.boavizta.org");
    let opt_instance_type = Some(instance_type.as_str());

    let verbose = Some(false);
    let usage_cloud: Option<UsageCloud> = Some(UsageCloud::new());

    let res = cloud_api::instance_cloud_impact_v1_cloud_aws_post(
        &configuration,
        opt_instance_type,
        verbose,
        usage_cloud,
    )
    .await;
    match res {
        Ok(res) => res,
        Err(e) => {
            eprintln!(
                "Warning: Cannot get impacts from API for instance type {}: {}",
                instance_type, e
            );
            serde_json::from_str("{}").unwrap()
        }
    }
}

/// Returns the default impacts of an instance from Boavizta API
/// without considering usage pattern (default is 100% usage)
///
async fn get_default_impacts_from_instance(
    instance: &aws_sdk_ec2::model::Instance,
) -> serde_json::Value {
    // Call boavizta API, passing an instance type, returns a standard impact
    let instance_type = get_instance_type_as_string(instance);
    get_default_impacts(instance_type).await
}
/// Returns the instance type (extracted from instance) as a new String
fn get_instance_type_as_string(instance: &aws_sdk_ec2::model::Instance) -> String {
    instance.instance_type().unwrap().as_str().to_owned()
}

#[tokio::test]
async fn retrieve_instance_types_through_sdk_works() {
    let mut configuration = configuration::Configuration::new();
    configuration.base_path = String::from("https://api.boavizta.org");

    let res =
        cloud_api::server_get_all_archetype_name_v1_cloud_aws_all_instances_get(&configuration)
            .await
            .unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn get_instance_default_impacts_through_sdk_works() {
    let mut configuration = configuration::Configuration::new();
    configuration.base_path = String::from("https://api.boavizta.org");
    let instance_type = Some("m6g.xlarge");
    let verbose = Some(false);
    let usage_cloud: Option<UsageCloud> = Some(UsageCloud::new());

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
async fn get_default_impact() {
    let data = r#"   
    {
        "adp": {
            "manufacture": 0.0084, 
            "unit":"kgSbeq", 
            "use": 8.6e-6
        }, 
        "gwp": {
            "manufacture": 87.0,
             "unit": "kgCO2eq", 
             "use": 51.0
            },
        "pe": {
            "manufacture": 1100.0,
             "unit": "MJ",
              "use": 1700.0
            }
    }
    "#;

    // Parse the string of data into serde_json::Value.
    let expected: serde_json::Value = serde_json::from_str(data).unwrap();

    let instance_type = String::from("m6g.xlarge");
    let impacts = get_default_impacts(instance_type).await;

    assert_eq!(expected, impacts);
}

#[tokio::test]
async fn get_instance_default_impacts_through_sdk_fails_for_some_instance_types() {
    let mut configuration = configuration::Configuration::new();
    configuration.base_path = String::from("https://api.boavizta.org");

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
