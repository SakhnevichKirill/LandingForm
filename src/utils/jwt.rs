// This file contains the tools for JWT authentication.

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

/// This structure represents claims for JWT.
#[derive(Serialize, Deserialize)]
pub struct Claims {
    // Expiration time.
    exp: usize,
    // Issued at time.
    iat: usize,
} // end struct Claims

/// This function creates JWT.
/// If JWT generation is successful, then it returns
/// a string with the token. Otherwise, it returns
/// None.
pub fn create_jwt() -> Option<String> {
    // Setup data for claims.
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    now += Duration::minutes(10);
    let exp = now.timestamp() as usize;

    // Generate the claim.
    let claim = Claims { iat, exp };

    // Import secret for encoding the token.
    let secret = EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes());

    // Encode the token.
    if let Ok(token) = encode(&Header::default(), &claim, &secret) {
        // The token was generated successfully.
        return Some(token);
    } // end if

    // The encoding was unsuccessful, return None.
    None
} // end fn create_jwt

/// This function checks whether or not JWT is valid.
pub fn is_valid_jwt(token: &str) -> (bool, String) {
    // Import secret.
    let secret = DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes());

    // Try to decode the token and check whether or not it is valid.
    match decode::<Claims>(&token, &secret, &Validation::default()) {
        // Deal with errors
        Err(error) => match error.kind() {
            // This error might occur if the token has already expired.
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                return (
                    false,
                    "Your session has expired, please log in again".to_string(),
                );
            }
            // If any other error occurs, just inform a user about it.
            _ => return (false, "Something went wrong on the server side".to_string()),
        },
        // The token is valid, return OK.
        Ok(_) => return (true, "OK".to_string()),
    } // end match
} // end fn is_valid_jwt
