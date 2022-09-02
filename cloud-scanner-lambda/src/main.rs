use lambda_http::{http::StatusCode, IntoResponse, Request, RequestExt, Response};
/*use lambda_http::*;*/

use lambda_runtime::{service_fn, Error, LambdaEvent};
use pkg_version::*;
use serde_json::{json, Value};
#[macro_use]
extern crate log;

type E = Box<dyn std::error::Error + Sync + Send + 'static>;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    boavizta_api_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(lambda_http::service_fn(|event: Request| scan(event))).await?;
    Ok(())
}

async fn scan(event: Request) -> Result<impl IntoResponse, Error> {
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

    let hours_use_time = match query_string_parameters.first("hours_use_time") {
        Some(hours_use_time) => hours_use_time.parse::<f32>().unwrap(),
        None => {
            println!("Missing 'hours_use_time' parameter in path");
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({ "message": "Missing 'hours_use_time' parameter in path" }).to_string(),
            ));
        }
    };

    let aws_region = match query_string_parameters.first("aws_region") {
        Some(aws_region) => aws_region,
        None => {
            println!("No 'aws_region' parameter in path, will fallback to default");
            //"eu-west-1"
            ""
        }
    };

    println!("Using use time of {}", hours_use_time);
    println!("Using aws_region {}", aws_region);
    let filter_tags: Vec<String> = Vec::new();
    let impacts: String =
        cloud_scanner_cli::get_default_impacts(&hours_use_time, &filter_tags, aws_region,&config.boavizta_api_url ).await;
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
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::Context;

    #[tokio::test]
    async fn scan_test() {
        let request = Request::default();
        let expected = json!({
            "message":"Missing 'hours_use_time' parameter in path"
        })
        .into_response();
        let response = scan(request)
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.await.body(), expected.await.body());
    }
}
