use axum::Extension;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use sqlx::PgPool;
use tracing::{error, info};

use crate::models::transport_company_model::{
    CreateDriverRequest, CreateTransportCompanyRequest, CreateVehicleRequest, DriverResponse,
    TransportCompanyResponse, UpdateDriverRequest, UpdateTransportCompanyRequest,
    UpdateVehicleRequest, VehicleResponse,
};
use crate::services::transport_company_service::{
    create_driver_service, create_transport_company_service, create_vehicle_service,
    delete_driver_service, delete_transport_company_service, delete_vehicle_service,
    get_driver_service, get_transport_company_service, get_vehicle_service, list_drivers_service,
    list_transport_companies_service, list_vehicles_service, patch_driver_service,
    patch_transport_company_service, patch_vehicle_service, update_driver_service,
    update_transport_company_service, update_vehicle_service,
};
use chrono::Utc;

// ============================================================================
// Transport Company Handlers
// ============================================================================

pub async fn list_transport_companies(
    Extension(pool): Extension<PgPool>,
) -> (StatusCode, Json<Vec<TransportCompanyResponse>>) {
    info!("Fetching all transport companies");
    let result = list_transport_companies_service(&pool).await;
    match result {
        Ok(companies) => {
            info!(
                count = companies.len(),
                "Transport companies fetched successfully"
            );
            (StatusCode::OK, Json(companies))
        }
        Err(e) => {
            error!(error = %e, "Failed to fetch transport companies");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn create_transport_company(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateTransportCompanyRequest>,
) -> (StatusCode, Json<TransportCompanyResponse>) {
    info!(email = %payload.email, name = %payload.name, "Creating new transport company");
    let result = create_transport_company_service(&pool, payload).await;

    match result {
        Ok(company) => {
            info!(company_id = %company.id, "Transport company created successfully");
            (StatusCode::CREATED, Json(company))
        }
        Err(e) => {
            error!(error = %e, "Failed to create transport company");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TransportCompanyResponse {
                    id: "".to_string(),
                    name: "".to_string(),
                    email: "".to_string(),
                    phone: None,
                    address: None,
                    is_active: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    vehicles: None,
                    drivers: None,
                }),
            )
        }
    }
}

pub async fn get_transport_company(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, Json<TransportCompanyResponse>) {
    info!(company_id = %id, "Fetching transport company by ID");
    match get_transport_company_service(&pool, id.clone()).await {
        Ok(company) => {
            info!(company_id = %id, "Transport company fetched successfully");
            (StatusCode::OK, Json(company))
        }
        Err(e) => {
            error!(company_id = %id, error = %e, "Transport company not found");
            (
                StatusCode::NOT_FOUND,
                Json(TransportCompanyResponse {
                    id: "".to_string(),
                    name: "".to_string(),
                    email: "".to_string(),
                    phone: None,
                    address: None,
                    is_active: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    vehicles: None,
                    drivers: None,
                }),
            )
        }
    }
}

pub async fn update_transport_company(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTransportCompanyRequest>,
) -> (StatusCode, Json<TransportCompanyResponse>) {
    info!(company_id = %id, "Updating transport company");
    match update_transport_company_service(&pool, id.clone(), payload).await {
        Ok(company) => {
            info!(company_id = %id, "Transport company updated successfully");
            (StatusCode::OK, Json(company))
        }
        Err(e) => {
            error!(company_id = %id, error = %e, "Failed to update transport company");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TransportCompanyResponse {
                    id: "".to_string(),
                    name: "".to_string(),
                    email: "".to_string(),
                    phone: None,
                    address: None,
                    is_active: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    vehicles: None,
                    drivers: None,
                }),
            )
        }
    }
}

pub async fn patch_transport_company(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTransportCompanyRequest>,
) -> (StatusCode, Json<TransportCompanyResponse>) {
    info!(company_id = %id, "Patching transport company");
    match patch_transport_company_service(&pool, id.clone(), payload).await {
        Ok(company) => {
            info!(company_id = %id, "Transport company patched successfully");
            (StatusCode::OK, Json(company))
        }
        Err(e) => {
            error!(company_id = %id, error = %e, "Failed to patch transport company");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TransportCompanyResponse {
                    id: "".to_string(),
                    name: "".to_string(),
                    email: "".to_string(),
                    phone: None,
                    address: None,
                    is_active: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    vehicles: None,
                    drivers: None,
                }),
            )
        }
    }
}

pub async fn delete_transport_company(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, String) {
    info!(company_id = %id, "Deleting transport company");
    match delete_transport_company_service(&pool, id.clone()).await {
        Ok(_) => {
            info!(company_id = %id, "Transport company deleted successfully");
            (
                StatusCode::OK,
                format!("Transport company {} deleted successfully", id),
            )
        }
        Err(e) => {
            error!(company_id = %id, error = %e, "Failed to delete transport company");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete transport company: {}", e),
            )
        }
    }
}

// ============================================================================
// Vehicle Handlers
// ============================================================================

