use axum::response::Html;
use std::fs;

pub async fn index() -> Html<String> {
    let html_file = fs::read_to_string("frontend/index.html").unwrap();

    Html(html_file)
}
