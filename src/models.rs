use crate::routes::auth::login::__path_login;
use crate::routes::auth::register::__path_register;
use crate::routes::dispatch_email::{EmailPayload, __path_dispatch_email};
use crate::routes::insert::__path_insert;
use crate::schema::users;
use crate::utils::responses::{DefaultResponseJson, LoginResponseJson};
use diesel::prelude::*;
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

/// This is a struct for retrieving a user from a database.
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub phone_number_code: i32,
    pub phone_number: String,
    pub password: Option<String>,
    pub token: Option<String>,
    pub verified: bool,
} // end struct User

// This is a struct for inserting a user in a database.
#[derive(Insertable, Deserialize, ToSchema, Debug, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[schema(example = "John")]
    pub name: String,
    #[schema(example = "john@gmail.com")]
    pub email: Option<String>,
    #[schema(example = 1)]
    pub phone_number_code: i32,
    #[schema(example = "9999999999")]
    pub phone_number: String,
    #[schema(example = "qwerty123")]
    pub password: Option<String>,
} // end struct NewUser

/// This struct represents an existing user that already has
/// a verified account and just wants to log in in the system.
#[derive(Deserialize, ToSchema)]
pub struct LoginUser {
    #[schema(example = "john@gmail.com")]
    pub email: Option<String>,
    #[schema(example = 1)]
    pub phone_number_code: Option<i32>,
    #[schema(example = "9999999999")]
    pub phone_number: Option<String>,
    #[schema(example = "qwerty123")]
    pub password: String,
} // end struct LoginUser

// This is a swagger REST API documentation generator.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Landing page",
        license(
            name = "MIT",
            url = "https://opensource.org/license/mit/"
        ),
        description = "This API might be used for registering and authorizing users on a website.",
    ),
    servers(
        (url = "http://localhost", description = "This is a local server for testing"),
        (url = "http://95.165.88.39", description = "This is a remote server for testing"),
    ),
    paths(insert, dispatch_email, register, login),
    components(schemas(NewUser, DefaultResponseJson, EmailPayload, DefaultResponseJson, LoginUser, LoginResponseJson))
)] // end openapi
pub struct ApiDoc;
