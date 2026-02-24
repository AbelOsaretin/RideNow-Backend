use axum::Json;

use crate::models::user_model::{CreateUserRequest, UpdateUserRequest};

// ============================================================================
// User Handlers
// ============================================================================

pub async fn list_users() -> &'static str {
    "List users"
}

pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> &'static str {
    "Create user"
}

pub async fn get_user(Json(id): Json<String>) -> &'static str {
    "Get user"
}

pub async fn update_user(Json(payload): Json<UpdateUserRequest>) -> &'static str {
    "Update user"
}

pub async fn patch_user(Json(id): Json<String>) -> &'static str {
    "Patch user"
}

pub async fn delete_user(Json(id): Json<String>) -> &'static str {
    "Delete user"
}
