use axum::{Extension, Router, routing::get};
use services::database_service::init_db_pool;
use tracing::info;
mod auth;
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
        .nest("/login", routes::login_route::login_route())
        .nest("/protected", routes::protected_route::protected_routes())
        .nest("/users", routes::user_route::user_routes())
        .nest(
            "/transport-companies",
            routes::transport_company_route::transport_company_routes(),
        )
        .nest("/payments", routes::payment_route::payment_routes())
        .layer(Extension(db_pool));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    info!("Listening on http://127.0.0.1:{}", port);
    axum::serve(listener, app).await.unwrap();
}
