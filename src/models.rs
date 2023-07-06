use crate::routes::auth::{
    auth_controller::{__path_login, __path_register},
    auth_dto::{login_dto::LoginUserDto, register_dto::RegisterUserDto, token_dto::TokenDto},
};
use crate::routes::dispatch_email::{EmailPayload, __path_dispatch_email};
use crate::routes::insert::__path_insert;
use crate::utils::responses::DefaultResponseJson;
use diesel::prelude::*;
use utoipa::OpenApi;

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
    components(schemas(RegisterUserDto, DefaultResponseJson, EmailPayload, DefaultResponseJson, LoginUserDto, TokenDto))
)] // end openapi
pub struct ApiDoc;
