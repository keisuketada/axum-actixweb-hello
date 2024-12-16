use actix_web::{get, web::Path, App, HttpServer, Responder};
use serde::Deserialize;

// コンパイルできません
// #[get("/hello/{name}/{friend_name}")]
// async fn greet(Path((name, friend_name)): Path<(String, String)>) -> impl Responder {
//     format!("Hello {} and {}!", name, friend_name)
// }

#[derive(Deserialize)]
pub struct PathInfo {
    pub name: String,
    pub friend_name: String,
}

#[get("/hello/{name}/{friend_name}")]
async fn greet(p: Path<PathInfo>) -> impl Responder {
    format!("Hello {} and {}!", p.name, p.friend_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}