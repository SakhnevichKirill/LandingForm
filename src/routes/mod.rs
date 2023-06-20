pub mod dispatch_email;
mod index;
pub mod insert;
pub mod verify;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::custom_middleware::metrics_collector::{metrics_collector, metrics_display};
use crate::models::ApiDoc;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use dispatch_email::dispatch_email;
use index::index;
use insert::insert;
use verify::verify;

/// This function creates a router with routes, middleware, layers and so on.
pub async fn create_routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/verify", post(verify))
        .route("/insert", post(insert))
        .route("/dispatch_email", post(dispatch_email))
        .route("/metrics", get(metrics_display))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .layer(middleware::from_fn(metrics_collector))
}

/// These are endpoint tests
///
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use hyper;
    use serde::{Deserialize, Serialize};
    use tower::ServiceExt;

    const SERVER_ADDR: &str = "127.0.0.1:8181";

    /// Test an endpoint that is responsible for sending emails.
    #[tokio::test]
    async fn dispatch_email() {
        // A request body template.
        #[derive(Serialize)]
        struct RequestBody {
            email: String,
            fullname: String,
            message: String,
            subject: String,
        }

        // A response body template.
        #[derive(Deserialize)]
        struct ResponseBody {
            message: Option<String>,
            redirect: Option<String>,
        }

        // Data to send.
        let json_data: RequestBody = RequestBody {
            email: "example@example.com".to_string(),
            fullname: "John Johnson".to_string(),
            message: "Hello, world!".to_string(),
            subject: "A great greeting!".to_string(),
        };

        // Get router.
        let app = create_routes().await;

        // Run a fake Axum server.
        tokio::spawn(async move {
            axum::Server::bind(&SERVER_ADDR.parse().unwrap())
                .serve(app.into_make_service())
                .await
                .unwrap();
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

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
        println!("{:?}", body);

        // Assemble the response body in a struct.
        let res: ResponseBody = serde_json::from_slice(&body).unwrap();

        assert_eq!(res.message.unwrap(), "The email was sent successfully!");
    }
}
