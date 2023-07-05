use crate::routes::AppState;
use axum::{extract::State, Form};

use crate::{utils::app_error::AppError, utils::responses::LoginResponse};

use super::{
    auth_service::AuthService,
    dto::{login_dto::LoginUserDto, register_dto::RegisterUserDto},
};
use axum::{routing::post, Router};

#[utoipa::path(
    post,
    tag = "Registration",
    path = "/auth/register",
    request_body(content = NewUser, description = "A filled out registration form", content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = StatusCode::OK, description = "A user was registered successfully", body = LoginResponseJson, example = json!("{\"message\": \"SUCCESSFULLY AUTHORIZED\", \"token\": \"293u5429*2%23$#@jlasdfl\"}")),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "There was an internal error on the server side (The user is not inserted into the database in this case)", body = DefaultResponseJson, example = json!("{\"message\": \"An error occurred on the server side. Email could not be sent.\", \"redirect\": null}")),
        (status = StatusCode::UNAUTHORIZED, description = "In this case the user has whether made a mistake while filling out the form, or they are already registered")
    )
)]
async fn register(
    State(app_state): State<AppState>,
    Form(user): Form<RegisterUserDto>,
) -> LoginResponse {
    AuthService::register(app_state, user).await
}

#[utoipa::path(
    post,
    tag = "Login",
    path = "/auth/login",
    request_body(content = LoginUser, description = "A filled out login form", content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = StatusCode::OK, description = "A user has logged in successfully", body = LoginResponseJson, example = json!("{\"message\": \"SUCCESSFULLY AUTHORIZED\", \"token\": \"293u5429*2%23$#@stuff\"}")),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "There was an internal error on the server side (The user is not inserted into the database in this case)", body = DefaultResponseJson, example = json!("{\"message\": \"An error occurred on the server side. Email could not be sent.\", \"redirect\": null}")),
        (status = StatusCode::UNAUTHORIZED, description = "In this case the user has either made a mistake while filling out the form. \
        E.g. they could specify login or password or both in a wrong way")
    )
)]
async fn login(
    app_state: State<AppState>,
    user: Form<LoginUserDto>,
) -> Result<LoginResponse, AppError> {
    let auth = AuthController::new();
    auth.login(app_state, user).await
}

pub(super) struct AuthController {
    authService: AuthService,
}

impl AuthController {
    pub fn new() -> Self {
        AuthController {
            authService: AuthService,
        }
    }

    pub fn init_auth_router() -> Router<AppState> {
        Router::new()
            .route("/register", post(register))
            .route("/login", post(login))
    } // end fn get_auth_routes

    pub async fn login(
        &self,
        State(app_state): State<AppState>,
        Form(user): Form<LoginUserDto>,
    ) -> Result<LoginResponse, AppError> {
        self.authService.login(app_state, user).await
    }
}
