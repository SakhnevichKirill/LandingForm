pub mod auth_controller;
pub mod auth_service;
pub mod dto;

use axum::Router;

use self::auth_controller::AuthController;

use super::AppState;

/// This function returns a router with routes
/// for authentication.
pub fn init_auth_router() -> Router<AppState> {
    AuthController::init_auth_router()
} // end fn get_auth_routes
