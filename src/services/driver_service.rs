use crate::auth::password_utils::hash_password;
use crate::models::driver_model::{
    CreateDriverRequest, Driver, DriverResponse, UpdateDriverRequest,
};
use chrono::Utc;
use sqlx::{PgPool, Row};
use tracing::{debug, error};
use uuid::Uuid;

pub async fn create_driver_service(
    pool: &PgPool,
    payload: CreateDriverRequest,
) -> Result<DriverResponse, String> {
    debug!(email = %payload.email, "Starting driver creation");

    // Validate input
    if payload.email.is_empty()
        || payload.first_name.is_empty()
        || payload.username.is_empty()
        || payload.password.is_empty()
    {
        error!("Email, username, first name, and password are required");
        return Err("Email, username, first name, and password are required".to_string());
    }

    // Hash password (you'll need to implement or use a crate like `bcrypt`)
    let password_hash = hash_password(&payload.password).map_err(|e| {
        error!(error = %e, "Password hashing failed");
        format!("Password hashing failed: {}", e)
    })?;

    // Create driver object
    let driver = Driver {
        id: Uuid::new_v4().to_string(),
        email: payload.email,
        username: payload.username,
        first_name: payload.first_name,
        last_name: payload.last_name,
        phone: payload.phone,
        rating: payload.rating,
        license_number: payload.license_number,
        vehicle_type: payload.vehicle_type,
        password_hash,
        is_available: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let row = sqlx::query(
        "INSERT INTO drivers \
        (id, email, username, password_hash, first_name, last_name, phone, license_number, vehicle_type, rating, is_available, created_at, updated_at) \
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13) \
        RETURNING id, email, username, first_name, last_name, phone, license_number, vehicle_type, rating, is_available, created_at, updated_at",
    )
    .bind(&driver.id)
    .bind(&driver.email)
    .bind(&driver.username)
    .bind(&driver.password_hash)
    .bind(&driver.first_name)
    .bind(&driver.last_name)
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

pub async fn list_drivers_service(pool: &PgPool) -> Result<Vec<DriverResponse>, String> {
    debug!("Fetching all drivers from database");
    let rows = sqlx::query(
        "SELECT id, email, username, first_name, last_name, phone, license_number, vehicle_type, rating, is_available, created_at, updated_at \
        FROM drivers ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Database fetch failed");
        map_db_err(e)
    })?;

    Ok(rows.into_iter().map(row_to_driver_response).collect())
}

pub async fn get_driver_service(pool: &PgPool, id: String) -> Result<DriverResponse, String> {
    debug!(driver_id = %id, "Fetching driver from database");

    // Validate input
    if id.is_empty() {
        error!("Driver ID is required");
        return Err("Driver ID is required".to_string());
    }

    let row = sqlx::query(
        "SELECT id, email, username, first_name, last_name, phone, license_number, vehicle_type, rating, is_available, created_at, updated_at \
        FROM drivers WHERE id = $1",
    )
    .bind(&id)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Database fetch failed");
        map_db_err(e)
    })?
    .ok_or_else(|| {
        error!(driver_id = %id, "Driver not found");
        "Driver not found".to_string()
    })?;

    Ok(row_to_driver_response(row))
}

pub async fn update_driver_service(
    pool: &PgPool,
    id: String,
    payload: UpdateDriverRequest,
) -> Result<DriverResponse, String> {
    debug!(driver_id = %id, "Starting driver update");

    // Validate input
    if id.is_empty() {
        error!("Driver ID is required");
        return Err("Driver ID is required".to_string());
    }

    let row = sqlx::query(
        "UPDATE drivers SET \
            first_name = COALESCE($1, first_name), \
            last_name = COALESCE($2, last_name), \
            phone = COALESCE($3, phone), \
            is_available = COALESCE($4, is_available), \
            password_hash = COALESCE($5, password_hash), \
            rating = COALESCE($6, rating), \
            updated_at = $7 \
        WHERE id = $8 \
        RETURNING id, email, username, first_name, last_name, phone, license_number, vehicle_type, rating, is_available, created_at, updated_at",
    )
    .bind(payload.first_name)
    .bind(payload.last_name)
    .bind(payload.phone)
    .bind(payload.is_available)
    .bind(payload.password_hash)
    .bind(payload.rating)
    .bind(Utc::now())
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_db_err)?
    .ok_or_else(|| "Driver not found".to_string())?;

    Ok(row_to_driver_response(row))
}

pub async fn patch_driver_service(
    pool: &PgPool,
    id: String,
    payload: UpdateDriverRequest,
) -> Result<DriverResponse, String> {
    update_driver_service(pool, id, payload).await
}

pub async fn delete_driver_service(pool: &PgPool, id: String) -> Result<String, String> {
    debug!(driver_id = %id, "Starting driver deletion");

    // Validate input
    if id.is_empty() {
        error!("Driver ID is required");
        return Err("Driver ID is required".to_string());
    }

    let result = sqlx::query("DELETE FROM drivers WHERE id = $1")
        .bind(&id)
        .execute(pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Database delete failed");
            map_db_err(e)
        })?;

    if result.rows_affected() == 0 {
        error!(driver_id = %id, "Driver not found");
        return Err("Driver not found".to_string());
    }

    debug!(driver_id = %id, "Driver deleted from database");
    Ok(format!("Driver {} deleted successfully", id))
}



fn row_to_driver_response(row: sqlx::postgres::PgRow) -> DriverResponse {
    DriverResponse {
        id: row.get("id"),
        email: row.get("email"),
        username: row.get("username"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        phone: row.get("phone"),
        license_number: row.get("license_number"),
        vehicle_type: row.get("vehicle_type"),
        is_available: row.get("is_available"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn map_db_err(err: sqlx::Error) -> String {
    err.to_string()
}
