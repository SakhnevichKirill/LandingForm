pub mod dispatch_email;
mod index;
pub mod insert;
pub mod login;
pub mod register;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};

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

/// This struct contains some information that should be
/// passed to endpoints with the client requests.
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<AsyncPgConnection>,
} // end struct AppState

/// This function creates an AppState for the Router.
fn create_app_state() -> AppState {
    // create a new connection pool with the default config
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        std::env::var("DATABASE_URL")
            .expect("Failed to find the environment variable DATABASE_URL"),
    ); // end config

    // Create a connection pool.
    let pool = Pool::builder(config)
        .build()
        .expect("Failed to create a pool of connections to a database");

    // Return the required AppState.
    AppState { pool }
} // end fn create_app_state

/// This function creates a router with routes, middleware, layers and so on.
pub async fn create_routes() -> Router {
    // Create app state.
    let app_state = create_app_state();

    Router::new()
        .route("/dispatch_email", post(dispatch_email))
        .route("/metrics", get(metrics_display))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_guard,
        ))
        .route("/", get(index))
        .route("/insert", post(insert))
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(middleware::from_fn(metrics_collector))
        .with_state(app_state)
} // end fn create_routes

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

    use dotenvy::dotenv;

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
    #[should_panic]
    async fn dispatch_email_unauthorized_fails() {
        // Import environment variables.
        dotenv().ok();

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
        assert!(hyper::body::to_bytes(response.into_body()).await.is_err());

        // Abort server.
        server.abort();
    }

    /// Test endpoint that is responsible for inserting
    /// users to the database.
    #[tokio::test]
    async fn insert() {
        // Import environment variables.
        dotenv().ok();

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
