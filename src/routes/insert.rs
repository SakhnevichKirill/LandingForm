use axum::{http::StatusCode, Form};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use regex::Regex;
use serde_json;

use crate::{
    models::{NewUser, User},
    schema::users::{self, dsl::*},
    utils::{
        database_functions::establish_connection, responses::DefaultResponse,
        security::hash_password,
    },
};

use crate::routes::dispatch_email;
use crate::routes::dispatch_email::EmailPayload;

/// Add a new user to the database.
///
#[utoipa::path(
    post,
    tag = "AddUser",
    path = "/insert",
    request_body(content = NewUser, description = "Some data about a user", content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = StatusCode::OK, description = "The user is added to the database successfully", body = ResponseJson, example = json!("{\"message\": \"The user is added successfully!\", \"redirect\": null}")),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "There was an internal error on the server side", body = ResponseJson, example = json!("{\"message\": \"An error occurred on the server side.\", \"redirect\": null}")),
        (status = StatusCode::UNAUTHORIZED, description = "The user already exists in the database, no need to add them again", body = ResponseJson, example = json!("{\"message\": \"The user has already been added, no need to do that again.\", \"redirect\": \"http://localhost/user_added.html\"}"))
    )
)]
pub async fn insert(Form(user): Form<NewUser>) -> DefaultResponse {
    const SERVER_ERROR: &str = "Something went wrong on the server side";

    // Try to establish a connection with the database.
    let mut connection: PgConnection = match establish_connection() {
        Ok(connection) => connection,
        Err(error) => {
            eprintln!("{}", error);
            return DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            };
        }
    };

    // Check if the form is filled out properly.
    match verify_form(&user) {
        Ok(_) => (),
        Err(err) => {
            match err {
                // The user has already been added to the database.
                "USER_EXISTS" => {
                    return DefaultResponse {
                        status_code: StatusCode::UNAUTHORIZED,
                        message: Some("The user already exists".to_string()),
                        redirect: Some(
                            "http://127.0.0.1:5500/frontend/HTML/already_subscribed.html"
                                .to_string(),
                        ),
                    }
                }
                // The password is too short.
                "TOO_SHORT_PASSWORD" => {
                    return DefaultResponse {
                        status_code: StatusCode::UNAUTHORIZED,
                        message: Some("The password is too short".to_string()),
                        redirect: Some(
                            "http://127.0.0.1:5500/frontend/HTML/too_short_password.html"
                                .to_string(),
                        ),
                    }
                }
                // There is an issue with the phone number.
                "INCORRECT_PHONE_NUMBER" => {
                    return DefaultResponse {
                        status_code: StatusCode::UNAUTHORIZED,
                        message: Some("The phone number is written in a wrong format".to_string()),
                        redirect: Some(
                            "http://127.0.0.1:5500/frontend/HTML/incorrect_phone_number.html"
                                .to_string(),
                        ),
                    }
                }
                // Any other error, but this should not be called.
                error => {
                    eprintln!("{}", error);
                    return DefaultResponse {
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        message: Some(SERVER_ERROR.to_string()),
                        redirect: None,
                    };
                }
            };
        }
    }

    println!("Form is verified");

    // Hash user's password.
    let mut user = user;

    if let Some(passwrd) = user.password {
        user.password = match hash_password(passwrd).await {
            Ok(passwrd) => Some(passwrd),
            Err(error) => {
                eprintln!("{}", error);
                return DefaultResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: Some(SERVER_ERROR.to_string()),
                    redirect: None,
                };
            }
        }
        // println!("Hashed password: {}", user.password.clone().unwrap());
    }

    // Try to insert a user to the database.
    match diesel::insert_into(users::table)
        .values(&user)
        .get_result::<User>(&mut connection)
    {
        Ok(_) => (),
        Err(error) => {
            eprintln!("{}", error);
            return DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            };
        }
    };

    // Send an email to the new user if the user specified an email.
    if let Some(user_email) = user.email {
        // Check if the email provided is correct.
        if is_valid_email(&user_email) {
            // The email is valid, send the email to the user.
            dispatch_email(
                EmailPayload {
                    fullname: user.name,
                    email: user_email,
                    message: "Welcome to Manuspect!".to_string(),
                }
                .into(),
            )
            .await;
        }
    }

    // Everything went successfully, return OK to the user.
    DefaultResponse {
        status_code: StatusCode::OK,
        message: Some("The subscription was successful!".to_string()),
        redirect: Some("http://127.0.0.1:5500/frontend/HTML/subscribed.html".to_string()),
    }
}

/// This function verifies an email address.
fn is_valid_email(test_email: &str) -> bool {
    // Define the regular expression pattern for email validation
    let pattern =
        Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
            .unwrap();

    // Check if the email matches the pattern
    pattern.is_match(test_email)
}

// This function verifies a submitted form.
fn verify_form(user: &NewUser) -> Result<(), &'static str> {
    // Establish a new connection with the database.
    let connection = &mut establish_connection().map_err(|error| {
        println!("{}", error);
        "INTERNAL_SERVER_ERROR"
    })?;

    // Check that the phone number is a number.
    let phone_num = user.phone_number.clone();

    phone_num
        .parse::<i64>()
        .map_err(|_error| "INCORRECT_PHONE_NUMBER")?;

    // Make sure that the user with the same phone number does not exist.
    let result: Vec<User> = users
        .filter(users::columns::phone_number_code.eq(user.phone_number_code))
        .filter(users::columns::phone_number.eq(&user.phone_number))
        .load::<User>(connection)
        .map_err(|error| {
            println!("{}", error);
            "INTERNAL_SERVER_ERROR"
        })?;

    // If a user was found, then this user already exists.
    if result.len() > 0 {
        return Err("USER_EXISTS");
    }

    if let Some(passwrd) = &user.password {
        if passwrd.len() < 7 {
            return Err("TOO_SHORT_PASSWORD");
        }
    }

    Ok(())
}
