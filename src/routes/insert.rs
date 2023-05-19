use axum::{http::StatusCode, response::Redirect, Form};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    models::{NewUser, User},
    schema::users::{self, dsl::*},
    utils::{database_functions::establish_connection, security::hash_password},
};

pub async fn insert(Form(user): Form<NewUser>) -> Result<Redirect, StatusCode> {
    let connection = &mut establish_connection()?;

    match verify_form(&user) {
        Ok(_) => (),
        Err(err) => {
            match err {
                "USER_EXISTS" => {
                    return Ok(Redirect::to(
                        "http://127.0.0.1:5500/frontend/HTML/already_subscribed.html",
                    ))
                }
                "TOO_SHORT_PASSWORD" => {
                    return Ok(Redirect::to(
                        "http://127.0.0.1:5500/frontend/HTML/too_short_password.html",
                    ))
                }
                "INCORRECT_PHONE_NUMBER" => {
                    return Ok(Redirect::to(
                        "http://127.0.0.1:5500/frontend/HTML/incorrect_phone_number.html",
                    ))
                }
                _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            };
        }
    }

    println!("Form is verified");

    // Hash user's password.
    let mut user = user;

    if let Some(passwrd) = user.password {
        user.password = Some(hash_password(passwrd).await.map_err(|error| {
            println!("{}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?);
        // println!("Hashed password: {}", user.password.clone().unwrap());
    }

    diesel::insert_into(users::table)
        .values(user)
        .get_result::<User>(connection)
        .map_err(|error| {
            println!("{}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Redirect::to(
        "http://127.0.0.1:5500/frontend/HTML/subscribed.html",
    ))
}

fn verify_form(user: &NewUser) -> Result<(), &'static str> {
    // Establish a new connection with the database.
    let connection = &mut establish_connection().map_err(|error| {
        println!("{}", error);
        "INTERNAL_SERVER_ERROR"
    })?;

    // Check that the phone number is a number.
    let phone_num = user.phone_number.clone();

    phone_num
        .parse::<i64>()
        .map_err(|_error| "INCORRECT_PHONE_NUMBER")?;

    // Check the length of the phone number.
    if phone_num.len() != 10 {
        return Err("INCORRECT_PHONE_NUMBER");
    }

    // Make sure that the user with the same phone number does not exist.
    let result: Vec<User> = users
        .filter(users::columns::phone_number_code.eq(user.phone_number_code))
        .filter(users::columns::phone_number.eq(&user.phone_number))
        .load::<User>(connection)
        .map_err(|error| {
            println!("{}", error);
            "INTERNAL_SERVER_ERROR"
        })?;

    // If a user was found, then this user already exists.
    if result.len() > 0 {
        return Err("USER_EXISTS");
    }

    if let Some(passwrd) = &user.password {
        if passwrd.len() < 7 {
            return Err("TOO_SHORT_PASSWORD");
        }
    }

    Ok(())
}
