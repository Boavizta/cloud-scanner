use crate::get_default_impacts_as_metrics;

// A standalone Metric HTTP endpoint
pub struct MetricEndpoint {
    boavizta_url: String,
}

pub async fn run() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, metrics])
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Cloud scanner metric endpoint is up. Values are exposed at /metrics"
}

#[get("/metrics")]
async fn metrics() -> String {
    let hours_use_time: f32 = 1.0;
    let tags = Vec::new();
    let aws_region = "eu-west-1".to_string();
    let api_url = "https://api.boavizta.org";
    let metrics =
        crate::get_default_impacts_as_metrics(&hours_use_time, &tags, &aws_region, &api_url).await;
    metrics.unwrap()
}
