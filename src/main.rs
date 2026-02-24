use axum::{Router, routing::get};
use sqlx;
mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/axum_course".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    let app = Router::new()
        .route("/", get(|| async { "Hello from Axum! 🦀" }))
        .nest("/users", routes::user_route::user_routes())
        .nest("/drivers", routes::driver_route::driver_routes());
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:{}", port);
    axum::serve(listener, app).await.unwrap();
}
