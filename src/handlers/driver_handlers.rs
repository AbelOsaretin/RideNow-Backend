use crate::models::driver_model::{CreateDriverRequest, UpdateDriverRequest};
use axum::Json;

// ============================================================================
// Driver Handlers
// ============================================================================

pub async fn list_drivers() -> &'static str {
    "List drivers"
}

pub async fn create_driver(Json(payload): Json<CreateDriverRequest>) -> &'static str {
    "Create driver"
}

pub async fn get_driver(Json(id): Json<String>) -> &'static str {
    "Get driver"
}

pub async fn update_driver(Json(payload): Json<UpdateDriverRequest>) -> &'static str {
    "Update driver"
}

pub async fn patch_driver(Json(id): Json<String>) -> &'static str {
    "Patch driver"
}

pub async fn delete_driver(Json(id): Json<String>) -> &'static str {
    "Delete driver"
}
