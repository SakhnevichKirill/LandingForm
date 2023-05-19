use crate::schema::users;
use diesel::prelude::*;
use serde::Deserialize;

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
#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: Option<String>,
    pub phone_number_code: i32,
    pub phone_number: String,
    pub password: Option<String>,
}
