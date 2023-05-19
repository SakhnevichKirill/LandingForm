use routes::create_routes;

pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

pub async fn run() {
    let app = create_routes().await;

    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
