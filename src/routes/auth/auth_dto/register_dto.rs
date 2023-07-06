use diesel::{AsChangeset, Insertable};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::schema::users;

// This is a struct for inserting a user in a database.
#[derive(Insertable, Deserialize, ToSchema, Debug, AsChangeset)]
#[diesel(table_name = users)]
pub struct RegisterUserDto {
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
} // end struct RegisterUserDto
