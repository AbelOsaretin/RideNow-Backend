use axum::{Extension, Router, routing::get};
use tracing::info;
use tracing_subscriber;

use services::database_service::init_db_pool;
mod handlers;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Starting RideNow Backend");

    let db_pool = init_db_pool()
        .await
        .expect("Failed to initialize database pool");
    
    info!("Database pool initialized successfully");
    
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    let app = Router::new()
        .route("/", get(|| async { "Hello from Axum! 🦀" }))
        .nest("/users", routes::user_route::user_routes())
        .nest("/drivers", routes::driver_route::driver_routes())
        .layer(Extension(db_pool));
    
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    
    info!("Listening on http://127.0.0.1:{}", port);
    axum::serve(listener, app).await.unwrap();
}