pub async fn list_vehicles(
    Extension(pool): Extension<PgPool>,
    Path(transport_company_id): Path<String>,
) -> (StatusCode, Json<Vec<VehicleResponse>>) {
    info!(company_id = %transport_company_id, "Fetching all vehicles");
    let result = list_vehicles_service(&pool, transport_company_id).await;
    match result {
        Ok(vehicles) => {
            info!(count = vehicles.len(), "Vehicles fetched successfully");
            (StatusCode::OK, Json(vehicles))
        }
        Err(e) => {
            error!(error = %e, "Failed to fetch vehicles");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn create_vehicle(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateVehicleRequest>,
) -> (StatusCode, Json<VehicleResponse>) {
    info!(license_plate = %payload.license_plate, "Creating new vehicle");
    let result = create_vehicle_service(&pool, payload).await;

    match result {
        Ok(vehicle) => {
            info!(vehicle_id = %vehicle.id, "Vehicle created successfully");
            (StatusCode::CREATED, Json(vehicle))
        }
        Err(e) => {
            error!(error = %e, "Failed to create vehicle");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(VehicleResponse {
                    id: "".to_string(),
                    transport_company_id: "".to_string(),
                    make: "".to_string(),
                    model: "".to_string(),
                    year: 0,
                    license_plate: "".to_string(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }),
            )
        }
    }
}

pub async fn get_vehicle(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, Json<VehicleResponse>) {
    info!(vehicle_id = %id, "Fetching vehicle by ID");
    match get_vehicle_service(&pool, id.clone()).await {
        Ok(vehicle) => {
            info!(vehicle_id = %id, "Vehicle fetched successfully");
            (StatusCode::OK, Json(vehicle))
        }
        Err(e) => {
            error!(vehicle_id = %id, error = %e, "Vehicle not found");
            (
                StatusCode::NOT_FOUND,
                Json(VehicleResponse {
                    id: "".to_string(),
                    transport_company_id: "".to_string(),
                    make: "".to_string(),
                    model: "".to_string(),
                    year: 0,
                    license_plate: "".to_string(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }),
            )
        }
    }
}

pub async fn update_vehicle(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateVehicleRequest>,
) -> (StatusCode, Json<VehicleResponse>) {
    info!(vehicle_id = %id, "Updating vehicle");
    match update_vehicle_service(&pool, id.clone(), payload).await {
        Ok(vehicle) => {
            info!(vehicle_id = %id, "Vehicle updated successfully");
            (StatusCode::OK, Json(vehicle))
        }
        Err(e) => {
            error!(vehicle_id = %id, error = %e, "Failed to update vehicle");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(VehicleResponse {
                    id: "".to_string(),
                    transport_company_id: "".to_string(),
                    make: "".to_string(),
                    model: "".to_string(),
                    year: 0,
                    license_plate: "".to_string(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }),
            )
        }
    }
}

pub async fn patch_vehicle(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateVehicleRequest>,
) -> (StatusCode, Json<VehicleResponse>) {
    info!(vehicle_id = %id, "Patching vehicle");
    match patch_vehicle_service(&pool, id.clone(), payload).await {
        Ok(vehicle) => {
            info!(vehicle_id = %id, "Vehicle patched successfully");
            (StatusCode::OK, Json(vehicle))
        }
        Err(e) => {
            error!(vehicle_id = %id, error = %e, "Failed to patch vehicle");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(VehicleResponse {
                    id: "".to_string(),
                    transport_company_id: "".to_string(),
                    make: "".to_string(),
                    model: "".to_string(),
                    year: 0,
                    license_plate: "".to_string(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }),
            )
        }
    }
}

pub async fn delete_vehicle(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, String) {
    info!(vehicle_id = %id, "Deleting vehicle");
    match delete_vehicle_service(&pool, id.clone()).await {
        Ok(_) => {
            info!(vehicle_id = %id, "Vehicle deleted successfully");
            (
                StatusCode::OK,
                format!("Vehicle {} deleted successfully", id),
            )
        }
        Err(e) => {
            error!(vehicle_id = %id, error = %e, "Failed to delete vehicle");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete vehicle: {}", e),
            )
        }
    }
}

// ============================================================================
// Driver Handlers
// ============================================================================

pub async fn list_drivers(
    Extension(pool): Extension<PgPool>,
    Path(transport_company_id): Path<String>,
) -> (StatusCode, Json<Vec<DriverResponse>>) {
    info!(company_id = %transport_company_id, "Fetching all drivers");
    let result = list_drivers_service(&pool, transport_company_id).await;
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
    Path(transport_company_id): Path<String>,
    Json(payload): Json<CreateDriverRequest>,
) -> (StatusCode, Json<DriverResponse>) {
    info!(email = %payload.email, username = %payload.username, "Creating new driver");
    let result = create_driver_service(&pool, transport_company_id, payload).await;

    match result {
        Ok(driver) => {
            info!(driver_id = %driver.id, "Driver created successfully");
            (StatusCode::CREATED, Json(driver))
        }
        Err(e) => {
            error!(error = %e, "Failed to create driver");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DriverResponse {
                    id: "".to_string(),
                    first_name: "".to_string(),
                    last_name: "".to_string(),
                    username: "".to_string(),
                    email: "".to_string(),
                    phone: "".to_string(),
                    vehicle_id: None,
                    license_number: None,
                    vehicle_type: None,
                    rating: None,
                    is_available: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }),
            )
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
                    first_name: "".to_string(),
                    last_name: "".to_string(),
                    username: "".to_string(),
                    email: "".to_string(),
                    phone: "".to_string(),
                    vehicle_id: None,
                    license_number: None,
                    vehicle_type: None,
                    rating: None,
                    is_available: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
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
                    first_name: "".to_string(),
                    last_name: "".to_string(),
                    username: "".to_string(),
                    email: "".to_string(),
                    phone: "".to_string(),
                    vehicle_id: None,
                    license_number: None,
                    vehicle_type: None,
                    rating: None,
                    is_available: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
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
                    first_name: "".to_string(),
                    last_name: "".to_string(),
                    username: "".to_string(),
                    email: "".to_string(),
                    phone: "".to_string(),
                    vehicle_id: None,
                    license_number: None,
                    vehicle_type: None,
                    rating: None,
                    is_available: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
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
        Ok(_) => {
            info!(driver_id = %id, "Driver deleted successfully");
            (
                StatusCode::OK,
                format!("Driver {} deleted successfully", id),
            )
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
