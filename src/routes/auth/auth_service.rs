use crate::{
    models::User,
    routes::AppState,
    schema::users::dsl,
    utils::{app_error::AppError, jwt::create_jwt, security::hash_password},
};
use axum::http::StatusCode;
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::RunQueryDsl;

use super::auth_dto::{
    login_dto::LoginUserDto, register_dto::RegisterUserDto, token_dto::TokenDto,
};

pub(super) struct AuthService;

impl AuthService {
    /// This function servers a registration endpoint.
    ///
    /// It receives a form, validates it, checks if the user
    /// already exists in the database. If so, then checks whether or
    /// not they are verified.
    ///
    /// If the user is not verified, then it
    /// updates their status in the database and inserts absent information
    /// about user in a database.
    ///
    /// Otherwise, the user is inserted in the database and their account
    /// is set to verified immediately.
    ///
    /// Form template:
    ///
    /// pub struct RegisterUserDto {
    ///     pub name: String,
    ///     pub email: Option<String>,
    ///     pub phone_number_code: i32,
    ///     pub phone_number: String,
    ///     pub password: Option<String>,
    /// }
    ///
    pub async fn register(
        app_state: AppState,
        mut user: RegisterUserDto,
    ) -> Result<TokenDto, AppError> {
        // This is a default error message from a server in order not to
        // disclose some information that could be used to
        // destroy the work of servers.
        const SERVER_ERROR: &str = "Something went wrong on the server side";

        // This variable is required to set up
        // some fields in the database that
        // are not expected from the client.
        //
        // This variable stores the user id of
        // the current client.
        let user_id: i32;

        // Get a database connection from the pool.
        let mut conn = match app_state.pool.get().await {
            // The connection was allocated successfully.
            Ok(conn) => conn,
            // An error occurred while allocating a connection.
            Err(error) => {
                eprintln!("{}", error);
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    SERVER_ERROR.to_string(),
                )); // end return
            } // end Err
        }; // end match

        // Check that the form is filled out in a proper way.
        let (passed, message) = Self::is_valid_form(&user);

        // Check if the form passed the verification.
        if !passed {
            // The form did not pass the verification.
            // Return an error response.
            return Err(AppError::new(StatusCode::UNAUTHORIZED, message));
        }

        // Get rid of unnecessary variables.
        drop(passed);
        drop(message);

        // Hash the user password.
        if let Ok(hashed_password) = hash_password(user.password.unwrap()).await {
            // The password was hashed successfully.
            user.password = Some(hashed_password);
        } else {
            // There was an error while hashing the password.
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                SERVER_ERROR.to_string(),
            )); // end return
        } // end if

        // Check if the user with the same phone number already exists.

        // Try to load the users with the provided phone number.
        let mut res: Vec<User> = match dsl::users
            .filter(crate::schema::users::columns::phone_number_code.eq(&user.phone_number_code))
            .filter(crate::schema::users::columns::phone_number.eq(&user.phone_number))
            .load::<User>(&mut conn)
            .await
        {
            Ok(res) => res,
            Err(error) => {
                eprintln!("{}", error);
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    SERVER_ERROR.to_string(),
                )); // end return
            } // end Err
        }; // end match

        // Check if a user was found.
        // NOTE: The logic of the program designed in such a way,
        // that the scenario in which there would be more than one
        // user with the same phone number is next to impossible.
        //
        // So, it might be considered to be guaranteed, that
        // there is only a unique user with a unique phone number.
        if res.len() > 0 {
            // A user with the provided phone number was found.
            //
            // Extract this user from the array.
            // NOTE: It is guaranteed that there is a unique user only
            // with a unique phone number.
            let cur_user = res.pop().unwrap();

            // Check if the user has already been verified.
            if cur_user.verified {
                // The user has already been verified, which means
                // they cannot be registered again.
                return Err(AppError::new(
                    StatusCode::UNAUTHORIZED,
                    "The user has already been registered".to_string(),
                )); // end return
            } // end if

            // Save the user id of the current client.
            user_id = cur_user.id;

            if diesel::update(crate::schema::users::table)
                .filter(crate::schema::users::columns::id.eq(user_id))
                .set(&user)
                .execute(&mut conn)
                .await
                .is_err()
            {
                // An error occurred while updating data in a database.
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    SERVER_ERROR.to_string(),
                )); // end return
            } // end if
        } else {
            // The user is absent in a database, so they
            // should be inserted from scratch.

            // Insert a user in a database.
            // Save the user id of the current client.
            user_id = if let Ok(inserted_user) = diesel::insert_into(crate::schema::users::table)
                .values(&user)
                .get_result::<User>(&mut conn)
                .await
            {
                // The user has been inserted successfully.
                inserted_user.id
            } else {
                // An error occurred while inserting data to a database.
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    SERVER_ERROR.to_string(),
                )); // end return
            }; // end if let
        } // end if

        // Assign the basic role to the user.
        if diesel::insert_into(crate::schema::users_roles::table)
            .values((
                crate::schema::users_roles::columns::user_id.eq(user_id),
                crate::schema::users_roles::role_id.eq(1),
            ))
            .execute(&mut conn)
            .await
            .is_err()
        {
            // An error occurred while inserting data in the database.
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                SERVER_ERROR.to_string(),
            )); // end return
        } // end if

        // The user is registered.
        // Now it is time to generate JWT for the user
        // and send it to them.

        // Generate JWT.
        let token = if let Some(token) = create_jwt() {
            // A token has been generated successfully.
            token
        } else {
            // An error occurred, while generating a token.
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                SERVER_ERROR.to_string(),
            )); // end return
        }; // end if let

        // Assign this JWT to the current client and
        // mark the current client as verified.
        // Insert it into the database.
        if diesel::update(crate::schema::users::table)
            .filter(crate::schema::users::columns::id.eq(user_id))
            .set((dsl::token.eq(&token), dsl::verified.eq(true)))
            .execute(&mut conn)
            .await
            .is_err()
        {
            // An error occurred while updating the data.
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                SERVER_ERROR.to_string(),
            )); // end return
        } // end if

        // Return JWT with success status.
        return Ok(TokenDto {
            access_token: Some(token),
            refresh_token: None, // TODO: implement refresh_token
        }); // end return
    } // fn register

    /// This function verifies that a form is filled out decently.
    /// It returns a status (bool), which indicates whether or not
    /// the verification has been passed, and a message that
    /// contains additional information about the result.
    fn is_valid_form(user: &RegisterUserDto) -> (bool, String) {
        // Validate username.
        if user.name.is_empty() {
            return (false, "The \"name\" field cannot be empty".to_string());
        } // end if

        // Validate email.
        if let Some(email) = user.email.clone() {
            if !email.contains("@") {
                // Email has to contain "@" symbol.
                return (false, "Email has to contain \"@\" symbol".to_string());
            } // end if
            if !email.contains(".") {
                // Email has to contain "." symbol.
                return (false, "Email has to contain \".\" symbol".to_string());
            } // end if
        } else {
            // Email cannot be empty.
            return (false, "Email cannot be empty".to_string());
        } // end if

        // Validate phone number code.
        if user.phone_number_code == 0 || user.phone_number_code > 999 {
            // The phone number code is invalid.
            return (false, "The phone number code is invalid".to_string());
        } // end if

        // Validate phone number.
        if user.phone_number.len() < 4 {
            // The phone number is too short.
            return (false, "The phone number is too short".to_string());
        } else {
            // Traverse all the symbols in a phone number and make
            // sure that they are all decimal digits.
            for symbol in user.phone_number.chars() {
                // Check if the current symbol is a valid digit.
                if !symbol.is_digit(10) {
                    // This is not a valid decimal digit.
                    return (false, "The phone number is not valid".to_string());
                } // end if
            } // end for
        } // end if

        // Validate password.
        // NOTE: The password must be required since now.
        if let Some(password) = user.password.clone() {
            // Check that the password meets the rules.
            // NOTE: The password rules are:
            //  1. Password length min 7, max 30 symbols.
            //  2. Password must contain ASCII characters only.

            // Check that the password length meets the requirements.
            if password.len() < 7 || password.len() > 30 {
                // The password length is out of boundaries.
                return (
                    false,
                    "The password length must be between 7 and 30 characters inclusive".to_string(),
                ); // end return
            } // end if

            // Check that the password contains ASCII characters only.
            //
            // Traverse all the password characters and check if there are
            // any invalid (non-ASCII) characters.
            for symbol in password.chars() {
                // Check if the current symbols is a valid ASCII character.
                if !symbol.is_ascii() {
                    // The current character is out of ASCII range.
                    (false, "The password must contain only a-z, A-Z, 0-9, !$%#> or some other ASCII characters only");
                } // end if
            } // end for
        } else {
            // The password cannot be absent.
            return (false, "The password cannot be absent".to_string());
        } // end if

        (true, "".to_string())
    } // end fn is_valid_form

    /// This is a function that serves login endpoint on the server.
    /// It receives a form filled out by the client and in case of
    /// success returns a web token that can be used for maintaining
    /// a session without logging in for a some time.
    ///
    /// Form template:
    ///
    /// pub struct LoginUserDto {
    ///     pub email: Option<String>,
    ///     pub phone_number_code: Option<i32>,
    ///     pub phone_number: Option<String>,
    ///     pub password: String,
    /// }
    ///
    pub async fn login(
        &self,
        app_state: AppState,
        user: LoginUserDto,
    ) -> Result<TokenDto, AppError> {
        // This is a default error message from a server in order not to
        // disclose some information that could be used to
        // destroy the work of servers.
        const SERVER_ERROR: &str = "Something went wrong on the server side";

        // User id is required to check if passwords match later in the code.
        let mut user_id: i32 = -1;
        let mut user_password: String = String::new();

        // Allocate a connection to the database from the pool.
        let mut conn = match app_state.pool.get().await {
            // The conn with the database has been
            // established successfully.
            Ok(conn) => conn,
            // An error occurred while establishing a conn
            // with the database.
            Err(error) => {
                eprintln!("{}", error);
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    SERVER_ERROR.to_string(),
                ));
            } // end Err
        }; // end match

        // Check if the user with the same email or phone number
        // already exists.

        // NOTE: A user is allowed to specify either an email or a phone number.

        // If the user specified the phone number.
        if user.phone_number_code.is_some() && user.phone_number.is_some() {
            // Check the phone number.
            match crate::schema::users::dsl::users
                .filter(
                    crate::schema::users::columns::phone_number_code
                        .eq(&user.phone_number_code.unwrap()),
                )
                .filter(crate::schema::users::columns::phone_number.eq(&user.phone_number.unwrap()))
                .load::<User>(&mut conn)
                .await
            {
                // The data extraction has been successful.
                Ok(users) => {
                    // Check that the array of users is empty.
                    if !users.is_empty() {
                        // Save the user id and password.
                        // NOTE: It is guaranteed that there might be
                        // the only user with a unique phone number.
                        user_id = users[0].id;
                        user_password = users[0].password.clone().unwrap();
                    } // end if
                } // end Ok
                // An error occurred while extracting data from the database.
                Err(error) => {
                    eprintln!("{}", error);
                    return Err(AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        SERVER_ERROR.to_string(),
                    )); // end return
                } // end Err
            } // end match
        } else if user.email.is_some() {
            // If the user specified email instead.

            // Check the email.
            match crate::schema::users::dsl::users
                .filter(crate::schema::users::columns::email.eq(&user.email))
                .load::<User>(&mut conn)
                .await
            {
                // The data extraction has been successful.
                Ok(users) => {
                    // Check that the array of users is empty.
                    if !users.is_empty() {
                        // Save the user id and password.
                        // NOTE: It is guaranteed that there might be
                        // the only user with a unique phone number.
                        user_id = users[0].id;
                        user_password = users[0].password.clone().unwrap();
                    } // end if
                } // end Ok
                // An error occurred while extracting data from the database.
                Err(error) => {
                    eprintln!("{}", error);
                    return Err(AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        SERVER_ERROR.to_string(),
                    )); // end return
                } // end Err
            } // end match
        } // end if

        // Hash the supplied password.
        if let Ok(hashed_password) = hash_password(user.password).await {
            // The password has been hashed successfully.

            // Compare two hashes.
            // If the hashes are identical, then the password is correct.
            if hashed_password == user_password {
                // The password is correct, the user is verified.
                if let Some(token) = create_jwt() {
                    // The JWT has been generated successfully.

                    // Assign this JWT to the client
                    // and insert the JWT into the database.
                    if diesel::update(crate::schema::users::table)
                        .filter(crate::schema::users::columns::id.eq(user_id))
                        .set(crate::schema::users::dsl::token.eq(&token))
                        .execute(&mut conn)
                        .await
                        .is_err()
                    {
                        // An error occurred while updating the information
                        // about client.
                        return Err(AppError::new(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            SERVER_ERROR.to_string(),
                        )); // end return
                    } // end if

                    // Return the token to the client.
                    return Ok(TokenDto {
                        access_token: Some(token),
                        refresh_token: None, // TODO: implement refresh_token
                    }); // end return
                } else {
                    // An error occurred while generating JWT.
                    return Err(AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        SERVER_ERROR.to_string(),
                    )); // end return
                } // end if
            } else {
                // The password is incorrect, the user is not verified.
                // NOTE: The user could specify the login incorrectly.
                // But for safety reasons the exact reason is not disclosed.
                return Err(AppError::new(
                    StatusCode::UNAUTHORIZED,
                    "The login or password or both are incorrect".to_string(),
                )); // end return
            } // end if
        } else {
            // An error occurred while hashing password.
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                SERVER_ERROR.to_string(),
            )); // end return
        } // end if
    } // fn login
}
