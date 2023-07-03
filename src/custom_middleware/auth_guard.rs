// This file contains middleware that prevents unauthorized
// access to some resources.

use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::RunQueryDsl;

use crate::{
    models::User,
    routes::AppState,
    schema::users::dsl,
    utils::{jwt::is_valid_jwt, responses::DefaultResponse},
};

/// This function is middleware that protects some endpoints from unauthorized
/// access.
pub async fn auth_guard<B>(
    State(app_state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, DefaultResponse> {
    // This is a default error message from a server in order not to
    // disclose some information that could be used to
    // destroy the work of servers.
    const SERVER_ERROR: &str = "Something went wrong on the server side";

    // Load token from the provided header.
    let token = token.token().to_owned();

    // Try to allocate a connection to the database from the pool.
    let mut conn = match app_state.pool.get().await {
        Ok(conn) => conn,
        Err(error) => {
            eprintln!("{}", error);
            return Err(DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            }); // end return
        } // end Err
    }; // end match// end establish_connection()

    // Try to load the user by their token.
    let mut users: Vec<User> = match dsl::users
        .filter(crate::schema::users::columns::token.eq(&token))
        .load::<User>(&mut conn)
        .await
    {
        // Everything went well and the database provided a response.
        Ok(users) => users,
        // An error occurred, while retrieving data from the database.
        Err(error) => {
            eprintln!("{}", error);
            return Err(DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            }); // end return
        } // end Err
    }; // end match

    // Check if the user was found in the database.
    if users.is_empty() {
        // The user with such a token does not exist.
        return Err(DefaultResponse {
            status_code: StatusCode::UNAUTHORIZED,
            message: Some("You are not authorized, please log in".to_string()),
            redirect: None,
        }); // end return
    } // end if

    // NOTE: It can be assumed, that there is at most
    // one user that could be found in the database.

    // Get the result of token validation.
    let (res, message) = is_valid_jwt(&token);

    // Check if the token is still valid.
    if !res {
        // The token is invalid.
        return Err(DefaultResponse {
            status_code: StatusCode::UNAUTHORIZED,
            message: Some(message),
            redirect: None,
        }); // end return
    } // end if

    // Verification has been passed successfully.

    // Add the user to the request.
    // NOTE: It is guaranteed that there is one user only in the array.
    req.extensions_mut().insert(users.pop());

    // Proceed to the request.
    Ok(next.run(req).await)
} // end fn auth_guard
