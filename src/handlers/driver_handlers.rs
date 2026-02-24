use crate::models::driver_model::{CreateDriverRequest, DriverResponse, UpdateDriverRequest};
use axum::Extension;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use sqlx::PgPool;

use crate::services::driver_service::{
    create_driver_service, delete_driver_service, get_driver_service, list_drivers_service,
    patch_driver_service, update_driver_service,
};

// ============================================================================
// Driver Handlers
// ============================================================================

pub async fn list_drivers(
    Extension(pool): Extension<PgPool>,
) -> (StatusCode, Json<Vec<DriverResponse>>) {
    let result = list_drivers_service(&pool).await;
    match result {
        Ok(drivers) => (StatusCode::OK, Json(drivers)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}

pub async fn create_driver(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateDriverRequest>,
) -> (StatusCode, Json<DriverResponse>) {
    (
        StatusCode::CREATED,
        Json(create_driver_service(&pool, payload).await.unwrap()),
    )
}

pub async fn get_driver(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, Json<DriverResponse>) {
    match get_driver_service(&pool, id).await {
        Ok(driver) => (StatusCode::OK, Json(driver)),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(DriverResponse {
                id: "".to_string(),
                email: "".to_string(),
                username: "".to_string(),
                first_name: "".to_string(),
                last_name: "".to_string(),
                phone: None,
                license_number: "".to_string(),
                vehicle_type: "".to_string(),
                is_available: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }),
        ),
    }
}

pub async fn update_driver(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateDriverRequest>,
) -> (StatusCode, Json<DriverResponse>) {
    match update_driver_service(&pool, id, payload).await {
        Ok(driver) => (StatusCode::OK, Json(driver)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DriverResponse {
                id: "".to_string(),
                email: "".to_string(),
                username: "".to_string(),
                first_name: "".to_string(),
                last_name: "".to_string(),
                phone: None,
                license_number: "".to_string(),
                vehicle_type: "".to_string(),
                is_available: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }),
        ),
    }
}

pub async fn patch_driver(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateDriverRequest>,
) -> (StatusCode, Json<DriverResponse>) {
    match patch_driver_service(&pool, id, payload).await {
        Ok(driver) => (StatusCode::OK, Json(driver)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DriverResponse {
                id: "".to_string(),
                email: "".to_string(),
                username: "".to_string(),
                first_name: "".to_string(),
                last_name: "".to_string(),
                phone: None,
                license_number: "".to_string(),
                vehicle_type: "".to_string(),
                is_available: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }),
        ),
    }
}

pub async fn delete_driver(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, String) {
    match delete_driver_service(&pool, id).await {
        Ok(_) => (StatusCode::OK, "Driver deleted successfully".to_string()),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete driver: {}", e),
        ),
    }
}
