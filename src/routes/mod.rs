pub mod dispatch_email;
mod index;
pub mod insert;
pub mod login;
pub mod register;
pub mod verify;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::custom_middleware::{
    auth_guard::auth_guard,
    metrics_collector::{metrics_collector, metrics_display},
};
use crate::models::ApiDoc;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use dispatch_email::dispatch_email;
use index::index;
use insert::insert;
use login::login;
use register::register;
use verify::verify;

/// This function creates a router with routes, middleware, layers and so on.
pub async fn create_routes() -> Router {
    Router::new()
        .route("/dispatch_email", post(dispatch_email))
        .route("/metrics", get(metrics_display))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .layer(middleware::from_fn(auth_guard))
        .route("/", get(index))
        .route("/verify", post(verify))
        .route("/insert", post(insert))
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(middleware::from_fn(metrics_collector))
}

/// These are endpoint tests
///
#[cfg(test)]
mod tests {
    use crate::models::NewUser;

    use super::*;
    use axum::{body::Body, http::Request};
    use hyper;
    use serde::{Deserialize, Serialize};
    use urlencoding::encode;

    const SERVER_ADDR: &str = "127.0.0.1:8181";

    // A response body template.
    #[derive(Deserialize)]
    struct ResponseBody {
        message: Option<String>,
        _redirect: Option<String>,
    }

    /// This is a helper function that sets up a mock server.
    /// It also returns a task with the server, so it would
    /// be feasible to kill it after finishing testing.
    async fn setup_server() -> tokio::task::JoinHandle<()> {
        // Get router.
        let app = create_routes().await;

        // Run a fake Axum server.
        let server = tokio::spawn(async move {
            axum::Server::bind(&SERVER_ADDR.parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        });

        // Give some time to a server to become available.
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        server
    }

    /// Test an endpoint that is responsible for sending emails.
    #[tokio::test]
    async fn dispatch_email() {
        // A request body template.
        #[derive(Serialize)]
        struct RequestBody {
            email: String,
            full_name: String,
            message: String,
            subject: String,
        }

        // Data to send.
        let json_data: RequestBody = RequestBody {
            email: "example@example.com".to_string(),
            full_name: "John Johnson".to_string(),
            message: "Hello, world!".to_string(),
            subject: "A great greeting!".to_string(),
        }; // end RequestBody

        // Set up a mock server.
        let server = setup_server().await;

        // Create a hyper client.
        let client = hyper::Client::new();

        // Send a request and get a response from the server.
        let response = client
            .request(
                Request::builder()
                    .method(hyper::Method::POST)
                    .header("Content-Type", "application/json")
                    .uri(format!("http://{SERVER_ADDR}/dispatch_email"))
                    .body(Body::from(serde_json::to_string(&json_data).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Get a server response.
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

        // Debug display.
        println!("{:?}", body);

        // Assemble the response body in a struct.
        let res: ResponseBody = serde_json::from_slice(&body).unwrap();

        // Kill the server.
        server.abort();

        assert_eq!(res.message.unwrap(), "The email was sent successfully!");
    }

    /// Test endpoint that is responsible for inserting
    /// users to the database.
    #[tokio::test]
    async fn insert() {
        // Mock data to insert in database.
        let new_user = NewUser {
            name: "John".to_string(),
            email: Some("john@example.com".to_string()),
            phone_number_code: 1,
            phone_number: "1111111111".to_string(),
            password: None,
        }; // end NewUser

        // Build the form data string from the new_user data structure
        let form_data = format!(
            "name={}&email={}&phone_number_code={}&phone_number={}&password={}",
            encode(&new_user.name),
            new_user
                .email
                .map_or("".to_string(), |email| encode(&email).to_string()),
            new_user.phone_number_code,
            encode(&new_user.phone_number),
            new_user
                .password
                .map_or("".to_string(), |password| encode(&password).to_string()),
        );

        // Debug display.
        println!("{}", form_data);

        // Set up a mock server.
        let server = setup_server().await;

        // Set up a client.
        let client = hyper::Client::new();

        // Send a request to the server.
        // Send a request and get a response from the server.
        let response = client
            .request(
                Request::builder()
                    .method(hyper::Method::POST)
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .uri(format!("http://{SERVER_ADDR}/insert"))
                    .body(Body::from(form_data))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Get a server response.
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        println!("{:?}", body);

        // Assemble the response body in a struct.
        let res: ResponseBody = serde_json::from_slice(&body).unwrap();

        // Kill the server.
        server.abort();

        assert_eq!(res.message.unwrap(), "The subscription was successful!");
    }
}
