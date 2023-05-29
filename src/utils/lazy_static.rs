use std::collections::HashSet;

use lazy_static::lazy_static;
use prometheus::{
    opts, register_histogram_vec, register_int_counter, register_int_counter_vec, HistogramVec,
    IntCounter, IntCounterVec,
};

const HTTP_RESPONSE_TIME_CUSTOM_BUCKETS: &[f64; 14] = &[
    0.0005, 0.0008, 0.00085, 0.0009, 0.00095, 0.001, 0.00105, 0.0011, 0.00115, 0.0012, 0.0015,
    0.002, 0.003, 1.0,
];

lazy_static! {
    // The total number of requests.
    pub static ref HTTP_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        opts!("http_requests_total", "HTTP total requests"),
        &["method", "path"]
    )
    .expect("Cannot create a metric");

    // The total number of SSE connections.
    pub static ref HTTP_CONNECTED_SSE_CLIENTS: IntCounter = register_int_counter!(
        opts!("http_connected_sse_clients", "Connected SSE clients"),
    )
    .expect("Cannot create a metric");

    // Measure the HTTP response time in seconds.
    pub static ref HTTP_RESPONSE_TIME_SECONDS: HistogramVec = register_histogram_vec!(
        "http_response_time_seconds", "HTTP response time", &["method", "path"],
        HTTP_RESPONSE_TIME_CUSTOM_BUCKETS.to_vec()
    )
    .expect("Cannot create a metric");

    pub static ref ALLOWED_PATHS: HashSet<&'static str> = {
        let allowed_paths: HashSet<&str> = HashSet::from(["/", "/metrics", "/swagger-ui", "/api-doc", "/insert"]);
        allowed_paths
    };
}
