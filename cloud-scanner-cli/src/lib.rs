use aws_sdk_cloudwatch::Error;
use boavizta_api_sdk::models::UsageCloud;

mod aws_api;
mod boavizta_api;

/// Prints impacts without using use time and default Boavizta impacts
pub async fn print_default_impacts_as_json(hours_use_time: &f32, tags: &Vec<String>) {
    let instances = aws_api::list_instances(tags).await.unwrap();
    let country_code = aws_api::get_current_iso_country().await;

    let mut instances_with_impacts: Vec<boavizta_api::AwsInstanceWithImpacts> = Vec::new();

    for instance in &instances {
        let mut usage_cloud: UsageCloud = UsageCloud::new();
        usage_cloud.usage_location = Some(String::from(country_code));
        usage_cloud.hours_use_time = Some(hours_use_time.to_owned());

        let value = boavizta_api::get_instance_impacts(instance, usage_cloud).await;
        instances_with_impacts.push(value);
    }

    let j = serde_json::to_string(&instances_with_impacts).unwrap();
    println!("{}", j);
}

/// Prints impacts considering the instance workload / CPU load
pub async fn print_cpu_load_impacts_as_json(tags: &Vec<String>) {
    eprintln!("Warning: getting impacts for specific CPU load is not yet implemented, will just display instances and average load");
    let instances = aws_api::list_instances(tags).await.unwrap();

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
pub async fn show_instances(tags: &Vec<String>) {
    aws_api::display_instances_as_text(tags).await;
}
