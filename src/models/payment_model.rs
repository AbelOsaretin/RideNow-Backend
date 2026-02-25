use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// Request/Response Models
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeRequest {
    pub email: String,
    pub amount: String,
    pub currency: Option<String>,
    pub user_id: Option<String>,
    pub driver_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeResponse {
    pub status: bool,
    pub message: String,
    pub data: InitializeDataResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeDataResponse {
    pub authorization_url: String,
    pub access_code: String,
    pub reference: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct VerifyResponse {
    pub status: bool,
    pub message: String,
    pub data: VerifyResponseData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct VerifyResponseData {
    pub status: String,
    pub amount: u64,
    pub reference: String,
    pub gateway_response: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookEvent {
    pub event: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub id: String,
    pub payer_type: String, // "user" or "driver"
    pub payer_id: String,
    pub email: String,
    pub amount: String,
    pub currency: String,
    pub status: String,
    pub reference: String,
    pub authorization_url: Option<String>,
    pub access_code: Option<String>,
    pub gateway_response: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
