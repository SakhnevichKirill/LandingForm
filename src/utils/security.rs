use argon2::Argon2;
use axum::http::StatusCode;
use dotenvy::dotenv;
use std::env;

pub async fn hash_password(password: String) -> Result<String, StatusCode> {
    dotenv().ok();

    // Set up password hasher.
    let salt = env::var("ARGON2_SALT").map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    let argon2 = Argon2::default();

    let mut hashed_password = [0u8; 50];

    argon2
        .hash_password_into(password.as_bytes(), salt.as_bytes(), &mut hashed_password)
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(hashed_password
        .into_iter()
        .map(|val| char::from(val))
        .collect::<String>())
}
