// This file contains all the functionality for sending
// emails.

use axum::{http::StatusCode, Json};
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;
use serde::Deserialize;
use std::env;
use utoipa::ToSchema;

use crate::utils::responses::DefaultResponse;

#[derive(Deserialize, ToSchema)]
pub struct EmailPayload {
    #[schema(example = "John Johnson")]
    pub full_name: String,
    #[schema(example = "Login confirmation")]
    pub subject: String,
    #[schema(example = "johnjohnson@gmail.com")]
    pub email: String,
    #[schema(example = "This is a message from a server.")]
    pub message: String,
}

/// Send an email to a user.
///
#[utoipa::path(
    post,
    tag = "Email",
    path = "/dispatch_email",
    request_body(content = EmailPayload, description = "Some information about the email sent", content_type = "application/json"),
    responses(
        (status = StatusCode::OK, description = "The email was sent successfully", body = ResponseJson, example = json!("{\"message\": \"The email was sent successfully!\", \"redirect\": \"http://localhost/success.html\"}")),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "There was an internal error on the server side (Email is not sent in this case)", body = ResponseJson, example = json!("{\"message\": \"An error occurred on the server side. Email could not be sent.\", \"redirect\": null}")),
    )
)]
pub async fn dispatch_email(Json(payload): Json<EmailPayload>) -> DefaultResponse {
    const SERVER_ERROR: &str = "Something went wrong on the server side";

    // Destructure the HTTP request body.
    let EmailPayload {
        full_name,
        subject,
        email,
        message,
    } = &payload;

    // Construct email config.
    let from_address = String::from("Manuspect <manuspect.prod@gmail.com>");
    let to_address = format!("{full_name} <{email}>");
    let reply_to = String::from("Manuspect <manuspect.prod@gmail.com>");
    let email_subject = subject;

    let email = Message::builder()
        .from(from_address.parse().unwrap())
        .reply_to(reply_to.parse().unwrap())
        .to(to_address.parse().unwrap())
        .subject(email_subject)
        .body(String::from(message))
        .unwrap();

    let creds = Credentials::new(
        match env::var("SMTP_USERNAME") {
            Ok(var) => var,
            Err(error) => {
                eprintln!("{}", error);
                return DefaultResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: Some(SERVER_ERROR.to_string()),
                    redirect: None,
                };
            }
        },
        match env::var("SMTP_PASSWORD") {
            Ok(var) => var,
            Err(error) => {
                eprintln!("{}", error);
                return DefaultResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: Some(SERVER_ERROR.to_string()),
                    redirect: None,
                };
            }
        },
    );

    // Open a remote connection to SMTP server.
    let mailer = match SmtpTransport::relay(match &env::var("SMTP_HOST") {
        Ok(var) => var,
        Err(error) => {
            eprintln!("{}", error);
            return DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            };
        }
    }) {
        Ok(res) => res,
        Err(error) => {
            eprintln!("{}", error);
            return DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            };
        }
    }
    .credentials(creds)
    .build();

    // Send the email.
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(error) => {
            eprintln!("{}", error);
            return DefaultResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: Some(SERVER_ERROR.to_string()),
                redirect: None,
            };
        }
    }

    DefaultResponse {
        status_code: StatusCode::OK,
        message: Some("The email was sent successfully!".to_string()),
        redirect: None,
    }
} // fn dispatch_email
