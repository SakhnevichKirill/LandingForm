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

// This is a required implementation of IntoResponse for DefaultResponse.
impl IntoResponse for DefaultResponse {
    fn into_response(self) -> axum::response::Response {
        let custom_response = ResponseJson {
            message: self.message,
            redirect: self.redirect,
        };

        (self.status_code, Json(custom_response)).into_response()
    }
}

// This is a low-level helper structure for DefaultResponse.
// It is sent with a status code to the client as a response.
#[derive(Serialize, ToResponse, ToSchema)]
pub struct ResponseJson {
    #[schema(example = "This is a simple message.")]
    pub message: Option<String>,
    #[schema(example = "https://this/is/another/page.html")]
    pub redirect: Option<String>,
}
