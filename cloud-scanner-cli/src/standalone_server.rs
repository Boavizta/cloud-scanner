//! A standalone  HTTP endpoint

use crate::model::{Inventory, ResourcesWithImpacts};
use rocket::State;
use rocket::{get, serde::json::Json};
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};

///  Configuration for the metric server
pub struct Config {
    pub boavizta_url: String,
}

/// Start the server
pub async fn run(config: Config) -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", openapi_get_routes![index, metrics, inventory, impacts])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .manage(config)
        .launch()
        .await?;
    Ok(())
}

/// Just display help
#[openapi(skip)]
#[get("/")]
fn index(config: &State<Config>) -> String {
    warn!("Getting request on /");
    let version: String = crate::get_version();
    format!("Cloud scanner metric server  {} is running.\n\nUsing Boavizta API at: {}.\nMetrics are exposed on /metrics path and require passing a **region** in query string.\n e.g.  http://localhost:8000/metrics?aws_region=eu-west-3 \n See also /swagger-ui .", version, config.boavizta_url)
}

/// Returns the metrics (corresponding to one hour use)
/// Region is mandatory, tags are optional
/// Example query: http://localhost:8000/metrics?aws_region=eu-west-3&filter_tag=Name=boatest&filter_tag=OtherTag=other-value&use_duration_hours=1.0
#[openapi(skip)]
#[get("/metrics?<aws_region>&<filter_tags>&<use_duration_hours>")]
async fn metrics(
    config: &State<Config>,
    aws_region: &str,
    filter_tags: Vec<String>,
    use_duration_hours: Option<f32>,
) -> String {
    warn!("Getting something on /metrics");
    let hours_use_time = use_duration_hours.unwrap_or(1.0);
    //let tags = Vec::new();
    warn!("Filtering on tags {:?}", filter_tags);
    let metrics = crate::get_default_impacts_as_metrics(
        &hours_use_time,
        &filter_tags,
        aws_region,
        &config.boavizta_url,
    )
    .await;
    metrics.unwrap()
}

/// Returns the inventory as json
/// Region is mandatory, tags are optional
/// Example query: http://localhost:8000/inventorynew?aws_region=eu-west-3&filter_tag=Name=boatest&filter_tag=OtherTag=other-value
#[openapi(tag = "inventory")]
#[get("/inventory?<aws_region>&<filter_tags>")]
async fn inventory(
    _config: &State<Config>,
    aws_region: &str,
    filter_tags: Vec<String>,
) -> Json<Inventory> {
    warn!("Getting something on /inventory");
    warn!("Filtering on tags {:?}", filter_tags);
    Json(
        crate::get_inventory(&filter_tags, aws_region)
            .await
            .unwrap(),
    )
}

/// Returns the impacts of use as json
/// Region is mandatory, tags are optional
/// Example query: http://localhost:8000/impacts?aws_region=eu-west-3&filter_tag=Name=boatest&filter_tag=OtherTag=other-value&use_duration_hours=1.0
#[openapi(tag = "impacts")]
#[get("/impacts?<aws_region>&<filter_tags>&<use_duration_hours>&<verbose_output>")]
async fn impacts(
    _config: &State<Config>,
    aws_region: &str,
    filter_tags: Vec<String>,
    use_duration_hours: Option<f32>,
    verbose_output: Option<bool>,
) -> Json<ResourcesWithImpacts> {
    let hours_use_time = use_duration_hours.unwrap_or(1.0);
    //let hours_use_time: f32 = 1.0;
    warn!(
        "Requesting /impacts for a duration of {} hours",
        hours_use_time
    );
    warn!("Filtering on tags {:?}", filter_tags);
    let res = crate::standard_scan(
        &hours_use_time,
        &filter_tags,
        aws_region,
        &_config.boavizta_url,
        verbose_output.unwrap_or(false),
    )
    .await
    .unwrap();
    Json(res)
}
