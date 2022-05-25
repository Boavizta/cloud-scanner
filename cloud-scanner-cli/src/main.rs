use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// List aws instances and their environmental impact (from Boavizta API
struct Arguments {
    #[clap(short = 't', long)]
    /// Filter instances on tags (like tag-key-1=val_1 tag-key_2=val2)
    filter_tags: Vec<String>,
    #[clap(subcommand)]
    cmd: SubCommand,
    #[clap(short, long, default_value_t = String::from(""))]
    /// AWS region (default profile region is assumed if not provided)
    aws_region: String,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// get Average (standard) impacts for a given usage duration
    Standard {
        #[clap(short = 'u', long)]
        /// The number of hours of use for which we want to estimate the impacts
        hours_use_time: f32,
    },
    ///get impacts related to measured instance usage: depending on usage rate (use instance workload),
    Measured {},
    ///just list instances and their metadata (without impacts)
    ListInstances {},
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    match args.cmd {
        SubCommand::Standard{ hours_use_time } => {
            cloud_scanner_cli::print_default_impacts_as_json(&hours_use_time, &args.filter_tags)
                .await
        }
        SubCommand::Measured {} => {
            cloud_scanner_cli::print_cpu_load_impacts_as_json(&args.filter_tags).await
        }
        SubCommand::ListInstances {} => cloud_scanner_cli::show_instances(&args.filter_tags).await,
    }
    ()
}
