use crate::routes::dispatch_email::{EmailPayload, __path_dispatch_email};
use crate::routes::insert::__path_insert;
use crate::schema::users;
use crate::utils::responses::ResponseJson;
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
}

// This is a struct for inserting a user in a database.
#[derive(Insertable, Deserialize, ToSchema, Debug)]
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
}

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
    paths(insert, dispatch_email),
    components(schemas(NewUser, ResponseJson, EmailPayload))
)]
pub struct ApiDoc;
