//! A standalone Metric HTTP endpoint

use rocket::State;
use rocket::{get, http::Status, serde::json::Json};
use serde::Serialize;
//use rocket_contrib::json::Json;
use crate::CloudResourceWithImpacts;

///  Configuration for the metric server
pub struct Config {
    pub boavizta_url: String,
}

/// Start the metric server
pub async fn run(config: Config) -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, metrics, impacts])
        .manage(config)
        .launch()
        .await?;
    Ok(())
}

/// Just display help
#[get("/")]
fn index(config: &State<Config>) -> String {
    warn!("Getting request on /");
    let version: String = crate::get_version();
    format!("Cloud scanner metric server  {} is running.\n\nUsing Boavizta API at: {}.\nMetrics are exposed on /metrics path and require passing a **region** in query string.\n e.g.  http://localhost:8000/metrics?aws_region=eu-west-3", version, config.boavizta_url)
}

/// Returns the metrics
/// Region is mandatory, tags are optional
/// Example query: http://localhost:8000/metrics?aws_region=eu-west-3&filter_tag=Name=boatest&filter_tag=OtherTag=other-value
#[get("/metrics?<aws_region>&<filter_tag>")]
async fn metrics(config: &State<Config>, aws_region: &str, filter_tag: Vec<String>) -> String {
    warn!("Getting something on /metrics");
    let hours_use_time: f32 = 1.0;
    //let tags = Vec::new();
    warn!("Filtering on tags {:?}", filter_tag);
    let metrics = crate::get_default_impacts_as_metrics(
        &hours_use_time,
        &filter_tag,
        aws_region,
        &config.boavizta_url,
    )
    .await;
    metrics.unwrap()
}

/// Returns impacts
/// Region is mandatory, tags are optional
/// Example query: http://localhost:8000/impacts?aws_region=eu-west-3&filter_tag=Name=boatest&filter_tag=OtherTag=other-value
#[get("/impacts?<aws_region>&<filter_tag>")]
async fn impacts(
    config: &State<Config>,
    aws_region: &str,
    filter_tag: Vec<String>,
) -> Json<Vec<CloudResourceWithImpacts>> {
    warn!("Getting something on /impacts");
    let hours_use_time: f32 = 1.0;
    //let tags = Vec::new();
    warn!("Filtering on tags {:?}", filter_tag);

    let res = crate::get_impacts(
        &hours_use_time,
        &filter_tag,
        aws_region,
        &config.boavizta_url,
    )
    .await;
    Json(res.unwrap())
}


