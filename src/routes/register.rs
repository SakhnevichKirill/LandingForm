use crate::{
    models::User,
    schema::users::dsl,
    utils::{jwt::create_jwt, security::hash_password},
};
use axum::Form;
use axum::{extract::State, http::StatusCode};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::RunQueryDsl;

use crate::{models::NewUser, utils::responses::LoginResponse};

use super::AppState;

/// This function servers a registration endpoint.
///
/// It receives a form, validates it, checks if the user
/// already exists in the database. If so, then checks whether or
/// not they are verified.
///
/// If the user is not verified, then it
/// updates their status in the database and inserts absent information
/// about user in a database.
///
/// Otherwise, the user is inserted in the database and their account
/// is set to verified immediately.
///
/// Form template:
///
/// pub struct NewUser {
///     pub name: String,
///     pub email: Option<String>,
///     pub phone_number_code: i32,
///     pub phone_number: String,
///     pub password: Option<String>,
/// }
///
#[utoipa::path(
    post,
    tag = "Registration",
    path = "/register",
    request_body(content = NewUser, description = "A filled out registration form", content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = StatusCode::OK, description = "A user was registered successfully", body = LoginResponse, example = json!("{\"message\": \"SUCCESSFULLY AUTHORIZED\", \"token\": \"293u5429*2%23$#@jlasdfl\"}")),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "There was an internal error on the server side (The user is not inserted into the database in this case)", body = ResponseJson, example = json!("{\"message\": \"An error occurred on the server side. Email could not be sent.\", \"redirect\": null}")),
        (status = StatusCode::UNAUTHORIZED, description = "In this case the user has whether made a mistake while filling out the form, or they are already registered")
    )
)]
pub async fn register(
    State(app_state): State<AppState>,
    Form(mut user): Form<NewUser>,
) -> LoginResponse {
    // This is a default error message from a server in order not to
    // disclose some information that could be used to
    // destroy the work of servers.
    const SERVER_ERROR: &str = "Something went wrong on the server side";

    // This variable is required to set up
    // some fields in the database that
    // are not expected from the client.
    //
    // This variable stores the user id of
    // the current client.
    let user_id: i32;

    // Get a database connection from the pool.
    let mut conn = match app_state.pool.get().await {
        // The connection was allocated successfully.
        Ok(conn) => conn,
        // An error occurred while allocating a connection.
        Err(error) => {
            eprintln!("{}", error);
            return LoginResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: SERVER_ERROR.to_string(),
                token: None,
            }; // end return
        } // end Err
    }; // end match

    // Check that the form is filled out in a proper way.
    let (passed, message) = is_valid_form(&user);

    // Check if the form passed the verification.
    if !passed {
        // The form did not pass the verification.
        // Return an error response.
        return LoginResponse {
            status_code: StatusCode::UNAUTHORIZED,
            message: message,
            token: None,
        };
    }

    // Get rid of unnecessary variables.
    drop(passed);
    drop(message);

    // Hash the user password.
    if let Ok(hashed_password) = hash_password(user.password.unwrap()).await {
        // The password was hashed successfully.
        user.password = Some(hashed_password);
    } else {
        // There was an error while hashing the password.
        return LoginResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: SERVER_ERROR.to_string(),
            token: None,
        }; // end return
    } // end if

    // Check if the user with the same phone number already exists.

    // Try to load the users with the provided phone number.
    let mut res: Vec<User> = match dsl::users
        .filter(crate::schema::users::columns::phone_number_code.eq(&user.phone_number_code))
        .filter(crate::schema::users::columns::phone_number.eq(&user.phone_number))
        .load::<User>(&mut conn)
        .await
    {
        Ok(res) => res,
        Err(error) => {
            eprintln!("{}", error);
            return LoginResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: SERVER_ERROR.to_string(),
                token: None,
            }; // end return
        } // end Err
    }; // end match

    // Check if a user was found.
    // NOTE: The logic of the program designed in such a way,
    // that the scenario in which there would be more than one
    // user with the same phone number is next to impossible.
    //
    // So, it might be considered to be guaranteed, that
    // there is only a unique user with a unique phone number.
    if res.len() > 0 {
        // A user with the provided phone number was found.
        //
        // Extract this user from the array.
        // NOTE: It is guaranteed that there is a unique user only
        // with a unique phone number.
        let cur_user = res.pop().unwrap();

        // Check if the user has already been verified.
        if cur_user.verified {
            // The user has already been verified, which means
            // they cannot be registered again.
            return LoginResponse {
                status_code: StatusCode::UNAUTHORIZED,
                message: "The user has already been registered".to_string(),
                token: None,
            }; // end return
        } // end if

        // Save the user id of the current client.
        user_id = cur_user.id;

        if diesel::update(crate::schema::users::table)
            .filter(crate::schema::users::columns::id.eq(user_id))
            .set(&user)
            .execute(&mut conn)
            .await
            .is_err()
        {
            // An error occurred while updating data in a database.
            return LoginResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: SERVER_ERROR.to_string(),
                token: None,
            }; // end return
        } // end if
    } else {
        // The user is absent in a database, so they
        // should be inserted from scratch.

        // Insert a user in a database.
        // Save the user id of the current client.
        user_id = if let Ok(inserted_user) = diesel::insert_into(crate::schema::users::table)
            .values(&user)
            .get_result::<User>(&mut conn)
            .await
        {
            // The user has been inserted successfully.
            inserted_user.id
        } else {
            // An error occurred while inserting data to a database.
            return LoginResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: SERVER_ERROR.to_string(),
                token: None,
            }; // end return
        }; // end if let
    } // end if

    // The user is registered.
    // Now it is time to generate JWT for the user
    // and send it to them.

    // Generate JWT.
    let token = if let Some(token) = create_jwt() {
        // A token has been generated successfully.
        token
    } else {
        // An error occurred, while generating a token.
        return LoginResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: SERVER_ERROR.to_string(),
            token: None,
        }; // end return
    }; // end if let

    // Assign this JWT to the current client and
    // mark the current client as verified.
    // Insert it into the database.
    if diesel::update(crate::schema::users::table)
        .filter(crate::schema::users::columns::id.eq(user_id))
        .set((dsl::token.eq(&token), dsl::verified.eq(true)))
        .execute(&mut conn)
        .await
        .is_err()
    {
        // An error occurred while updating the data.
        return LoginResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: SERVER_ERROR.to_string(),
            token: None,
        }; // end return
    } // end if

    // Return JWT with success status.
    return LoginResponse {
        status_code: StatusCode::OK,
        message: "SUCCESSFUL AUTHORIZATION".to_string(),
        token: Some(token),
    }; // end return
} // fn register

