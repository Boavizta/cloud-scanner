use lambda_http::{http::StatusCode, IntoResponse, Request, RequestExt, Response};
use lambda_runtime::Error;
use pkg_version::*;

extern crate log;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    boavizta_api_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(lambda_http::service_fn(summary)).await?;
    Ok(())
}

async fn summary(event: Request) -> Result<impl IntoResponse, Error> {
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };

    println!(
        "Cloud scanner {}, using scanner lib {}",
        get_version(),
        cloud_scanner_cli::get_version()
    );
    println!("Using config {:#?}", config);
    println!("Scan account invoked with event : {:?}", event);

    let query_string_parameters = event.query_string_parameters();

    let aws_region = match query_string_parameters.first("aws_region") {
        Some(aws_region) => aws_region,
        None => {
            println!("No 'aws_region' parameter in path, will fallback to default");
            ""
        }
    };

    let filter_tags = match query_string_parameters.all("filter_tag") {
        Some(filter_tags) => filter_tags.iter().map(|t| t.to_string()).collect(),
        None => {
            let filter_tags: Vec<String> = Vec::new();
            println!("No 'filter_tag' parameter in path, will fallback to default");
            filter_tags
        }
    };

    let include_block_storage: bool = match query_string_parameters.first("include_block_storage") {
        Some(include_block_storage_string) => include_block_storage_string.parse().unwrap_or(false),
        None => false,
    };

    println!("Using fixed use time of 1 hour.");
    println!("Using aws_region: {}", aws_region);
    println!("Using tag filers: {:?}", filter_tags);
    println!("Include block storage: {:?}", include_block_storage);

    let impacts: String = cloud_scanner_cli::get_default_impacts_as_metrics(
        &1.0,
        &filter_tags,
        aws_region,
        &config.boavizta_api_url,
        include_block_storage,
    )
    .await
    .unwrap();
    Ok(response(StatusCode::OK, impacts))
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
        .header("Content-Type", "text/plain")
        .body(body)
        .unwrap()
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn scan_test() {
        let request = Request::default();
        let expected = json!({
            "message":"Missing 'hours_use_time' parameter in path"
        })
        .into_response();
        let response = summary(request)
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.await.body(), expected.await.body())
    }
}
 */
