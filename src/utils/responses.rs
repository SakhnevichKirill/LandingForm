use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

/// This structure is a default response that is returned to a client
/// independently on the result (whether it was a failure or success)
pub struct DefaultResponse {
    pub status_code: StatusCode,
    pub message: Option<String>,
    pub redirect: Option<String>,
}

/// This structure is a login/register response, that returns
/// a special login token with some additional information.
///
pub struct LoginResponse {
    pub status_code: StatusCode,
    pub message: String,
    pub token: Option<String>,
}

/// This is a required implementation of IntoResponse for DefaultResponse.
impl IntoResponse for DefaultResponse {
    fn into_response(self) -> axum::response::Response {
        let custom_response = DefaultResponseJson {
            message: self.message,
            redirect: self.redirect,
        };

        (self.status_code, Json(custom_response)).into_response()
    }
}

/// This is a required implementation of IntoResponse for LoginResponse.
impl IntoResponse for LoginResponse {
    fn into_response(self) -> axum::response::Response {
        let custom_response = LoginResponseJson {
            message: self.message,
            token: self.token,
        };
        (self.status_code, Json(custom_response)).into_response()
    }
}

/// This is a low-level helper structure for DefaultResponse.
/// It is sent with a status code to the client as a response.
#[derive(Serialize, ToResponse, ToSchema)]
pub struct DefaultResponseJson {
    #[schema(example = "This is a simple message.")]
    pub message: Option<String>,
    #[schema(example = "https://this/is/another/page.html")]
    pub redirect: Option<String>,
}

/// This is a low-level helper structure for LoginResponse.
/// It is sent with a status code to the client as a response.
#[derive(Serialize, ToSchema)]
pub struct LoginResponseJson {
    #[schema(example = "The user is registered successfully")]
    pub message: String,
    #[schema(example = "93$3vs$l3#$^*((*$#@%@#af49284")]
    pub token: Option<String>,
}
