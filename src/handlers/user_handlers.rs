use axum::Extension;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use sqlx::PgPool;

use crate::models::user_model::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::services::user_service::{
    create_user_service, delete_user_service, get_user_service, list_users_service,
    patch_user_service, update_user_service,
};

// ============================================================================
// User Handlers
// ============================================================================

pub async fn list_users(
    Extension(pool): Extension<PgPool>,
) -> (StatusCode, Json<Vec<UserResponse>>) {
    let result = list_users_service(&pool).await;
    match result {
        Ok(users) => (StatusCode::OK, Json(users)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<UserResponse>) {
    (
        StatusCode::CREATED,
        Json(create_user_service(&pool, payload).await.unwrap()),
    )
}

pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, Json<UserResponse>) {
    match get_user_service(&pool, id).await {
        Ok(user) => (StatusCode::OK, Json(user)),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(UserResponse {
                id: "".to_string(),
                email: "".to_string(),
                username: "".to_string(),
                first_name: "".to_string(),
                last_name: "".to_string(),
                phone: None,
                profile_picture: None,
                is_active: false,
                created_at: chrono::Utc::now(),
            }),
        ),
    }
}

pub async fn update_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> (StatusCode, Json<UserResponse>) {
    match update_user_service(&pool, id, payload).await {
        Ok(user) => (StatusCode::OK, Json(user)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UserResponse {
                id: "".to_string(),
                email: "".to_string(),
                username: "".to_string(),
                first_name: "".to_string(),
                last_name: "".to_string(),
                phone: None,
                profile_picture: None,
                is_active: false,
                created_at: chrono::Utc::now(),
            }),
        ),
    }
}

pub async fn patch_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> (StatusCode, Json<UserResponse>) {
    match patch_user_service(&pool, id, payload).await {
        Ok(user) => (StatusCode::OK, Json(user)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(UserResponse {
                id: "".to_string(),
                email: "".to_string(),
                username: "".to_string(),
                first_name: "".to_string(),
                last_name: "".to_string(),
                phone: None,
                profile_picture: None,
                is_active: false,
                created_at: chrono::Utc::now(),
            }),
        ),
    }
}

pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, String) {
    match delete_user_service(&pool, id).await {
        Ok(_) => (StatusCode::OK, "User deleted successfully".to_string()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete user: {}", e),
        ),
    }
}
