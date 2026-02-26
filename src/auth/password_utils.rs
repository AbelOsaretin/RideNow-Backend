use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AuthConfig {
    jwt_secret: String,
    jwt_expiry_hours: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,  // expiry timestamp
    pub role: String,
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let result = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    Ok(result)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn create_token(user_id: &str, role: &str) -> Result<String, String> {
    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| "Failed to read JWT_SECRET environment variable".to_string())?;
    let jwt_expiry_hours = std::env::var("JWT_EXPIRATION_SECONDS")
        .map_err(|_| "Failed to read JWT_EXPIRATION_SECONDS environment variable".to_string())?;
    let jwt_expiry_hours = jwt_expiry_hours
        .parse::<i64>()
        .map_err(|_| "Failed to parse JWT_EXPIRATION_SECONDS as integer".to_string())?;
    let expiry = Utc::now() + Duration::hours(jwt_expiry_hours);
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiry.timestamp() as usize,
        role: role.to_string(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| "Failed to create JWT token".into())
}

pub fn verify_token(token: &str) -> Result<Claims, String> {
    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| "Failed to read JWT_SECRET environment variable".to_string())?;
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| "Failed to verify JWT token".into())
}
