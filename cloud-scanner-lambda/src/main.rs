use lambda_http::{http::StatusCode, IntoResponse, Request, RequestExt, Response};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};
use std::env;
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
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    match envy::from_env::<Config>() {
        Ok(config) => println!("{:#?}", config),
        Err(error) => panic!("{:#?}", error),
    }

    println!("Scan account invoked with event : {:?}", event);
    warn!("Using hardcoded use time of 1 hour");

    // warn!("Using 1 hour");
    let hours_use_time = 1 as f32;
    let filter_tags: Vec<String> = Vec::new();
    let impacts: String =
        cloud_scanner_cli::get_default_impacts(&hours_use_time, &filter_tags).await;
    Ok(response(StatusCode::OK, impacts))
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
            "message": "Go Serverless v1.0! Your function executed successfully!"
        })
        .into_response();
        let response = scan(request)
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
