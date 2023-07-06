use crate::models::User;
use crate::routes::auth::auth_dto::register_dto::RegisterUserDto;
use crate::schema::users;
use axum::http::StatusCode;
use diesel::pg::PgConnection;
use diesel::prelude::*;

/// This function tries to insert a new user in a database.
pub fn insert_user(
    connection: &mut PgConnection,
    new_user: RegisterUserDto,
) -> Result<User, StatusCode> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connection)
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
} // end fn insert_user

/// This function tries to remove a user from a database.
pub fn delete_user(connection: &mut PgConnection, user_id: i32) -> Result<(), StatusCode> {
    diesel::delete(users::table)
        .filter(users::columns::id.eq(user_id))
        .execute(connection)
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|_res| ())
} // end fn delete_user
