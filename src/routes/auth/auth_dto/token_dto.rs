use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// This structure is a login/register response, that returns
/// a special login token with some additional information.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct TokenDto {
    #[schema(example = "zhsdiguo19034r8kjbkHV")]
    pub access_token: Option<String>,
    #[schema(example = "zhsdiguo19034r8kjbkHV", )]
    pub refresh_token: Option<String>,
}

impl IntoResponse for TokenDto {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
