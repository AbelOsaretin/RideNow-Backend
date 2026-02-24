use crate::models::driver_model::{CreateDriverRequest, DriverResponse, UpdateDriverRequest};
use axum::Extension;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use sqlx::PgPool;
use tracing::{info, error};

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
    info!("Fetching all drivers");
    let result = list_drivers_service(&pool).await;
    match result {
        Ok(drivers) => {
            info!(count = drivers.len(), "Drivers fetched successfully");
            (StatusCode::OK, Json(drivers))
        }
        Err(e) => {
            error!(error = %e, "Failed to fetch drivers");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn create_driver(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateDriverRequest>,
) -> (StatusCode, Json<DriverResponse>) {
    info!(email = %payload.email, username = %payload.username, "Creating new driver");
    let result = create_driver_service(&pool, payload).await;
    
    match result {
        Ok(driver) => {
            info!(driver_id = %driver.id, "Driver created successfully");
            (StatusCode::CREATED, Json(driver))
        }
        Err(e) => {
            error!(error = %e, "Failed to create driver");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DriverResponse {
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
            }))
        }
    }
}

pub async fn get_driver(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, Json<DriverResponse>) {
    info!(driver_id = %id, "Fetching driver by ID");
    match get_driver_service(&pool, id.clone()).await {
        Ok(driver) => {
            info!(driver_id = %id, "Driver fetched successfully");
            (StatusCode::OK, Json(driver))
        }
        Err(e) => {
            error!(driver_id = %id, error = %e, "Driver not found");
            (
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
            )
        }
    }
}

pub async fn update_driver(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateDriverRequest>,
) -> (StatusCode, Json<DriverResponse>) {
    info!(driver_id = %id, "Updating driver");
    match update_driver_service(&pool, id.clone(), payload).await {
        Ok(driver) => {
            info!(driver_id = %id, "Driver updated successfully");
            (StatusCode::OK, Json(driver))
        }
        Err(e) => {
            error!(driver_id = %id, error = %e, "Failed to update driver");
            (
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
            )
        }
    }
}

pub async fn patch_driver(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateDriverRequest>,
) -> (StatusCode, Json<DriverResponse>) {
    info!(driver_id = %id, "Patching driver");
    match patch_driver_service(&pool, id.clone(), payload).await {
        Ok(driver) => {
            info!(driver_id = %id, "Driver patched successfully");
            (StatusCode::OK, Json(driver))
        }
        Err(e) => {
            error!(driver_id = %id, error = %e, "Failed to patch driver");
            (
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
            )
        }
    }
}

pub async fn delete_driver(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, String) {
    info!(driver_id = %id, "Deleting driver");
    match delete_driver_service(&pool, id.clone()).await {
        Ok(msg) => {
            info!(driver_id = %id, "Driver deleted successfully");
            (StatusCode::OK, msg)
        }
        Err(e) => {
            error!(driver_id = %id, error = %e, "Failed to delete driver");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete driver: {}", e),
            )
        }
    }
}
