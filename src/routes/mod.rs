mod index;
pub mod insert;
pub mod verify;

use axum::{
    routing::{get, post},
    Router,
};

use crate::models::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use index::index;
use insert::insert;
use verify::verify;

pub async fn create_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/verify", post(verify))
        .route("/insert", post(insert))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
}