/// This function verifies that a form is filled out decently.
/// It returns a status (bool), which indicates whether or not
/// the verification has been passed, and a message that
/// contains additional information about the result.
fn is_valid_form(user: &NewUser) -> (bool, String) {
    // Validate username.
    if user.name.is_empty() {
        return (false, "The \"name\" field cannot be empty".to_string());
    } // end if

    // Validate email.
    if let Some(email) = user.email.clone() {
        if !email.contains("@") {
            // Email has to contain "@" symbol.
            return (false, "Email has to contain \"@\" symbol".to_string());
        } // end if
        if !email.contains(".") {
            // Email has to contain "." symbol.
            return (false, "Email has to contain \".\" symbol".to_string());
        } // end if
    } else {
        // Email cannot be empty.
        return (false, "Email cannot be empty".to_string());
    } // end if

    // Validate phone number code.
    if user.phone_number_code == 0 || user.phone_number_code > 999 {
        // The phone number code is invalid.
        return (false, "The phone number code is invalid".to_string());
    } // end if

    // Validate phone number.
    if user.phone_number.len() < 4 {
        // The phone number is too short.
        return (false, "The phone number is too short".to_string());
    } else {
        // Traverse all the symbols in a phone number and make
        // sure that they are all decimal digits.
        for symbol in user.phone_number.chars() {
            // Check if the current symbol is a valid digit.
            if !symbol.is_digit(10) {
                // This is not a valid decimal digit.
                return (false, "The phone number is not valid".to_string());
            } // end if
        } // end for
    } // end if

    // Validate password.
    // NOTE: The password must be required since now.
    if let Some(password) = user.password.clone() {
        // Check that the password meets the rules.
        // NOTE: The password rules are:
        //  1. Password length min 7, max 30 symbols.
        //  2. Password must contain ASCII characters only.

        // Check that the password length meets the requirements.
        if password.len() < 7 || password.len() > 30 {
            // The password length is out of boundaries.
            return (
                false,
                "The password length must be between 7 and 30 characters inclusive".to_string(),
            ); // end return
        } // end if

        // Check that the password contains ASCII characters only.
        //
        // Traverse all the password characters and check if there are
        // any invalid (non-ASCII) characters.
        for symbol in password.chars() {
            // Check if the current symbols is a valid ASCII character.
            if !symbol.is_ascii() {
                // The current character is out of ASCII range.
                (false, "The password must contain only a-z, A-Z, 0-9, !$%#> or some other ASCII characters only");
            } // end if
        } // end for
    } else {
        // The password cannot be absent.
        return (false, "The password cannot be absent".to_string());
    } // end if

    (true, "".to_string())
} // end fn is_valid_form
