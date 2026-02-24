use axum::Extension;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use sqlx::PgPool;
use tracing::{error, info};

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
    info!("Fetching all users");
    let result = list_users_service(&pool).await;
    match result {
        Ok(users) => {
            info!(count = users.len(), "Users fetched successfully");
            (StatusCode::OK, Json(users))
        }
        Err(e) => {
            error!(error = %e, "Failed to fetch users");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<UserResponse>) {
    info!(email = %payload.email, username = %payload.username, "Creating new user");
    let result = create_user_service(&pool, payload).await;

    match result {
        Ok(user) => {
            info!(user_id = %user.id, "User created successfully");
            (StatusCode::CREATED, Json(user))
        }
        Err(e) => {
            error!(error = %e, "Failed to create user");
            (
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
            )
        }
    }
}

pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, Json<UserResponse>) {
    info!(user_id = %id, "Fetching user by ID");
    match get_user_service(&pool, id.clone()).await {
        Ok(user) => {
            info!(user_id = %id, "User fetched successfully");
            (StatusCode::OK, Json(user))
        }
        Err(e) => {
            error!(user_id = %id, error = %e, "User not found");
            (
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
            )
        }
    }
}

pub async fn update_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> (StatusCode, Json<UserResponse>) {
    info!(user_id = %id, "Updating user");
    match update_user_service(&pool, id.clone(), payload).await {
        Ok(user) => {
            info!(user_id = %id, "User updated successfully");
            (StatusCode::OK, Json(user))
        }
        Err(e) => {
            error!(user_id = %id, error = %e, "Failed to update user");
            (
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
            )
        }
    }
}

pub async fn patch_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> (StatusCode, Json<UserResponse>) {
    info!(user_id = %id, "Patching user");
    match patch_user_service(&pool, id.clone(), payload).await {
        Ok(user) => {
            info!(user_id = %id, "User patched successfully");
            (StatusCode::OK, Json(user))
        }
        Err(e) => {
            error!(user_id = %id, error = %e, "Failed to patch user");
            (
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
            )
        }
    }
}

pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, String) {
    info!(user_id = %id, "Deleting user");
    match delete_user_service(&pool, id.clone()).await {
        Ok(msg) => {
            info!(user_id = %id, "User deleted successfully");
            (StatusCode::OK, msg)
        }
        Err(e) => {
            error!(user_id = %id, error = %e, "Failed to delete user");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete user: {}", e),
            )
        }
    }
}
