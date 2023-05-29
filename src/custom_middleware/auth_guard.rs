use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn auth_guard<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // Check if it is an allowed uri.

    Ok(next.run(req).await)
}
