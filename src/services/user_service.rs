use crate::models::user_model::{CreateUserRequest, UpdateUserRequest, User, UserResponse};
use chrono::Utc;
use sqlx::{PgPool, Row};
use tracing::{debug, error};
use uuid::Uuid;

use crate::auth::password_utils::hash_password;

pub async fn create_user_service(
    pool: &PgPool,
    payload: CreateUserRequest,
) -> Result<UserResponse, String> {
    debug!(email = %payload.email, "Starting user creation");

    // Validate input
    if payload.email.is_empty() || payload.username.is_empty() || payload.password.is_empty() {
        error!("Email, username, and password are required");
        return Err("Email, username, and password are required".to_string());
    }

    // Hash password (you'll need to implement or use a crate like `bcrypt`)
    let password_hash = hash_password(&payload.password).map_err(|e| {
        error!(error = %e, "Password hashing failed");
        format!("Password hashing failed: {}", e)
    })?;

    // Create user object
    let user = User {
        id: Uuid::new_v4().to_string(),
        email: payload.email,
        username: payload.username,
        password_hash,
        first_name: payload.first_name,
        last_name: payload.last_name,
        phone: payload.phone,
        profile_picture: None,
        is_active: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let row = sqlx::query(
        "INSERT INTO users \
        (id, email, username, password_hash, first_name, last_name, phone, profile_picture, is_active, created_at, updated_at) \
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) \
        RETURNING id, email, username, first_name, last_name, phone, profile_picture, is_active, created_at",
    )
    .bind(&user.id)
    .bind(&user.email)
    .bind(&user.username)
    .bind(&user.password_hash)
    .bind(&user.first_name)
    .bind(&user.last_name)
    .bind(&user.phone)
    .bind(&user.profile_picture)
    .bind(user.is_active)
    .bind(user.created_at)
    .bind(user.updated_at)
    .fetch_one(pool)
    .await
    .map_err(map_db_err)?;

    Ok(row_to_user_response(row))
}

pub async fn list_users_service(pool: &PgPool) -> Result<Vec<UserResponse>, String> {
    debug!("Fetching all users from database");
    let rows = sqlx::query(
        "SELECT * \
        FROM users ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Database fetch failed");
        map_db_err(e)
    })?;

    Ok(rows.into_iter().map(row_to_user_response).collect())
}

pub async fn get_user_service(pool: &PgPool, id: String) -> Result<UserResponse, String> {
    debug!(user_id = %id, "Fetching user from database");

    // Validate input
    if id.is_empty() {
        error!("User ID is required");
        return Err("User ID is required".to_string());
    }

    let row = sqlx::query(
        "SELECT id, email, username, first_name, last_name, phone, profile_picture, is_active, created_at \
        FROM users WHERE id = $1",
    )
    .bind(&id)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Database fetch failed");
        map_db_err(e)
    })?
    .ok_or_else(|| {
        error!(user_id = %id, "User not found");
        "User not found".to_string()
    })?;

    Ok(row_to_user_response(row))
}

pub async fn update_user_service(
    pool: &PgPool,
    id: String,
    payload: UpdateUserRequest,
) -> Result<UserResponse, String> {
    debug!(user_id = %id, "Starting user update");

    // Validate input
    if id.is_empty() {
        error!("User ID is required");
        return Err("User ID is required".to_string());
    }

    let row = sqlx::query(
        "UPDATE users SET \
            first_name = COALESCE($1, first_name), \
            last_name = COALESCE($2, last_name), \
            phone = COALESCE($3, phone), \
            profile_picture = COALESCE($4, profile_picture), \
            is_active = COALESCE($5, is_active), \
            password_hash = COALESCE($6, password_hash), \
            updated_at = $7 \
        WHERE id = $8 \
        RETURNING id, email, username, first_name, last_name, phone, profile_picture, is_active, created_at",
    )
    .bind(payload.first_name)
    .bind(payload.last_name)
    .bind(payload.phone)
    .bind(payload.profile_picture)
    .bind(payload.is_active)
    .bind(payload.password_hash)
    .bind(Utc::now())
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(map_db_err)?
    .ok_or_else(|| "User not found".to_string())?;

    Ok(row_to_user_response(row))
}

pub async fn patch_user_service(
    pool: &PgPool,
    id: String,
    payload: UpdateUserRequest,
) -> Result<UserResponse, String> {
    update_user_service(pool, id, payload).await
}

pub async fn delete_user_service(pool: &PgPool, id: String) -> Result<String, String> {
    debug!(user_id = %id, "Starting user deletion");

    // Validate input
    if id.is_empty() {
        error!("User ID is required");
        return Err("User ID is required".to_string());
    }

    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(&id)
        .execute(pool)
        .await
        .map_err(|e| {
            error!(error = %e, "Database delete failed");
            map_db_err(e)
        })?;

    if result.rows_affected() == 0 {
        error!(user_id = %id, "User not found");
        return Err("User not found".to_string());
    }

    debug!(user_id = %id, "User deleted from database");
    Ok(format!("User {} deleted successfully", id))
}


fn row_to_user_response(row: sqlx::postgres::PgRow) -> UserResponse {
    UserResponse {
        id: row.get("id"),
        email: row.get("email"),
        username: row.get("username"),
        password_hash: row.get("password_hash"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        phone: row.get("phone"),
        profile_picture: row.get("profile_picture"),
        is_active: row.get("is_active"),
        created_at: row.get("created_at"),
    }
}

fn map_db_err(err: sqlx::Error) -> String {
    err.to_string()
}
