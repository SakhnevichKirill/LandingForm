use axum::{routing::post, Router};

pub mod login;
pub mod register;

use login::login;
use register::register;

use super::AppState;

/// This function returns a router with routes
/// for authentication.
pub fn get_auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
} // end fn get_auth_routes
