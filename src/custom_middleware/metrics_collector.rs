// This is a middleware that collects metrics

use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{Html, Response},
};
use prometheus::{Encoder, TextEncoder};

use crate::utils::lazy_static::{self, ALLOWED_PATHS};

/// This is a middleware that collects all the metrics.
/// NOTE: It also protects the application from trying
/// to serve nonexistent endpoints.
pub async fn metrics_collector<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // Extract the method and the path of a request.
    let request_method = req.method().to_string();
    let request_path = req.uri().path();
    let request_path_beginning = request_path.split('/').nth(1);

    // Check if the request path has to be ignored.
    if let Some(first_part) = request_path_beginning {
        let first_part = "/".to_owned() + first_part;
        if ALLOWED_PATHS.contains(first_part.as_str()) {
            // Record the new data.

            // Increment the number of requests to a particular page.
            lazy_static::HTTP_REQUESTS_TOTAL
                .with_label_values(&[&request_method, &first_part])
                .inc();

            // Start the timer that measures the request time.
            let histogram_timer = lazy_static::HTTP_RESPONSE_TIME_SECONDS
                .with_label_values(&[&request_method, &request_path])
                .start_timer();

            // Proceed to the requested source.
            let req_result = next.run(req).await;

            // Check if it was a successful request.
            if req_result.status().is_success() {
                histogram_timer.stop_and_record();
            } else {
                histogram_timer.stop_and_discard();
            }
            return Ok(req_result);
        }
    }

    Err(StatusCode::NOT_FOUND)
}

/// This function displays all the metrics on a webpage.
pub async fn metrics_display() -> Result<Html<String>, StatusCode> {
    let mut buffer = Vec::new();
    let metric_family = prometheus::gather();

    let encoder = TextEncoder::new();

    encoder
        .encode(&metric_family, &mut buffer)
        .map_err(|error| {
            eprintln!("{}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let output = String::from_utf8(buffer).map_err(|error| {
        eprintln!("{}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // let formatted_output = output.replace("\n", "<br>");

    Ok(Html(output))
}
