use axum::{http::StatusCode, Form};

use crate::{
    models::{LoginUser, User},
    utils::{
        database_functions::establish_connection, jwt::create_jwt, responses::LoginResponse,
        security::hash_password,
    },
};

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

/// This is a function that serves login endpoint on the server.
/// It receives a form filled out by the client and in case of
/// success returns a web token that can be used for maintaining
/// a session without logging in for a some time.
///
/// Form template:
///
/// pub struct LoginUser {
///     pub email: Option<String>,
///     pub phone_number_code: Option<i32>,
///     pub phone_number: Option<String>,
///     pub password: String,
/// }
///
#[utoipa::path(
    post,
    tag = "Login",
    path = "/login",
    request_body(content = LoginUser, description = "A filled out login form", content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = StatusCode::OK, description = "A user has logged in successfully", body = LoginResponse, example = json!("{\"message\": \"SUCCESSFULLY AUTHORIZED\", \"token\": \"293u5429*2%23$#@jlasdfl\"}")),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "There was an internal error on the server side (The user is not inserted into the database in this case)", body = ResponseJson, example = json!("{\"message\": \"An error occurred on the server side. Email could not be sent.\", \"redirect\": null}")),
        (status = StatusCode::UNAUTHORIZED, description = "In this case the user has either made a mistake while filling out the form. \
        E.g. they could specify login or password or both in a wrong way")
    )
)]
pub async fn login(Form(user): Form<LoginUser>) -> LoginResponse {
    // This is a default error message from a server in order not to
    // disclose some information that could be used to
    // destroy the work of servers.
    const SERVER_ERROR: &str = "Something went wrong on the server side";

    // User id is required to check if passwords match later in the code.
    let mut user_id: i32 = -1;
    let mut user_password: String = String::new();

    // Try to establish a connection with the database.
    let mut connection = match establish_connection() {
        // The connection with the database has been
        // established successfully.
        Ok(connection) => connection,
        // An error occurred while establishing a connection
        // with the database.
        Err(error) => {
            eprintln!("{}", error);
            return LoginResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: SERVER_ERROR.to_string(),
                token: None,
            }; // end return
        } // end Err
    }; // end match

    // Check if the user with the same email or phone number
    // already exists.

    // NOTE: A user is allowed to specify either an email or a phone number.

    // If the user specified the phone number.
    if user.phone_number_code.is_some() && user.phone_number.is_some() {
        // Check the phone number.
        match crate::schema::users::dsl::users
            .filter(
                crate::schema::users::columns::phone_number_code
                    .eq(&user.phone_number_code.unwrap()),
            )
            .filter(crate::schema::users::columns::phone_number.eq(&user.phone_number.unwrap()))
            .load::<User>(&mut connection)
        {
            // The data extraction has been successful.
            Ok(users) => {
                // Check that the array of users is empty.
                if !users.is_empty() {
                    // Save the user id and password.
                    // NOTE: It is guaranteed that there might be
                    // the only user with a unique phone number.
                    user_id = users[0].id;
                    user_password = users[0].password.clone().unwrap();
                } // end if
            } // end Ok
            // An error occurred while extracting data from the database.
            Err(error) => {
                eprintln!("{}", error);
                return LoginResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: SERVER_ERROR.to_string(),
                    token: None,
                }; // end return
            } // end Err
        } // end match
    } else if user.email.is_some() {
        // If the user specified email instead.

        // Check the email.
        match crate::schema::users::dsl::users
            .filter(crate::schema::users::columns::email.eq(&user.email))
            .load::<User>(&mut connection)
        {
            // The data extraction has been successful.
            Ok(users) => {
                // Check that the array of users is empty.
                if !users.is_empty() {
                    // Save the user id and password.
                    // NOTE: It is guaranteed that there might be
                    // the only user with a unique phone number.
                    user_id = users[0].id;
                    user_password = users[0].password.clone().unwrap();
                } // end if
            } // end Ok
            // An error occurred while extracting data from the database.
            Err(error) => {
                eprintln!("{}", error);
                return LoginResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: SERVER_ERROR.to_string(),
                    token: None,
                }; // end return
            } // end Err
        } // end match
    } // end if

    // Hash the supplied password.
    if let Ok(hashed_password) = hash_password(user.password).await {
        // The password has been hashed successfully.

        // Compare two hashes.
        // If the hashes are identical, then the password is correct.
        if hashed_password == user_password {
            // The password is correct, the user is verified.
            if let Some(token) = create_jwt() {
                // The JWT has been generated successfully.

                // Assign this JWT to the client
                // and insert the JWT into the database.
                if diesel::update(crate::schema::users::table)
                    .filter(crate::schema::users::columns::id.eq(user_id))
                    .set(crate::schema::users::dsl::token.eq(&token))
                    .execute(&mut connection)
                    .is_err()
                {
                    // An error occurred while updating the information
                    // about client.
                    return LoginResponse {
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        message: SERVER_ERROR.to_string(),
                        token: None,
                    }; // end return
                } // end if

                // Return the token to the client.
                return LoginResponse {
                    status_code: StatusCode::OK,
                    message: "SUCCESSFUL AUTHORIZATION".to_string(),
                    token: Some(token),
                }; // end return
            } else {
                // An error occurred while generating JWT.
                return LoginResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: SERVER_ERROR.to_string(),
                    token: None,
                }; // end return
            } // end if
        } else {
            // The password is incorrect, the user is not verified.
            // NOTE: The user could specify the login incorrectly.
            // But for safety reasons the exact reason is not disclosed.
            return LoginResponse {
                status_code: StatusCode::UNAUTHORIZED,
                message: "The login or password or both are incorrect".to_string(),
                token: None,
            }; // end return
        } // end if
    } else {
        // An error occurred while hashing password.
        return LoginResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: SERVER_ERROR.to_string(),
            token: None,
        }; // end return
    } // end if
} // fn login
