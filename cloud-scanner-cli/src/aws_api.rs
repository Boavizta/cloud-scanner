//use aws_sdk_cloudwatch::;
//use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cloudwatch::Client as CW_client;
use aws_sdk_ec2::model::Instance;
use aws_sdk_ec2::{Client, Error /*Region*/};
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

    let mut instances: Vec<Instance> = Vec::new();

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

// async fn show_regions(client: &Client) -> Result<(), Error> {
//     let rsp = client.describe_regions().send().await?;
//     println!("Regions:");
//     for region in rsp.regions().unwrap_or_default() {
//         println!("  {}", region.region_name().unwrap());
//     }
//     Ok(())
// }

/// Query account for instances and display as text
pub async fn display_instances_as_text(tags: Vec<String>) {
    let shared_config = aws_config::from_env()
        //.region(Region::new("eu-west-1"))
        .load()
        .await;
    let client = Client::new(&shared_config);
    let instances = list_instances(&client, tags).await;
    print_instances(instances.unwrap());
}

// List metrics.
// snippet-start:[cloudwatch.rust.list-metrics]
pub async fn list_metrics() -> Result<(), aws_sdk_cloudwatch::Error> {
    let shared_config = aws_config::from_env().load().await;
    let client = CW_client::new(&shared_config);

    let rsp = client.list_metrics().send().await?;
    let metrics = rsp.metrics().unwrap_or_default();

    let num_metrics = metrics.len();

    for metric in metrics {
        println!("Namespace: {}", metric.namespace().unwrap_or_default());
        println!("Name:      {}", metric.metric_name().unwrap_or_default());
        println!("Dimensions:");

        if let Some(dimension) = metric.dimensions.as_ref() {
            for d in dimension {
                println!("  Name:  {}", d.name().unwrap_or_default());
                println!("  Value: {}", d.value().unwrap_or_default());
                println!();
            }
        }

        println!();
    }

    println!("Found {} metrics.", num_metrics);

    Ok(())
}
