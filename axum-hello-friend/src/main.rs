use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use serde::Deserialize;

// 以下もOK
// async fn greet(Path((name, friend_name)): Path<(String, String)>) -> impl IntoResponse {
//     format!("Hello {name} and {friend_name}!")
// }

#[derive(Deserialize)]
pub struct PathInfo {
    pub name: String,
    pub friend_name: String,
}

async fn greet(Path(p): Path<PathInfo>) -> impl IntoResponse {
    format!("Hello {} and {}!", p.name, p.friend_name)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello/:name/:friend_name", get(greet));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
