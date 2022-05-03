use aws_sdk_cloudwatch::Error;
use boavizta_api::AwsInstanceWithImpacts;
use structopt::clap::crate_version;
use structopt::StructOpt;
mod aws_api;
mod boavizta_api;

#[derive(StructOpt, Debug)]
#[structopt(name = "cloud-scanner-cli", version = crate_version!(), about = "AWS account scanner to list instances.")]
struct Opt {
    /// Filter instances on tags (like tag-key-1=val_1 tag-key_2=val2)
    #[structopt(short, long)]
    filter_tags: Vec<String>,

    /// Display results as text (instead of json)
    #[structopt(short, long)]
    text: bool,

    /// Display available metrics
    #[structopt(short, long)]
    list_metrics: bool,
}

async fn print_all_impacts_as_json(tags: Vec<String>) {
    let instances = aws_api::list_instances(tags).await.unwrap();

    let mut instances_with_impacts: Vec<boavizta_api::AwsInstanceWithImpacts> = Vec::new();

    for instance in &instances {
        let value = boavizta_api::get_instance_with_impacts(instance).await;
        instances_with_impacts.push(value);
    }
    for instance_with_impact in instances_with_impacts {
        println!("{:?}", instance_with_impact);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    if opt.list_metrics {
        aws_api::list_metrics().await?;
    }

    if opt.text {
        aws_api::display_instances_as_text(opt.filter_tags).await;
    } else {
        println!("json output coming soon");
        print_all_impacts_as_json(opt.filter_tags).await;
    }

    Ok(())
}
