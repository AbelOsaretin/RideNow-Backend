use crate::models::transport_company_model::{
    CreateDriverRequest, CreateTransportCompanyRequest, CreateVehicleRequest, Driver,
    DriverResponse, TransportCompany, TransportCompanyResponse, UpdateDriverRequest,
    UpdateTransportCompanyRequest, UpdateVehicleRequest, Vehicle, VehicleResponse,
};
use chrono::Utc;
use sqlx::{PgPool, Row};
use tracing::{debug, error};
use uuid::Uuid;

//===============================================================================
// Transport Company Services
//===============================================================================

pub async fn create_transport_company_service(
    pool: &PgPool,
    payload: CreateTransportCompanyRequest,
) -> Result<TransportCompanyResponse, String> {
    debug!(email = %payload.email, "Starting transport company creation");

    // Validate input
    if payload.name.is_empty() || payload.email.is_empty() {
        error!("Name and email are required");
        return Err("Name and email are required".to_string());
    }

    // Create transport company object
    let company = TransportCompany {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        email: payload.email,
        phone: payload.phone,
        address: payload.address,
        vehicles: None,
        drivers: None,
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let row = sqlx::query(
        "INSERT INTO transport_companies \
        (id, name, email, phone, address, is_active, created_at, updated_at) \
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8) \
        RETURNING id, name, email, phone, address, is_active, created_at, updated_at",
    )
    .bind(&company.id)
    .bind(&company.name)
    .bind(&company.email)
    .bind(&company.phone)
    .bind(&company.address)
    .bind(company.is_active)
    .bind(company.created_at)
    .bind(company.updated_at)
    .fetch_one(pool)
    .await
    .map_err(map_db_err)?;

    Ok(row_to_transport_company_response(row))
}

pub async fn list_transport_companies_service(
    pool: &PgPool,
) -> Result<Vec<TransportCompanyResponse>, String> {
    let rows = sqlx::query(
        "SELECT id, name, email, phone, address, is_active, created_at, updated_at \
        FROM transport_companies",
    )
    .fetch_all(pool)
    .await
    .map_err(map_db_err)?;

    Ok(rows
        .into_iter()
        .map(row_to_transport_company_response)
        .collect())
}

pub async fn get_transport_company_service(
    pool: &PgPool,
    id: String,
) -> Result<TransportCompanyResponse, String> {
    let row = sqlx::query(
        "SELECT id, name, email, phone, address, is_active, created_at, updated_at \
        FROM transport_companies WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(map_db_err)?;

    Ok(row_to_transport_company_response(row))
}

pub async fn update_transport_company_service(
    pool: &PgPool,
    id: String,
    payload: UpdateTransportCompanyRequest,
) -> Result<TransportCompanyResponse, String> {
    let row = sqlx::query(
        "UPDATE transport_companies SET \
        name = COALESCE($1, name), \
        email = COALESCE($2, email), \
        phone = COALESCE($3, phone), \
        address = COALESCE($4, address), \
        is_active = COALESCE($5, is_active), \
        updated_at = $6 \
        WHERE id = $7 \
        RETURNING id, name, email, phone, address, is_active, created_at, updated_at",
    )
    .bind(payload.name)
    .bind(payload.email)
    .bind(payload.phone)
    .bind(payload.address)
    .bind(payload.is_active)
    .bind(Utc::now())
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(map_db_err)?;

    Ok(row_to_transport_company_response(row))
}

pub async fn patch_transport_company_service(
    pool: &PgPool,
    id: String,
    payload: UpdateTransportCompanyRequest,
) -> Result<TransportCompanyResponse, String> {
    // For simplicity, this is the same as update - in a real implementation, you might want to handle this differently
    update_transport_company_service(pool, id, payload).await
}

pub async fn delete_transport_company_service(pool: &PgPool, id: String) -> Result<(), String> {
    sqlx::query("DELETE FROM transport_companies WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_db_err)?;

    Ok(())
}

// =====================================================================================
// Vehicle Services
// =====================================================================================

pub async fn create_vehicle_service(
    pool: &PgPool,
    payload: CreateVehicleRequest,
) -> Result<VehicleResponse, String> {
    debug!(license_plate = %payload.license_plate, "Starting vehicle creation");

    // Validate input
    if payload.make.is_empty() || payload.model.is_empty() || payload.license_plate.is_empty() {
        error!("Make, model, and license plate are required");
        return Err("Make, model, and license plate are required".to_string());
    }

    // Create vehicle object
    let vehicle = Vehicle {
        id: Uuid::new_v4().to_string(),
        transport_company_id: payload.transport_company_id,
        make: payload.make,
        model: payload.model,
        year: payload.year,
        license_plate: payload.license_plate,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let row = sqlx::query(
        "INSERT INTO vehicles \
        (id, transport_company_id, make, model, year, license_plate, created_at, updated_at) \
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8) \
        RETURNING id, transport_company_id, make, model, year, license_plate, created_at, updated_at",
    )
    .bind(&vehicle.id)
    .bind(&vehicle.transport_company_id)
    .bind(&vehicle.make)
    .bind(&vehicle.model)
    .bind(vehicle.year)
    .bind(&vehicle.license_plate)
    .bind(vehicle.created_at)
    .bind(vehicle.updated_at)
    .fetch_one(pool)
    .await
    .map_err(map_db_err)?;

    Ok(row_to_vehicle_response(row))
}

pub async fn list_vehicles_service(
    pool: &PgPool,
    transport_company_id: String,
) -> Result<Vec<VehicleResponse>, String> {
    let rows = sqlx::query(
        "SELECT id, transport_company_id, make, model, year, license_plate, created_at, updated_at \
        FROM vehicles WHERE transport_company_id = $1",
    )
    .bind(transport_company_id)
    .fetch_all(pool)
    .await
    .map_err(map_db_err)?;

    Ok(rows.into_iter().map(row_to_vehicle_response).collect())
}

pub async fn get_vehicle_service(pool: &PgPool, id: String) -> Result<VehicleResponse, String> {
    let row = sqlx::query(
        "SELECT id, transport_company_id, make, model, year, license_plate, created_at, updated_at \
        FROM vehicles WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(map_db_err)?;

    Ok(row_to_vehicle_response(row))
}

pub async fn update_vehicle_service(
    pool: &PgPool,
    id: String,
    payload: UpdateVehicleRequest,
) -> Result<VehicleResponse, String> {
    let row = sqlx::query(
        "UPDATE vehicles SET \
        make = COALESCE($1, make), \
        model = COALESCE($2, model), \
        year = COALESCE($3, year), \
        license_plate = COALESCE($4, license_plate), \
        updated_at = $5 \
        WHERE id = $6 \
        RETURNING id, transport_company_id, make, model, year, license_plate, created_at, updated_at",
    )
    .bind(payload.make)
    .bind(payload.model)
    .bind(payload.year)
    .bind(payload.license_plate)
    .bind(Utc::now())
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(map_db_err)?;

    Ok(row_to_vehicle_response(row))
}

pub async fn patch_vehicle_service(
    pool: &PgPool,
    id: String,
    payload: UpdateVehicleRequest,
) -> Result<VehicleResponse, String> {
    // For simplicity, this is the same as update - in a real implementation, you might want to handle this differently
    update_vehicle_service(pool, id, payload).await
}

pub async fn delete_vehicle_service(pool: &PgPool, id: String) -> Result<(), String> {
    sqlx::query("DELETE FROM vehicles WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_db_err)?;

    Ok(())
}

// =====================================================================================
// Driver Services
// =====================================================================================

pub async fn create_driver_service(
    pool: &PgPool,
    transport_company_id: String,
    payload: CreateDriverRequest,
) -> Result<DriverResponse, String> {
    debug!(email = %payload.email, "Starting driver creation");

    // Validate input
    if payload.email.is_empty() || payload.username.is_empty() || payload.password.is_empty() {
        error!("Email, username, and password are required");
        return Err("Email, username, and password are required".to_string());
    }

    // Hash password
    let password_hash = hash_password(&payload.password).map_err(|e| {
        error!(error = %e, "Password hashing failed");
        format!("Password hashing failed: {}", e)
    })?;

    // Create driver object
    let driver = Driver {
        id: Uuid::new_v4().to_string(),
        transport_company_id,
        first_name: payload.first_name,
        last_name: payload.last_name,
        username: payload.username,
        email: payload.email,
        password_hash,
        role: crate::models::transport_company_model::Role::TCDriver,
        phone: payload.phone,
        license_number: payload.license_number.unwrap_or_default(),
        vehicle_type: payload.vehicle_type.unwrap_or_default(),
        rating: payload.rating,
        is_available: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let row = sqlx::query(
        "INSERT INTO transport_company_drivers \
        (id, transport_company_id, first_name, last_name, username, email, password_hash, role, phone, license_number, vehicle_type, rating, is_available, created_at, updated_at) \
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15) \
        RETURNING id, first_name, last_name, username, email, phone, license_number, vehicle_type, rating, is_available, created_at, updated_at",
    )
    .bind(&driver.id)
    .bind(&driver.transport_company_id)
    .bind(&driver.first_name)
    .bind(&driver.last_name)
    .bind(&driver.username)
    .bind(&driver.email)
    .bind(&driver.password_hash)
    .bind(format!("{:?}", driver.role))
    .bind(&driver.phone)
    .bind(&driver.license_number)
    .bind(&driver.vehicle_type)
    .bind(driver.rating)
    .bind(driver.is_available)
    .bind(driver.created_at)
    .bind(driver.updated_at)
    .fetch_one(pool)
    .await
    .map_err(map_db_err)?;

    Ok(row_to_driver_response(row))
}

pub async fn list_drivers_service(
    pool: &PgPool,
    transport_company_id: String,
) -> Result<Vec<DriverResponse>, String> {
    let rows = sqlx::query(
        "SELECT id, first_name, last_name, username, email, phone, license_number, vehicle_type, rating, is_available, created_at, updated_at \
        FROM transport_company_drivers WHERE transport_company_id = $1",
    )
    .bind(transport_company_id)
    .fetch_all(pool)
    .await
    .map_err(map_db_err)?;
    let mut drivers = Vec::new();
    for row in rows {
        drivers.push(row_to_driver_response(row));
    }
    Ok(drivers)
}

pub async fn get_driver_service(pool: &PgPool, id: String) -> Result<DriverResponse, String> {
    let row = sqlx::query(
        "SELECT id, first_name, last_name, username, email, phone, license_number, vehicle_type, rating, is_available, created_at, updated_at \
        FROM transport_company_drivers WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(map_db_err)?;

    Ok(row_to_driver_response(row))
}

pub async fn update_driver_service(
    pool: &PgPool,
    id: String,
    payload: UpdateDriverRequest,
) -> Result<DriverResponse, String> {
    let row = sqlx::query(
        "UPDATE transport_company_drivers SET \
        first_name = COALESCE($1, first_name), \
        last_name = COALESCE($2, last_name), \
        username = COALESCE($3, username), \
        email = COALESCE($4, email), \
        password_hash = COALESCE($5, password_hash), \
        phone = COALESCE($6, phone), \
        license_number = COALESCE($7, license_number), \
        vehicle_type = COALESCE($8, vehicle_type), \
        rating = COALESCE($9, rating), \
        is_available = COALESCE($10, is_available), \
        updated_at = $11 \
        WHERE id = $12 \
        RETURNING id, first_name, last_name, username, email, phone, license_number, vehicle_type, rating, is_available, created_at, updated_at",
    )
    .bind(payload.first_name)
    .bind(payload.last_name)
    .bind(payload.username)
    .bind(payload.email)
    .bind(payload.password_hash)
    .bind(payload.phone)
    .bind(payload.license_number)
    .bind(payload.vehicle_type)
    .bind(payload.rating)
    .bind(payload.is_available)
    .bind(Utc::now())
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(map_db_err)?;

    Ok(row_to_driver_response(row))
}

pub async fn patch_driver_service(
    pool: &PgPool,
    id: String,
    payload: UpdateDriverRequest,
) -> Result<DriverResponse, String> {
    // For simplicity, this is the same as update - in a real implementation, you might want to handle this differently
    update_driver_service(pool, id, payload).await
}

pub async fn delete_driver_service(pool: &PgPool, id: String) -> Result<(), String> {
    sqlx::query("DELETE FROM transport_company_drivers WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_db_err)?;

    Ok(())
}

//=====================================================================================
// Helper Functions
//=====================================================================================

fn hash_password(password: &str) -> Result<String, String> {
    // Placeholder - implement with bcrypt or argon2
    Ok(format!("hashed_{}", password))
}

fn row_to_transport_company_response(row: sqlx::postgres::PgRow) -> TransportCompanyResponse {
    TransportCompanyResponse {
        id: row.get("id"),
        name: row.get("name"),
        email: row.get("email"),
        phone: row.get("phone"),
        address: row.get("address"),
        is_active: row.get("is_active"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        vehicles: None,
        drivers: None,
    }
}

fn row_to_vehicle_response(row: sqlx::postgres::PgRow) -> VehicleResponse {
    VehicleResponse {
        id: row.get("id"),
        transport_company_id: row.get("transport_company_id"),
        make: row.get("make"),
        model: row.get("model"),
        year: row.get("year"),
        license_plate: row.get("license_plate"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn row_to_driver_response(row: sqlx::postgres::PgRow) -> DriverResponse {
    DriverResponse {
        id: row.get("id"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        username: row.get("username"),
        email: row.get("email"),
        phone: row.get("phone"),
        vehicle_id: None,
        license_number: row.get("license_number"),
        vehicle_type: row.get("vehicle_type"),
        rating: row.get("rating"),
        is_available: row.get("is_available"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn map_db_err(err: sqlx::Error) -> String {
    err.to_string()
}
