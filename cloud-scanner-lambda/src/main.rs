use lambda_http::{http::StatusCode, IntoResponse, Request, RequestExt, Response};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

type E = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(lambda_http::service_fn(|event: Request| scan(event))).await?;
    Ok(())
}

async fn scan(event: Request) -> Result<impl IntoResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    println!("Scan account invoked with event : {:?}", event);

    let hours_use_time = 1 as f32;
    let filter_tags: Vec<String> = Vec::new();
    cloud_scanner_cli::print_default_impacts_as_json(&hours_use_time, &filter_tags).await;
    Ok(response(
        StatusCode::OK,
        json!({"message": "Scan done"}).to_string(),
    ))
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
