use aws_sdk_cloudwatch::Error;
use boavizta_api_sdk::models::UsageCloud;

use structopt::clap::crate_version;
use structopt::StructOpt;
mod aws_api;
mod boavizta_api;

#[derive(StructOpt, Debug)]
#[structopt(name = "cloud-scanner-cli", version = crate_version!(), about = "List AWS instances and their impacts.")]
struct Opt {
    /// Filter instances on tags (like tag-key-1=val_1 tag-key_2=val2)
    #[structopt(short, long)]
    filter_tags: Vec<String>,

    /// Display results as text (instead of json)
    #[structopt(long)]
    text: bool,

    /// The number of hours of usage for which we want to estimate the impacts
    #[structopt(short, long)]
    hours_use_time: f32,

    /// Take the CPU load of instances into consideration to estimate the impacts
    #[structopt(short, long)]
    use_cpu_load: bool,
}

///
async fn print_default_impacts_as_json(hours_use_time: &f32, tags: &Vec<String>) {
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

async fn print_cpu_load_impacts_as_json(tags: &Vec<String>) {
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    if opt.use_cpu_load {
        print_cpu_load_impacts_as_json(&opt.filter_tags).await;
    } else {
        print_default_impacts_as_json(&opt.hours_use_time, &opt.filter_tags).await;
    }
    if opt.text {
        aws_api::display_instances_as_text(&opt.filter_tags).await;
    }

    Ok(())
}
