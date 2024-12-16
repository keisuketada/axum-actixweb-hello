use axum::{extract::Path, response::IntoResponse, routing::get, Router};

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {name}!")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello/:name", get(greet));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}