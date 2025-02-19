use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use cloud_scanner_cli::estimated_inventory_exporter::get_estimated_inventory_as_json;
use cloud_scanner_cli::inventory_exporter::print_inventory;
use cloud_scanner_cli::model::EstimatedInventory;
use std::path::PathBuf;

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
    /// Get estimation of impacts for a given usage duration as json
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

        /// Returns only the summary of the impacts as json
        #[arg(short = 's', long)]
        summary_only: bool,

        /// Estimate impacts of an existing json inventory file (instead of performing live inventory)
        #[arg(short, long)]
        inventory_file: Option<PathBuf>,
    },
    /// Get estimation of impacts for a given usage duration as OpenMetrics (Prometheus) instead of json
    Metrics {
        #[arg(short = 'u', long)]
        /// The number of hours of use for which we want to estimate the impacts
        use_duration_hours: f32,

        #[arg(long, short = 'b', action)]
        /// Experimental feature: estimate impacts of block storage
        include_block_storage: bool,
    },
    /// List resources (and instances average cpu load for the last 5 minutes) without returning impacts
    Inventory {
        #[arg(long, short = 'b', action)]
        /// Experimental feature: include block storage in the inventory
        include_block_storage: bool,

        /// Print the json schema of the inventory (instead of performing inventory)
        #[arg(short = 's', long)]
        print_json_schema: bool,
    },
    ///  Run as a standalone server.
    /// Access metrics (e.g. http://localhost:8000/metrics?aws_region=eu-west-3), inventory or impacts (see http://localhost:8000/swagger-ui)
    Serve {},
}

fn set_region(optional_region: Option<String>) -> String {
    optional_region.map_or_else(String::new, |region_arg| {
        info!("Using region: {}", region_arg);
        region_arg
    })
}

fn set_api_url(optional_url: Option<String>) -> String {
    optional_url.map_or_else(
        || {
            let default_url = "https://api.boavizta.org".to_string();
            warn!("Using default API at:  {default_url}");
            default_url
        },
        |url_arg| {
            info!("Using API at:  {}", url_arg);
            url_arg
        },
    )
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
            summary_only,
            inventory_file,
        } => {
            if let Some(path) = inventory_file {
                info!("Providing estimation for inventory file");
                let i = cloud_scanner_cli::estimate_impacts_of_inventory_file(
                    &use_duration_hours,
                    &api_url,
                    output_verbose_json,
                    &path,
                )
                .await?;
                println!("{}", serde_json::to_string(&i)?);
            } else {
                info!("Providing estimation for live inventory");
                let i: EstimatedInventory = cloud_scanner_cli::estimate_impacts(
                    &use_duration_hours,
                    &args.filter_tags,
                    &region,
                    &api_url,
                    output_verbose_json,
                    include_block_storage,
                )
                .await?;
                let result =
                    get_estimated_inventory_as_json(&i, &region, &use_duration_hours, summary_only)
                        .await?;
                println!("{result}");
            }
        }
        SubCommand::Metrics {
            use_duration_hours,
            include_block_storage,
        } => {
            // Produce metrics
            let metrics = cloud_scanner_cli::get_impacts_as_metrics(
                &use_duration_hours,
                &args.filter_tags,
                &region,
                &api_url,
                include_block_storage,
            )
            .await?;
            println!("{metrics}");
        }
        SubCommand::Inventory {
            include_block_storage,
            print_json_schema,
        } => {
            if print_json_schema {
                cloud_scanner_cli::inventory_exporter::print_inventory_schema()?;
            } else {
                info!("Using filter tags {:?}", &args.filter_tags);
                let inventory = cloud_scanner_cli::get_inventory(
                    &args.filter_tags,
                    &region,
                    include_block_storage,
                )
                .await?;
                print_inventory(&inventory).await?;
            }
        }
        SubCommand::Serve {} => cloud_scanner_cli::serve_metrics(&api_url).await?,
    }
    Ok(())
}
