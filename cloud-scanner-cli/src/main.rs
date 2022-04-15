//use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{Client, Error, Region};

async fn show_instances(client: &Client) -> Result<(), Error> {
    let resp = client
        .describe_instances()
        //.set_instance_ids(Some(ids))
        //.set_filters() // Use filters for tags
        .send()
        .await?;

    //println!("{:?}", resp);

    for reservation in resp.reservations().unwrap_or_default() {
        for instance in reservation.instances().unwrap_or_default() {
            println!("Instance ID: {}", instance.instance_id().unwrap());
            println!(
                "Type:       {:?}",
                instance.instance_type().unwrap()
            );
            println!("Tags:  {:?}", instance.tags().unwrap());
            println!();
        }
    }

    Ok(())
}

async fn show_regions(client: &Client) -> Result<(), Error> {
    let rsp = client.describe_regions().send().await?;

    println!("Regions:");
    for region in rsp.regions().unwrap_or_default() {
        println!("  {}", region.region_name().unwrap());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::from_env()
        //.region(Region::new("eu-west-1"))
        .load()
        .await;
    let client = Client::new(&shared_config);

    // let req = client.list_tables().limit(10);
    // let resp = req.send().await?;
    show_regions(&client).await;
    show_instances(&client).await;

    //println!("Current DynamoDB tables: {:?}", resp.table_names.unwrap_or_default());
    Ok(())
}
