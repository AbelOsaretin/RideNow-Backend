use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub phone: String,
    pub license_number: String,
    pub vehicle_type: String,
    pub rating: Option<f32>,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDriverRequest {
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub email: String,
    pub username: String,
    pub phone: String,
    pub license_number: String,
    pub vehicle_type: String,
    pub rating: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDriverRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub is_available: Option<bool>,
    pub password_hash: Option<String>,
    pub rating: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DriverResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub license_number: String,
    pub vehicle_type: String,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
