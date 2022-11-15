//! A standalone Metric HTTP endpoint

use rocket::State;

///  Configuration for the metric server
pub struct Config {
    pub boavizta_url: String,
}

/// Start the metric server
pub async fn run(config: Config) -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, metrics])
        .manage(config)
        .launch()
        .await?;
    Ok(())
}

/// Just display help
#[get("/")]
fn index(config: &State<Config>) -> String {
    warn!("Getting something on /");
    format!("Cloud scanner metric server is running.\n\nUsing boavizta API at: {}.\nValues are exposed on /metrics path and require passing a **region** in query string.\n e.g.  http://localhost:8000/metrics?aws_region=eu-west-3", config.boavizta_url)
}

/// Returns the metrics
#[get("/metrics?<aws_region>")]
async fn metrics(config: &State<Config>, aws_region: &str) -> String {
    warn!("Getting something on /metrics");
    let hours_use_time: f32 = 1.0;
    let tags = Vec::new();
    let metrics = crate::get_default_impacts_as_metrics(
        &hours_use_time,
        &tags,
        aws_region,
        &config.boavizta_url,
    )
    .await;
    metrics.unwrap()
}
