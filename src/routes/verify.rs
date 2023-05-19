use super::super::models::*;
use axum::{extract::Form, http::StatusCode, response::Redirect};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use crate::{
    schema::users::{self, dsl::*},
    utils::{database_functions::establish_connection, security::hash_password},
};

#[derive(Deserialize)]
pub struct SignIn {
    login: String,
    password: String,
}

/// This function verifies if a user exists in a database.
pub async fn verify(Form(user): Form<SignIn>) -> Result<Redirect, StatusCode> {
    // Try to establish a connection with the database.
    let connection = &mut establish_connection().map_err(|error| {
        println!("{}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let result: Vec<User>;

    let hashed_password = hash_password(user.password).await?;

    // Check if there is a login by a phone number.
    if user.login.starts_with("+") {
        // It is most likely to be a phone number

        // Split the phone number into phone number code and phone number.
        let mut phone_number_string_vec = user
            .login
            .split(" ")
            .map(|val| val.to_string())
            .collect::<Vec<String>>();

        let phone_num_code: i32;
        let phone_num: i64;

        // Check if there are not 2 elements, then it is definitely an error.
        if phone_number_string_vec.len() != 2 {
            return Ok(Redirect::to(
                "http://127.0.0.1:5500/frontend/HTML/unauthorized.html",
            ));
        }

        // Remove "+" sign from the phone number.
        phone_number_string_vec[0] = phone_number_string_vec[0].replacen("+", "", 1);

        // Try to convert the first element to a number.
        match phone_number_string_vec[0].parse::<i32>() {
            Ok(val) => phone_num_code = val,
            Err(_) => {
                return Ok(Redirect::to(
                    "http://127.0.0.1:5500/frontend/HTML/unauthorized.html",
                ))
            }
        }

        // Try to convert the second element to a number.
        match phone_number_string_vec[1].parse::<i64>() {
            Ok(val) => phone_num = val,
            Err(_) => {
                return Ok(Redirect::to(
                    "http://127.0.0.1:5500/frontend/HTML/unauthorized.html",
                ))
            }
        }

        result = users
            .filter(users::columns::phone_number_code.eq(phone_num_code))
            .filter(users::columns::phone_number.eq(phone_num.to_string()))
            .filter(users::columns::password.eq(hashed_password))
            .load(connection)
            .map_err(|error| {
                println!("{}", error);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    } else {
        // It is most likely to be an email.

        println!("Checking email");
        println!("{}\n{}", user.login, hashed_password);
        result = users
            .filter(users::columns::email.eq(user.login))
            .filter(users::columns::password.eq(hashed_password))
            .load(connection)
            .map_err(|error| {
                println!("{}", error);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    // If there are no results, then the user whether does not exist or entered
    // a wrong password or login.
    if result.len() == 0 {
        return Ok(Redirect::to(
            "http://127.0.0.1:5500/frontend/HTML/unauthorized.html",
        ));
    }

    Ok(Redirect::to(
        "http://127.0.0.1:5500/frontend/HTML/authorized.html",
    ))
}
