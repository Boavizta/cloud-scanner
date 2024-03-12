use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
#[macro_use]
extern crate log;
extern crate loggerv;

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// List aws instances and their environmental impact (from Boavizta API)
struct Arguments {
    #[command(subcommand)]
    cmd: SubCommand,

    #[arg(short, long)]
    /// AWS region (The default aws profile region is used if not provided)
    aws_region: Option<String>,

    #[arg(short, long)]
    /// Optional Boavizta API URL if you want to use your own instance (URL without the trailing slash, e.g. https://api.boavizta.org)
    boavizta_api_url: Option<String>,

    #[arg(short = 't', long)]
    /// Filter instances on tags (like tag-key-1=val_1 tag-key_2=val2)
    filter_tags: Vec<String>,

    #[arg(short, long,  action = clap::ArgAction::Count)]
    /// Enable logging and show execution duration, use multiple `v`s to increase logging level warning to debug
    verbosity: u8,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Get estimation of impacts for a given usage duration
    Estimate {
        #[arg(short = 'u', long)]
        /// The number of hours of use for which we want to estimate the impacts
        use_duration_hours: f32,

        #[arg(long, short = 'f', action)]
        /// Retrieve and output the details from BoaviztaAPI (equivalent to the verbose flag when querying Boavizta API)
        output_verbose_json: bool,

        #[arg(long, short = 'b', action)]
        /// Experimental feature: estimate impacts of block storage
        include_block_storage: bool,

        /// Returns results as OpenMetrics (Prometheus) instead of json
        #[arg(short = 'm', long)]
        as_metrics: bool,
    },
    /// List instances and  their average cpu load for the last 5 minutes (without returning impacts)
    Inventory {
        #[arg(long, short = 'b', action)]
        /// Experimental feature: include block storage in the inventory
        include_block_storage: bool,
    },
    ///  Run as a standalone server.
    /// Access metrics (e.g. http://localhost:8000/metrics?aws_region=eu-west-3), inventory or impacts (see http://localhost:8000/swagger-ui)
    Serve {},
}

fn set_region(optional_region: Option<String>) -> String {
    match optional_region {
        Some(region_arg) => {
            info!("Using region: {}", region_arg);
            region_arg
        }
        None => "".to_owned(),
    }
}

fn set_api_url(optional_url: Option<String>) -> String {
    match optional_url {
        Some(url_arg) => {
            info!("Using API at:  {}", url_arg);
            url_arg
        }
        None => {
            let default_url = "https://api.boavizta.org".to_string();
            warn!("Using default API at:  {}", default_url);
            default_url
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();

    loggerv::init_with_verbosity(args.verbosity.into()).context("Cannot initialize logger")?;
    info!(
        "Starting cloud scanner {}",
        cloud_scanner_cli::get_version()
    );

    let region = set_region(args.aws_region);

    let api_url: String = set_api_url(args.boavizta_api_url);

    match args.cmd {
        SubCommand::Estimate {
            use_duration_hours,
            include_block_storage,
            output_verbose_json,
            as_metrics,
        } => {
            if as_metrics {
                cloud_scanner_cli::print_default_impacts_as_metrics(
                    &use_duration_hours,
                    &args.filter_tags,
                    &region,
                    &api_url,
                    include_block_storage,
                )
                .await?
            } else {
                cloud_scanner_cli::print_default_impacts_as_json(
                    &use_duration_hours,
                    &args.filter_tags,
                    &region,
                    &api_url,
                    output_verbose_json,
                    include_block_storage,
                )
                .await?
            }
        }
        SubCommand::Inventory {
            include_block_storage,
        } => {
            info!("Using filter tags {:?}", &args.filter_tags);
            cloud_scanner_cli::show_inventory(&args.filter_tags, &region, include_block_storage)
                .await?
        }
        SubCommand::Serve {} => cloud_scanner_cli::serve_metrics(&api_url).await?,
    }
    Ok(())
}
