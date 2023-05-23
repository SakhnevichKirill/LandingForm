use axum::{http::StatusCode, Form};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::{
    models::{NewUser, User},
    schema::users::{self, dsl::*},
    utils::{
        database_functions::establish_connection, responses::DefaultResponse,
        security::hash_password,
    },
};

pub async fn insert(Form(user): Form<NewUser>) -> DefaultResponse {
    const SERVER_ERROR: &str = "Something went wrong on the server side";

    // Try to establish a connection with the database.
    let mut connection: PgConnection = match establish_connection() {
        Ok(connection) => connection,
        Err(error) => {
            eprintln!("{}", error);
            return DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            };
        }
    };

    match verify_form(&user) {
        Ok(_) => (),
        Err(err) => {
            match err {
                "USER_EXISTS" => {
                    return DefaultResponse {
                        status_code: StatusCode::UNAUTHORIZED,
                        message: Some("The user already exists".to_string()),
                        redirect: Some(
                            "http://127.0.0.1:5500/frontend/HTML/already_subscribed.html"
                                .to_string(),
                        ),
                    }
                }
                "TOO_SHORT_PASSWORD" => {
                    return DefaultResponse {
                        status_code: StatusCode::UNAUTHORIZED,
                        message: Some("The password is too short".to_string()),
                        redirect: Some(
                            "http://127.0.0.1:5500/frontend/HTML/too_short_password.html"
                                .to_string(),
                        ),
                    }
                }
                "INCORRECT_PHONE_NUMBER" => {
                    return DefaultResponse {
                        status_code: StatusCode::UNAUTHORIZED,
                        message: Some("The phone number is written in a wrong format".to_string()),
                        redirect: Some(
                            "http://127.0.0.1:5500/frontend/HTML/incorrect_phone_number.html"
                                .to_string(),
                        ),
                    }
                }
                error => {
                    eprintln!("{}", error);
                    return DefaultResponse {
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        message: Some(SERVER_ERROR.to_string()),
                        redirect: None,
                    };
                }
            };
        }
    }

    println!("Form is verified");

    // Hash user's password.
    let mut user = user;

    if let Some(passwrd) = user.password {
        user.password = match hash_password(passwrd).await {
            Ok(passwrd) => Some(passwrd),
            Err(error) => {
                eprintln!("{}", error);
                return DefaultResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: Some(SERVER_ERROR.to_string()),
                    redirect: None,
                };
            }
        }
        // println!("Hashed password: {}", user.password.clone().unwrap());
    }

    match diesel::insert_into(users::table)
        .values(user)
        .get_result::<User>(&mut connection)
    {
        Ok(_) => (),
        Err(error) => {
            eprintln!("{}", error);
            return DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            };
        }
    };

    DefaultResponse {
        status_code: StatusCode::OK,
        message: Some("The subscription was successful!".to_string()),
        redirect: Some("http://127.0.0.1:5500/frontend/HTML/subscribed.html".to_string()),
    }
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
