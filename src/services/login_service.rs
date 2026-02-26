use crate::auth::password_utils::verify_password;
use crate::models::login_model::{LoginRequest, LoginResponse};
use sqlx::{PgPool, Row};

pub async fn login_service(pool: &PgPool, payload: LoginRequest) -> Result<LoginResponse, String> {
    // Validate input
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err("Username and password are required".to_string());
    }

    // Fetch user by username
    let row = sqlx::query("SELECT id, username, password_hash FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_one(pool)
        .await
        .map_err(|_| "Invalid username or password".to_string())?;

    let new_row: sqlx::postgres::PgRow = row;

    let user_id: String = new_row
        .try_get::<String, _>("id")
        .map_err(|_| "Invalid username or password".to_string())?;
    let username: String = new_row
        .try_get::<String, _>("username")
        .map_err(|_| "Invalid username or password".to_string())?;
    let password_hash: String = new_row
        .try_get::<String, _>("password_hash")
        .map_err(|_| "Invalid username or password".to_string())?;

    // Verify password
    if !verify_password(&payload.password, &password_hash) {
        return Err("Invalid username or password".to_string());
    }

    // Generate token (placeholder - implement JWT or similar)
    let token = format!("token_for_user_{}", user_id);

    Ok(LoginResponse {
        token,
        user_id,
        email: username.clone(),
        username,
    })
}
