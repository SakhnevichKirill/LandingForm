pub mod dispatch_email;
mod index;
pub mod insert;
pub mod verify;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::custom_middleware::metrics_collector::{metrics_collector, metrics_display};
use crate::models::ApiDoc;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use dispatch_email::dispatch_email;
use index::index;
use insert::insert;
use verify::verify;

// This function creates a router with routes, middleware, layers and so on.
pub async fn create_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/verify", post(verify))
        .route("/insert", post(insert))
        .route("/dispatch_email", post(dispatch_email))
        .route("/metrics", get(metrics_display))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .layer(middleware::from_fn(metrics_collector))
}
