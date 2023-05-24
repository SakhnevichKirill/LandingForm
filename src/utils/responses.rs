use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

pub struct DefaultResponse {
    pub status_code: StatusCode,
    pub message: Option<String>,
    pub redirect: Option<String>,
}

impl IntoResponse for DefaultResponse {
    fn into_response(self) -> axum::response::Response {
        let custom_response = ResponseJson {
            message: self.message,
            redirect: self.redirect,
        };

        (self.status_code, Json(custom_response)).into_response()
    }
}

#[derive(Serialize, ToResponse, ToSchema)]
pub struct ResponseJson {
    pub message: Option<String>,
    pub redirect: Option<String>,
}
