use boavizta_api_sdk::models::UsageCloud;
#[macro_use]
extern crate log;
use pkg_version::*;
mod aws_api;
mod boavizta_api;
mod metrics;

#[derive(Debug, Default)]
pub struct Scan_summary {
    number_of_instances_total: u32,
    number_of_instances_assessed: u32,
    number_of_instances_not_assessed: u32,
    duration_of_use_hours: f64,
    adp_manuf_kgsbeq: f64,
    adp_use_kgsbeq: f64,
    pe_manuf_Mjoules: f64,
    pe_use_Mjoules: f64,
    gwp_manuf_kgco2eq: f64,
    gwp_use_kgco2eq: f64,
    aws_region: String,
    country: String,
}

pub async fn get_scan_summary(
    instances_with_impacts: &Vec<boavizta_api::AwsInstanceWithImpacts>,
) -> Scan_summary {
    let mut number_of_instances_not_assessed: u32 = 0;
    let mut pe_use_Mjoules: f64 = 0.0;
    let mut pe_manuf_Mjoules: f64 = 0.0;
    for instance in instances_with_impacts {
        let impacts = &instance.impacts;
        if impacts.as_str().unwrap().eq("{}") {
            warn!("detected instance without impacts");
            number_of_instances_not_assessed = number_of_instances_not_assessed + 1;
        } else {
            pe_use_Mjoules = pe_use_Mjoules + impacts["pe"]["use"].as_f64().unwrap();
            pe_manuf_Mjoules = pe_manuf_Mjoules + impacts["pe"]["manufacturing"].as_f64().unwrap();
        }
    }

    Scan_summary {
        pe_use_Mjoules: pe_use_Mjoules,
        pe_manuf_Mjoules: pe_manuf_Mjoules,
        ..Default::default()
    }
}

/// Standard scan (standard workload)
async fn standard_scan(
    hours_use_time: &f32,
    tags: &Vec<String>,
    aws_region: &str,
) -> Vec<boavizta_api::AwsInstanceWithImpacts> {
    let instances = aws_api::list_instances(tags, aws_region).await.unwrap();
    let country_code = aws_api::get_iso_country(aws_region);

    let mut instances_with_impacts: Vec<boavizta_api::AwsInstanceWithImpacts> = Vec::new();

    for instance in &instances {
        let mut usage_cloud: UsageCloud = UsageCloud::new();
        usage_cloud.usage_location = Some(String::from(country_code));
        usage_cloud.hours_use_time = Some(hours_use_time.to_owned());

        let value = boavizta_api::get_instance_impacts(instance, usage_cloud).await;
        instances_with_impacts.push(value);
    }
    instances_with_impacts
}

/// Returns default impacts as json
pub async fn get_default_impacts(
    hours_use_time: &f32,
    tags: &Vec<String>,
    aws_region: &str,
) -> String {
    let instances_with_impacts = standard_scan(hours_use_time, tags, aws_region).await;

    /*println!(
        "Summary: {:#?}",
        get_scan_summary(&instances_with_impacts).await
    );*/

    serde_json::to_string(&instances_with_impacts).unwrap()
}

/// Prints impacts without using use time and default Boavizta impacts
pub async fn print_default_impacts_as_json(
    hours_use_time: &f32,
    tags: &Vec<String>,
    aws_region: &str,
) {
    let j = get_default_impacts(&hours_use_time, tags, aws_region).await;
    println!("{}", j);
}

/// Prints impacts considering the instance workload / CPU load
pub async fn print_cpu_load_impacts_as_json(tags: &Vec<String>, aws_region: &str) {
    warn!("Warning: getting impacts for specific CPU load is not yet implemented, will just display instances and average load");
    let instances = aws_api::list_instances(tags, aws_region).await.unwrap();

    for instance in &instances {
        let instance_id: &str = instance.instance_id.as_ref().unwrap();
        let cpu_load = aws_api::get_average_cpu_load_24hrs(instance_id).await;
        println!("Instance ID: {}", instance.instance_id().unwrap());
        println!("Type:       {:?}", instance.instance_type().unwrap());
        println!(
            "AZ of use:  {:?}",
            instance.placement().unwrap().availability_zone().unwrap()
        );
        println!("Tags:  {:?}", instance.tags().unwrap());
        println!("Average CPU load:  {}", cpu_load);
        println!();
    }
}

/// List instances as text
pub async fn show_instances(tags: &Vec<String>, aws_region: &str) {
    aws_api::display_instances_as_text(tags, aws_region).await;
}

/// Return current version of the cloud-scanner-cli crate
pub fn get_version() -> String {
    const MAJOR: u32 = pkg_version_major!();
    const MINOR: u32 = pkg_version_minor!();
    const PATCH: u32 = pkg_version_patch!();
    format!("{}.{}.{}", MAJOR, MINOR, PATCH)
}
