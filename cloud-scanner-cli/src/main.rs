use aws_sdk_cloudwatch::Error;
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
    }

    Ok(())
}
