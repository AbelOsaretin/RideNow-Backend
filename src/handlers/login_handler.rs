use crate::services::login_service::login_service;
use axum::{extract::Extension, http::StatusCode, Json};
use sqlx::PgPool;
use crate::models::login_model::LoginRequest;   

pub async fn login_handler(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    match login_service(&pool, payload).await {
        Ok(response) => (StatusCode::OK, Json(serde_json::json!({ "status": "success", "data": response }))),
        Err(e) => (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "status": "error", "message": e }))),
    }
}