use lambda_http::{http::StatusCode, IntoResponse, Request, RequestExt, Response};
/*use lambda_http::*;*/

use lambda_runtime::Error;
use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
use serde_json::json;

extern crate log;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    boavizta_api_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(lambda_http::service_fn(scan)).await?;
    Ok(())
}

async fn scan(event: Request) -> Result<impl IntoResponse, Error> {
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{error:#?}"),
    };

    println!(
        "Cloud scanner {}, using scanner lib {}",
        get_version(),
        cloud_scanner_cli::get_version()
    );
    println!("Using config {config:#?}");
    println!("Scan account invoked with event : {event:?}");

    let query_string_parameters = event.query_string_parameters();

    let use_duration_hours = match query_string_parameters.first("use_duration_hours") {
        Some(use_duration_hours) => use_duration_hours.parse::<f32>().unwrap(),
        None => {
            println!("Missing 'use_duration_hours' parameter in path");
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({ "message": "Missing 'use_duration_hours' parameter in path" }).to_string(),
            ));
        }
    };

    let aws_region = query_string_parameters
        .first("aws_region")
        .unwrap_or_default();
    if aws_region.is_empty() {
        println!("No 'aws_region' parameter in path, will fallback to default");
    }

    let filter_tags: Vec<String> = query_string_parameters
        .all("filter_tag")
        .unwrap_or_default()
        .iter()
        .map(ToString::to_string)
        .collect();
    if filter_tags.is_empty() {
        println!("No 'filter_tag' parameter in path, will fallback to default");
    }

    let verbose_output: bool = query_string_parameters
        .first("verbose_output")
        .and_then(|s| s.parse().ok())
        .unwrap_or(false);

    let include_block_storage: bool = query_string_parameters
        .first("include_block_storage")
        .and_then(|s| s.parse().ok())
        .unwrap_or(false);

    let summary_only: bool = query_string_parameters
        .first("summary_only")
        .and_then(|s| s.parse().ok())
        .unwrap_or(false);

    println!("Using use time of {use_duration_hours}");
    println!("Using aws_region {aws_region}");
    println!("Using tag filers {filter_tags:?}");

    let estimated_inventory = cloud_scanner_cli::estimate_impacts(
        &use_duration_hours,
        &filter_tags,
        aws_region,
        &config.boavizta_api_url,
        verbose_output,
        include_block_storage,
    )
    .await?;
    let json_impacts =
        cloud_scanner_cli::estimated_inventory_exporter::get_estimated_inventory_as_json(
            &estimated_inventory,
            aws_region,
            &use_duration_hours,
            summary_only,
        )
        .await?;

    Ok(response(StatusCode::OK, json_impacts))
}

/// Return current version of cloud-scanner-lambda
fn get_version() -> String {
    const MAJOR: u32 = pkg_version_major!();
    const MINOR: u32 = pkg_version_minor!();
    const PATCH: u32 = pkg_version_patch!();
    format!("{}.{}.{}", MAJOR, MINOR, PATCH)
}

/// HTTP Response with a JSON payload
fn response(status_code: StatusCode, body: String) -> Response<String> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn scan_test() {
        let request = Request::default();
        let expected = json!({
            "message":"Missing 'use_duration_hours' parameter in path"
        })
        .into_response();
        let response = scan(request)
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.await.body(), expected.await.body());
    }
}
