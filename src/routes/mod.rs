mod index;
mod insert;
mod verify;

use axum::{
    routing::{get, post},
    Router,
};

use index::index;
use insert::insert;
use verify::verify;

pub async fn create_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/verify", post(verify))
        .route("/insert", post(insert))
}
