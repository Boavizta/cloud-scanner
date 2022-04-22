use structopt::clap::crate_version;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cloud-scanner-cli", version = crate_version!(), about = "AWS account scanner to list instances.")]
struct Opt {
    /// Filter instances on tags (like tag-key-1=val_1 tag-key_2=val2)
    #[structopt(short, long)]
    filter_tags: Vec<String>,

    /// Display results as text (instead of json)
    #[structopt(short, long)]
    text: bool,
}

//use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::model::Instance;
use aws_sdk_ec2::{Client, Error, /*Region*/};

// struct InstanceUsage {
//     instance: aws_sdk_ec2::model::Instance,
//     timestamp: f64,
//     duration: f64,
//     usage: Vec<Usage>,
// }

// struct Usage {
//     percentile: u8,
//     cpu_usage: u8,
//     mem_consumption: u8,
// }


/// List all instances of the current account
/// 
/// Filtering instance on tags is not yet implemented.
async fn list_instances(client: &Client, tags: Vec<String>) -> Result<Vec<Instance>, Error> {
    
    println!("Skipping tag filer {:?}", tags);
    
    let mut instances: Vec<Instance>;
    instances = Vec::new();

    let resp = client
        .describe_instances()
        //.set_instance_ids(Some(ids))
        //.set_filters() // Use filters for tags
        .send()
        .await?;

    for reservation in resp.reservations().unwrap_or_default() {
        for instance in reservation.instances().unwrap_or_default() {
            instances.push(instance.clone());
        }
    }
    Ok(instances)
}

/// Prints some instance details
fn print_instances(instances: Vec<Instance>) {
    for instance in &instances {
        println!("Instance ID: {}", instance.instance_id().unwrap());
        println!("Type:       {:?}", instance.instance_type().unwrap());
        println!("Tags:  {:?}", instance.tags().unwrap());
        println!();
    }
}


// async fn get_default_impact(instance: Instance){
//  // Call boavizta API, passing an instance type, returns a sandard imapact of Boavizata API
// }

// async fn show_regions(client: &Client) -> Result<(), Error> {
//     let rsp = client.describe_regions().send().await?;
//     println!("Regions:");
//     for region in rsp.regions().unwrap_or_default() {
//         println!("  {}", region.region_name().unwrap());
//     }
//     Ok(())
// }


/// Query account for instances and display as text
async fn display_instances_as_text(tags: Vec<String>) {

    let shared_config = aws_config::from_env()
        //.region(Region::new("eu-west-1"))
        .load()
        .await;
    let client = Client::new(&shared_config);
    let instances = list_instances(&client, tags).await;
    print_instances(instances.unwrap());
    
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    if opt.text {
        display_instances_as_text(opt.filter_tags).await;
    } else {
        println!("json output coming soon");
    }

    Ok(())
}
