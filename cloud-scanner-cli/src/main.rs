use structopt::clap::crate_version;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cloud-scanner-cli", version = crate_version!(), about = "List AWS instances and their impacts.")]
struct Opt {
    /// The number of hours of usage for which we want to estimate the impacts
    #[structopt(short, long)]
    hours_use_time: f32,

    /// Take the CPU load of instances into consideration to estimate the impacts
    #[structopt(short, long)]
    use_cpu_load: bool,

    /// Just list instance as text
    #[structopt(long)]
    instances: bool,

    /// Filter instances on tags (like tag-key-1=val_1 tag-key_2=val2)
    #[structopt(short, long)]
    filter_tags: Vec<String>,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    if opt.use_cpu_load {
        cloud_scanner_cli::print_cpu_load_impacts_as_json(&opt.filter_tags).await;
    } else {
        cloud_scanner_cli::print_default_impacts_as_json(&opt.hours_use_time, &opt.filter_tags)
            .await;
    }
    if opt.instances {
        cloud_scanner_cli::show_instances(&opt.filter_tags).await;
    }
    ()
}
