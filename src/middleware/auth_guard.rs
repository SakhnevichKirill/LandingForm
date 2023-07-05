// This file contains middleware that prevents unauthorized
// access to some resources.

use std::collections::{HashMap, HashSet};

use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

use std::sync::{Arc, RwLock};

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

    // Try to load the user by their token with their roles.
    let mut users: Vec<(User, String)> = match dsl::users
        .filter(crate::schema::users::columns::token.eq(&token))
        .inner_join(crate::schema::users_roles::table.inner_join(crate::schema::roles::table))
        .select((
            crate::schema::users::all_columns,
            crate::schema::roles::title,
        ))
        .load::<(User, String)>(&mut conn)
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

    // Get a single user with all their roles assigned.
    let user = users.pop().unwrap();
    let mut user: (User, Vec<String>) = (user.0, vec![user.1]);

    // Assign all user roles to the client.
    for (_, role) in users.into_iter() {
        user.1.push(role);
    } // end for

    // Check if the user has permission to access
    // the route they want to access.
    if !has_permission(&user.1, req.uri().path(), app_state.allowed_roles) {
        // The user is not allowed to access the route.
        return Err(DefaultResponse {
            status_code: StatusCode::UNAUTHORIZED,
            message: Some("You do not have permissions to access this page".to_string()),
            redirect: None,
        }); // end return
    } // end if

    // Add the user to the request.
    // NOTE: It is guaranteed that there is one user only in the array.
    req.extensions_mut().insert(user);

    // Proceed to the request.
    Ok(next.run(req).await)
} // end fn auth_guard

/// This function checks if the user is allowed to access the
/// route they want to access.
fn has_permission(
    roles: &Vec<String>,
    path: &str,
    allowed_routes: Arc<RwLock<HashMap<String, HashSet<String>>>>,
) -> bool {
    // Get the exclusive rights on variable manipulation.
    let allowed_routes = (*allowed_routes)
        .write()
        .expect("An error occurred while unwrapping RwLock for writing");

    // This variable is required to check if the
    // user has access to all the parent routes of the request.
    let mut assembled_path = String::new();

    // Assemble the entire path and on each step check
    // if the user has access to the parent path.
    for part in path.split("/") {
        // Add the next part of the path.
        // If the part is empty, then there is no need to add it
        // to the path.
        //
        // NOTE: This logic assumes that the root path "/" is UNPROTECTED.
        if !part.is_empty() {
            assembled_path.push_str(&format!("/{}", part));
        } // end if

        println!("{}", assembled_path);

        // Check if the route is added to the set of protected routes.
        if let Some(allowed_roles) = allowed_routes.get(&assembled_path) {
            // Check if the user can access the route.
            let mut allowed = false;
            // Traverse all the roles the client has.
            for role in roles {
                // Check if the current role grands the client
                // permission to access the route.
                if allowed_roles.contains(role) {
                    // A user can access the route.
                    allowed = true;
                    break;
                } // end if
            } // end for

            // Check if the client has at least one
            // of the required roles assigned.
            if !allowed {
                // The user is not allowed to
                // access the route.
                return false;
            } // end if
        } // end if
    } // end for

    // All checks have been performed,
    // a user can access the route.
    true
} // end fn has_permission
