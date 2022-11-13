use crate::get_default_impacts_as_metrics;

// A standalone Metric HTTP endpoint
pub struct Config {
    pub boavizta_url: String,
}

use rocket::State;

pub async fn run(config: Config) -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, metrics])
        .manage(config)
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index(config: &State<Config>) -> String {
    format!("Cloud scanner metric endpoint is up.\n\nUsing boavizta api at {}.\nValues are exposed at /metrics and require passing a region in query string e.g.  http://localhost:8000/metrics?aws_region=eu-west-3", config.boavizta_url)
}

#[get("/metrics?<aws_region>")]
async fn metrics(config: &State<Config>, aws_region: &str) -> String {
    let hours_use_time: f32 = 1.0;
    let tags = Vec::new();
    //let aws_region = "eu-west-1".to_string();
    //let api_url = "https://api.boavizta.org";
    let metrics = crate::get_default_impacts_as_metrics(
        &hours_use_time,
        &tags,
        &aws_region,
        &config.boavizta_url,
    )
    .await;
    metrics.unwrap()
}
