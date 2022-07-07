use std::path::PathBuf;

use clap::{Parser, Subcommand};
#[macro_use]
extern crate log;
extern crate loggerv;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// List aws instances and their environmental impact (from Boavizta API)
struct Arguments {
    #[clap(subcommand)]
    cmd: SubCommand,
    #[clap(short, long)]
    /// AWS region (default profile region is assumed if not provided)
    aws_region: Option<String>,
    #[clap(short, long)]
    /// Optional Boavizta API URL (if you want to use your own instance)
    boavizta_api_url: Option<String>,
    #[clap(short = 't', long)]
    /// Filter instances on tags (like tag-key-1=val_1 tag-key_2=val2)
    filter_tags: Vec<String>,
    // /// Save results to a file (instead of printing json to stdout)
    // #[clap(short, long, parse(from_os_str))]
    // out_file: Option<PathBuf>,
    /// Enable logging, use multiple `v`s to increase verbosity
    #[clap(short, long, parse(from_occurrences))]
    verbosity: u64,
    /// Returns OpenMetrics (Prometheus like) instead of json output
    #[clap(short = 'm', long)]
    as_metrics: bool,
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

fn set_region(optional_region: Option<String>) -> String {
    match optional_region {
        Some(region_arg) => {
            info!("Using region {}", region_arg);
            region_arg
        }
        None => {
            let default_region = "eu-west-1".to_string();
            warn!("Using default region {}", default_region);
            default_region
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    loggerv::init_with_verbosity(args.verbosity).unwrap();

    let region = set_region(args.aws_region);

    match args.cmd {
        SubCommand::Standard { hours_use_time } => {
            if args.as_metrics {
                cloud_scanner_cli::print_default_impacts_as_metrics(
                    &hours_use_time,
                    &args.filter_tags,
                    &region,
                )
                .await
            } else {
                cloud_scanner_cli::print_default_impacts_as_json(
                    &hours_use_time,
                    &args.filter_tags,
                    &region,
                )
                .await
            }
        }
        SubCommand::Measured {} => {
            cloud_scanner_cli::print_cpu_load_impacts_as_json(&args.filter_tags, &region).await
        }
        SubCommand::ListInstances {} => {
            cloud_scanner_cli::show_instances(&args.filter_tags, &region).await
        }
    }
}
