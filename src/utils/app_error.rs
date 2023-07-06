use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct AppError {
    #[schema(example = "404")]
    code: StatusCode,
    #[schema(example = "Not found")]
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ResponseMessage {
                message: self.message,
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}
