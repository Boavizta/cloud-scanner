use aws_sdk_cloudwatch::{Client as CW_client};
use structopt::clap::crate_version;
use structopt::StructOpt;
use aws_sdk_cloudwatch::Error;
mod aws_api;

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



#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let shared_config = aws_config::from_env().load().await;
    let client = CW_client::new(&shared_config);

    aws_api::show_metrics(&client).await?;

    if opt.text {
        aws_api::display_instances_as_text(opt.filter_tags).await;
    } else {
        println!("json output coming soon");
    }

    Ok(())
}
