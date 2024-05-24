//! An HTTP endpoint that exposes the results of cloud-scanner like inventory, impacts or metrics.

use crate::model::{EstimatedInventory, Inventory};
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
        .mount(
            "/",
            openapi_get_routes![
                index,
                metrics,
                inventory,
                impacts,
                impacts_from_arbitrary_inventory
            ],
        )
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

/// # Returns Prometheus metrics.
///
/// Region is mandatory. Filter_tags (if any) should be written as string containing tag_name=tag_value
///
/// Results are estimated for one hour of use by default.
///
/// Example query: http://localhost:8000/metrics?aws_region=eu-west-3&filter_tag=Name=boatest&filter_tag=OtherTag=other-value&use_duration_hours=1.0&include_storage=true
#[openapi(tag = "metrics")]
#[get("/metrics?<aws_region>&<filter_tags>&<use_duration_hours>&<include_block_storage>")]
async fn metrics(
    config: &State<Config>,
    aws_region: &str,
    filter_tags: Option<Vec<String>>,
    use_duration_hours: Option<f32>,
    include_block_storage: Option<bool>,
) -> String {
    warn!("Getting something on /metrics");
    let hours_use_time = use_duration_hours.unwrap_or(1.0);
    warn!("Filtering on tags {:?}", filter_tags);
    let metrics = crate::get_impacts_as_metrics(
        &hours_use_time,
        &filter_tags.unwrap_or_default(),
        aws_region,
        &config.boavizta_url,
        include_block_storage.unwrap_or(false),
    )
    .await;
    metrics.unwrap()
}

/// # Returns current inventory.
///
/// Region is mandatory. Filter_tags (if any) should be written as string containing tag_name=tag_value
///
/// Example query: http://localhost:8000/inventory?aws_region=eu-west-3&filter_tag=Name=boatest&filter_tag=OtherTag=other-value
#[openapi(tag = "inventory")]
#[get("/inventory?<aws_region>&<filter_tags>&<include_block_storage>")]
async fn inventory(
    _config: &State<Config>,
    aws_region: &str,
    filter_tags: Option<Vec<String>>,
    include_block_storage: Option<bool>,
) -> Json<Inventory> {
    warn!("Getting something on /inventory");
    warn!("Filtering on tags {:?}", filter_tags);
    Json(
        crate::get_inventory(
            &filter_tags.unwrap_or_default(),
            aws_region,
            include_block_storage.unwrap_or(false),
        )
        .await
        .unwrap(),
    )
}

/// # Returns the impacts of current inventory.
///
/// Region is mandatory. Filter_tags (if any) should be written as string containing tag_name=tag_value
///
/// Example query: http://localhost:8000/impacts?aws_region=eu-west-3&filter_tag=Name=boatest&filter_tag=OtherTag=other-value&use_duration_hours=1.0
#[openapi(tag = "impacts")]
#[get(
    "/impacts?<aws_region>&<filter_tags>&<use_duration_hours>&<verbose_output>&<include_block_storage>"
)]
async fn impacts(
    _config: &State<Config>,
    aws_region: &str,
    filter_tags: Option<Vec<String>>,
    use_duration_hours: Option<f32>,
    verbose_output: Option<bool>,
    include_block_storage: Option<bool>,
) -> Json<EstimatedInventory> {
    let hours_use_time = use_duration_hours.unwrap_or(1.0);
    //let hours_use_time: f32 = 1.0;
    warn!(
        "Requesting /impacts for a duration of {} hours",
        hours_use_time
    );
    warn!("Filtering on tags {:?}", filter_tags);
    let res = crate::estimate_impacts(
        &hours_use_time,
        &filter_tags.unwrap_or_default(),
        aws_region,
        &_config.boavizta_url,
        verbose_output.unwrap_or(false),
        include_block_storage.unwrap_or(false),
    )
    .await
    .unwrap();
    Json(res)
}
/*
#[post(
"/impacts-from-json?<aws_region>&<use_duration_hours>", data = "<task>"
)]*/
/// # Retrieve the impacts of arbitrary inventory.
///
/// This can be used to evaluate impacts of a not yet implemented architecture.
///
/// The inventory is passed as json data in the request body.
#[openapi(tag = "impacts")]
#[post(
    "/impacts-from-arbitrary-inventory?<use_duration_hours>&<verbose_output>",
    data = "<inventory>"
)]
async fn impacts_from_arbitrary_inventory(
    _config: &State<Config>,
    use_duration_hours: Option<f32>,
    verbose_output: Option<bool>,
    inventory: Json<Inventory>,
) -> Json<EstimatedInventory> {
    let hours_use_time = use_duration_hours.unwrap_or(1.0);
    let res = crate::estimate_impacts_of_inventory(
        &hours_use_time,
        &_config.boavizta_url,
        verbose_output.unwrap_or(false),
        inventory.into_inner(),
    )
    .await
    .unwrap();
    Json(res)
}
