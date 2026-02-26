use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    TCUser,
    TCDriver,
    TCAdmin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportCompany {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub vehicles: Option<Vec<Vehicle>>,
    pub drivers: Option<Vec<Driver>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransportCompanyRequest {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTransportCompanyRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportCompanyResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub vehicles: Option<Vec<Vehicle>>,
    pub drivers: Option<Vec<Driver>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub transport_company_id: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub license_plate: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVehicleRequest {
    pub transport_company_id: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub license_plate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVehicleRequest {
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<i32>,
    pub license_plate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleResponse {
    pub id: String,
    pub transport_company_id: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub license_plate: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
    pub id: String,
    pub transport_company_id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
    pub phone: String,
    pub license_number: String,
    pub vehicle_type: String,
    pub rating: Option<f32>,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDriverRequest {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub vehicle_id: Option<String>,
    pub license_number: Option<String>,
    pub vehicle_type: Option<String>,
    pub rating: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDriverRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub phone: Option<String>,
    pub vehicle_id: Option<String>,
    pub license_number: Option<String>,
    pub vehicle_type: Option<String>,
    pub rating: Option<f32>,
    pub is_available: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverResponse {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub vehicle_id: Option<String>,
    pub license_number: Option<String>,
    pub vehicle_type: Option<String>,
    pub rating: Option<f32>,
    pub is_available: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
