// This is an endpoint that inserts a new user to the database.
// NOTE: This endpoint is used to add unverified accounts
// to the database.

use axum::{extract::State, http::StatusCode, Form};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use serde_json;

use crate::{
    models::{NewUser, User},
    utils::responses::DefaultResponse,
};

use crate::routes::dispatch_email;
use crate::routes::dispatch_email::EmailPayload;

use super::AppState;

/// Add a new user to the database.
/// NOTE: This function is used to add the user
/// to the database with the minimum information
/// provided. This function creates an unverified
/// account for the client.
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
pub async fn insert(
    State(app_state): State<AppState>,
    Form(user): Form<NewUser>,
) -> DefaultResponse {
    // This is a default error message from a server in order not to
    // disclose some information that could be used to
    // destroy the work of servers.
    const SERVER_ERROR: &str = "Something went wrong on the server side";

    // Try to allocate a connection with the database from the pool.
    let mut connection = match app_state.pool.get().await {
        // The connection with the database has been
        // established successfully.
        Ok(connection) => connection,
        // An error occurred while establishing a connection
        // with the database.
        Err(error) => {
            eprintln!("{}", error);
            return DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            }; // end return
        } // end Err
    }; // end match

    // Check if the form is filled out properly.
    let (status, message) = is_valid_form(&user);

    // Deal with all possible result of the form validation.
    if status != StatusCode::OK {
        // The form has not passed validation.
        // Inform the client about it.
        return DefaultResponse {
            status_code: status,
            message: Some(message),
            redirect: None,
        }; // end return
    } // end if

    // Check if the user with the same email or phone number
    // already exists.

    // Check the phone number.
    match crate::schema::users::dsl::users
        .filter(crate::schema::users::columns::phone_number_code.eq(&user.phone_number_code))
        .filter(crate::schema::users::columns::phone_number.eq(&user.phone_number))
        .load::<User>(&mut connection)
        .await
    {
        // The data extraction has been successful.
        Ok(users) => {
            // Check that the array of users is empty.
            if !users.is_empty() {
                // The user already exists,
                // reject the request.
                return DefaultResponse {
                    status_code: StatusCode::UNAUTHORIZED,
                    message: Some("The user with this phone number already exists".to_string()),
                    redirect: None,
                }; // end return
            } // end if
        } // end Ok
        // An error occurred while extracting data from the database.
        Err(error) => {
            eprintln!("{}", error);
            return DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            }; // end return
        } // end Err
    } // end match

    // Check the email.
    match crate::schema::users::dsl::users
        .filter(crate::schema::users::columns::email.eq(&user.email))
        .load::<User>(&mut connection)
        .await
    {
        // The data extraction has been successful.
        Ok(users) => {
            // Check that the array of users is empty.
            if !users.is_empty() {
                // The user already exists,
                // reject the request.
                return DefaultResponse {
                    status_code: StatusCode::UNAUTHORIZED,
                    message: Some("The user with this email already exists".to_string()),
                    redirect: None,
                }; // end return
            } // end if
        } // end Ok
        // An error occurred while extracting data from the database.
        Err(error) => {
            eprintln!("{}", error);
            return DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            }; // end return
        } // end Err
    } // end match

    // Try to insert a user to the database.
    match diesel::insert_into(crate::schema::users::table)
        .values(&user)
        .get_result::<User>(&mut connection)
        .await
    {
        Ok(_) => (),
        Err(error) => {
            eprintln!("{}", error);
            return DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            }; // end return
        } // end Err
    }; // end match

    // Send an email to the new user if the user specified an email.
    if let Some(user_email) = user.email {
        // The email should be valid, send the email to the user.
        dispatch_email(
            EmailPayload {
                full_name: user.name,
                subject: "Subscription is activated".to_string(),
                email: user_email,
                message: "Welcome to Manuspect!".to_string(),
            }
            .into(),
        )
        .await;
    } // end if

    // Everything went successfully, return OK to the user.
    DefaultResponse {
        status_code: StatusCode::OK,
        message: Some("The subscription was successful!".to_string()),
        redirect: None,
    } // end DefaultResponse
} // fn insert

/// This function verifies that a form is filled out decently.
/// It returns a status (StatusCode), which indicates whether or not
/// the verification has been passed, and a message that
/// contains additional information about the result.
fn is_valid_form(user: &NewUser) -> (StatusCode, String) {
    // Validate username.
    if user.name.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            "The \"name\" field cannot be empty".to_string(),
        ); // end return
    } // end if

    // Validate email (if it is supplied).
    if let Some(email) = user.email.clone() {
        if !email.contains("@") {
            // Email has to contain "@" symbol.
            return (
                StatusCode::UNAUTHORIZED,
                "Email has to contain \"@\" symbol".to_string(),
            ); // end return
        } // end if
        if !email.contains(".") {
            // Email has to contain "." symbol.
            return (
                StatusCode::UNAUTHORIZED,
                "Email has to contain \".\" symbol".to_string(),
            ); // end return
        } // end if
    } // end if

    // Validate phone number code.
    if user.phone_number_code == 0 || user.phone_number_code > 999 {
        // The phone number code is invalid.
        return (
            StatusCode::UNAUTHORIZED,
            "The phone number code is invalid".to_string(),
        ); // end return
    } // end if

    // Validate phone number.
    if user.phone_number.len() < 4 {
        // The phone number is too short.
        return (
            StatusCode::UNAUTHORIZED,
            "The phone number is too short".to_string(),
        ); // end return
    } else {
        // Traverse all the symbols in a phone number and make
        // sure that they are all decimal digits.
        for symbol in user.phone_number.chars() {
            // Check if the current symbol is a valid digit.
            if !symbol.is_digit(10) {
                // This is not a valid decimal digit.
                return (
                    StatusCode::UNAUTHORIZED,
                    "The phone number is not valid".to_string(),
                ); // end return
            } // end if
        } // end for
    } // end if

    (StatusCode::OK, "".to_string())
} // end fn is_valid_form
