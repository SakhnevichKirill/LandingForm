use argon2::Argon2;
use axum::http::StatusCode;
use base64::{engine::general_purpose, Engine};
use std::env;

/// This function hashes passwords.
/// Currently it is powered by Argon2.
pub async fn hash_password(password: String) -> Result<String, StatusCode> {
    // Set up password hasher.
    let salt = env::var("ARGON2_SALT").map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    let argon2 = Argon2::default();

    let mut hashed_password = [0u8; 35];

    // Hash the password via a special argon2 function.
    argon2
        .hash_password_into(password.as_bytes(), salt.as_bytes(), &mut hashed_password)
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Encode the password in such a way so that it could be stored
    // in the database.
    let res = general_purpose::STANDARD.encode(hashed_password);

    Ok(res)
} // end fn hash_password.
