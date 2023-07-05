use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// This struct represents an existing user that already has
/// a verified account and just wants to log in in the system.
#[derive(Deserialize, ToSchema)]
pub struct LoginUserDto {
    #[schema(example = "john@gmail.com")]
    pub email: Option<String>,
    #[schema(example = 1)]
    pub phone_number_code: Option<i32>,
    #[schema(example = "9999999999")]
    pub phone_number: Option<String>,
    #[schema(example = "qwerty123")]
    pub password: String,
} // end struct LoginUser

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}
