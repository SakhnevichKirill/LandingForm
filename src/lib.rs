use routes::create_routes;

pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

/// This function runs a server on a specified port, using pre-set-up router.
pub async fn run() {
    // Get a router with all the routes, middleware and so on.
    let app = create_routes().await;

    // Run a server based on the router specified above.
    axum::Server::bind(&"0.0.0.0:8181".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
