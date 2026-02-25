use crate::models::payment_model::{
    InitializeDataResponse, InitializeRequest, InitializeResponse, PaymentResponse, VerifyResponse,
    VerifyResponseData,
};
use chrono::Utc;
use hex::encode;
use hmac::{Hmac, Mac};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Serialize;
use serde_json::Value;
use sha2::Sha512;
use sqlx::{PgPool, Row, types::Json};
use tracing::{debug, error, info};
use uuid::Uuid;

type HmacSha512 = Hmac<Sha512>;

#[derive(Serialize)]
struct PaystackInitializePayload {
    email: String,
    amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<String>,
}

// ============================================================================
// Payment Initialization Services
// ============================================================================

pub async fn initialize_payment_service(
    pool: &PgPool,
    payload: InitializeRequest,
) -> Result<InitializeResponse, String> {
    debug!(email = %payload.email, amount = %payload.amount, "Initializing payment");

    let api_key = std::env::var("PAYSTACK_API_KEY")
        .map_err(|_| "PAYSTACK_API_KEY not set in environment".to_string())?;

    let url = std::env::var("PAYSTACK_INITIALIZE_URL")
        .map_err(|_| "PAYSTACK_INITIALIZE_URL not set in environment".to_string())?;

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| format!("Invalid authorization header: {}", e))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let paystack_payload = PaystackInitializePayload {
        email: payload.email.clone(),
        amount: payload.amount.clone(),
        currency: payload.currency.clone(),
    };

    let response = client
        .post(&url)
        .headers(headers)
        .json(&paystack_payload)
        .send()
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to send payment initialization request");
            format!("Payment initialization request failed: {}", e)
        })?;

    let paystack_res: InitializeResponse = response.json().await.map_err(|e| {
        error!(error = %e, "Failed to parse payment initialization response");
        format!("Failed to parse response: {}", e)
    })?;

    info!(reference = %paystack_res.data.reference, "Payment initialized successfully");

    store_initialized_payment(pool, &payload, &paystack_res).await?;

    Ok(InitializeResponse {
        status: paystack_res.status,
        message: paystack_res.message,
        data: InitializeDataResponse {
            authorization_url: paystack_res.data.authorization_url,
            access_code: paystack_res.data.access_code,
            reference: paystack_res.data.reference,
        },
    })
}

pub async fn get_payment_redirect_url(
    pool: &PgPool,
    payload: InitializeRequest,
) -> Result<String, String> {
    debug!(email = %payload.email, amount = %payload.amount, "Getting payment redirect URL");

    let api_key = std::env::var("PAYSTACK_API_KEY")
        .map_err(|_| "PAYSTACK_API_KEY not set in environment".to_string())?;

    let url = std::env::var("PAYSTACK_INITIALIZE_URL")
        .map_err(|_| "PAYSTACK_INITIALIZE_URL not set in environment".to_string())?;

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| format!("Invalid authorization header: {}", e))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let paystack_payload = PaystackInitializePayload {
        email: payload.email.clone(),
        amount: payload.amount.clone(),
        currency: payload.currency.clone(),
    };

    let response = client
        .post(&url)
        .headers(headers)
        .json(&paystack_payload)
        .send()
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to get redirect URL");
            format!("Failed to get redirect URL: {}", e)
        })?;

    let paystack_res: InitializeResponse = response.json().await.map_err(|e| {
        error!(error = %e, "Failed to parse redirect response");
        format!("Failed to parse response: {}", e)
    })?;

    info!(reference = %paystack_res.data.reference, "Redirect URL generated");

    store_initialized_payment(pool, &payload, &paystack_res).await?;

    Ok(paystack_res.data.authorization_url)
}

// ============================================================================
// Payment Verification Service
// ============================================================================

pub async fn verify_payment_service(
    pool: &PgPool,
    reference: String,
) -> Result<VerifyResponse, String> {
    debug!(reference = %reference, "Verifying payment");

    let api_key = std::env::var("PAYSTACK_API_KEY")
        .map_err(|_| "PAYSTACK_API_KEY not set in environment".to_string())?;

    let url_main = std::env::var("PAYSTACK_TRANS_VERIFY_URL")
        .map_err(|_| "PAYSTACK_TRANS_VERIFY_URL not set in environment".to_string())?;

    let url = format!("{}{}", url_main, reference);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(api_key)
        .send()
        .await
        .map_err(|e| {
            error!(error = %e, "Failed to verify payment");
            format!("Payment verification request failed: {}", e)
        })?;

    let paystack_res: VerifyResponse = response.json().await.map_err(|e| {
        error!(error = %e, "Failed to parse verification response");
        format!("Failed to parse response: {}", e)
    })?;

    info!(
        reference = %paystack_res.data.reference,
        status = %paystack_res.data.status,
        amount = %paystack_res.data.amount,
        "Payment verified"
    );

    let raw_payload = serde_json::to_value(&paystack_res)
        .map_err(|e| format!("Failed to serialize verification response: {}", e))?;
    update_payment_by_reference(
        pool,
        &paystack_res.data.reference,
        &paystack_res.data.status,
        Some(paystack_res.data.gateway_response.clone()),
        Some(raw_payload),
    )
    .await?;

    Ok(VerifyResponse {
        status: paystack_res.status,
        message: paystack_res.message,
        data: VerifyResponseData {
            status: paystack_res.data.status,
            amount: paystack_res.data.amount,
            reference: paystack_res.data.reference,
            gateway_response: paystack_res.data.gateway_response,
        },
    })
}

// ============================================================================
// Webhook Validation Service
// ============================================================================

pub fn verify_webhook_signature(signature: &str, body: &[u8]) -> Result<bool, String> {
    let api_key = std::env::var("PAYSTACK_API_KEY")
        .map_err(|_| "PAYSTACK_API_KEY not set in environment".to_string())?;

    let mut mac =
        HmacSha512::new_from_slice(api_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(body);

    let result = mac.finalize();
    let expected_signature = encode(result.into_bytes());

    Ok(expected_signature == signature)
}

pub async fn process_webhook_event(pool: &PgPool, payload: Value) -> Result<(), String> {
    let event = payload["event"].as_str().unwrap_or_default();
    let reference = payload["data"]["reference"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    if reference.is_empty() {
        return Err("Webhook payload missing reference".to_string());
    }

    let status = if event == "charge.success" {
        "success".to_string()
    } else if event == "charge.failed" {
        "failed".to_string()
    } else {
        payload["data"]["status"]
            .as_str()
            .unwrap_or("pending")
            .to_string()
    };

    let gateway_response = payload["data"]["gateway_response"]
        .as_str()
        .map(|value| value.to_string());

    update_payment_by_reference(pool, &reference, &status, gateway_response, Some(payload)).await?;

    info!(reference = %reference, status = %status, "Webhook payment update applied");

    Ok(())
}

// ============================================================================
// Database Helpers
// ============================================================================

enum Payer {
    User(String),
    Driver(String),
}

fn resolve_payer(payload: &InitializeRequest) -> Result<Payer, String> {
    match (payload.user_id.as_ref(), payload.driver_id.as_ref()) {
        (Some(user_id), None) => Ok(Payer::User(user_id.clone())),
        (None, Some(driver_id)) => Ok(Payer::Driver(driver_id.clone())),
        (Some(_), Some(_)) => Err("Provide only one of user_id or driver_id".to_string()),
        (None, None) => Err("user_id or driver_id is required".to_string()),
    }
}

fn resolve_currency(payload: &InitializeRequest) -> String {
    payload
        .currency
        .clone()
        .unwrap_or_else(|| "NGN".to_string())
}

async fn store_initialized_payment(
    pool: &PgPool,
    payload: &InitializeRequest,
    paystack_res: &InitializeResponse,
) -> Result<(), String> {
    let payer = resolve_payer(payload)?;
    let currency = resolve_currency(payload);
    let now = Utc::now();
    let payment_id = Uuid::new_v4().to_string();

    match payer {
        Payer::User(user_id) => {
            sqlx::query(
                "INSERT INTO user_payments \
                (id, user_id, email, amount, currency, status, reference, authorization_url, access_code, created_at, updated_at) \
                VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
            )
            .bind(&payment_id)
            .bind(&user_id)
            .bind(&payload.email)
            .bind(&payload.amount)
            .bind(&currency)
            .bind("pending")
            .bind(&paystack_res.data.reference)
            .bind(&paystack_res.data.authorization_url)
            .bind(&paystack_res.data.access_code)
            .bind(now)
            .bind(now)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to store user payment: {}", e))?;
        }
        Payer::Driver(driver_id) => {
            sqlx::query(
                "INSERT INTO driver_payments \
                (id, driver_id, email, amount, currency, status, reference, authorization_url, access_code, created_at, updated_at) \
                VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
            )
            .bind(&payment_id)
            .bind(&driver_id)
            .bind(&payload.email)
            .bind(&payload.amount)
            .bind(&currency)
            .bind("pending")
            .bind(&paystack_res.data.reference)
            .bind(&paystack_res.data.authorization_url)
            .bind(&paystack_res.data.access_code)
            .bind(now)
            .bind(now)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to store driver payment: {}", e))?;
        }
    }

    Ok(())
}

async fn update_payment_by_reference(
    pool: &PgPool,
    reference: &str,
    status: &str,
    gateway_response: Option<String>,
    raw_payload: Option<Value>,
) -> Result<(), String> {
    let now = Utc::now();
    let raw_payload = raw_payload.map(Json);

    let user_result = sqlx::query(
        "UPDATE user_payments \
        SET status = $1, gateway_response = $2, raw_payload = $3, updated_at = $4 \
        WHERE reference = $5",
    )
    .bind(status)
    .bind(gateway_response.clone())
    .bind(raw_payload.clone())
    .bind(now)
    .bind(reference)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to update user payment: {}", e))?;

    let driver_result = sqlx::query(
        "UPDATE driver_payments \
        SET status = $1, gateway_response = $2, raw_payload = $3, updated_at = $4 \
        WHERE reference = $5",
    )
    .bind(status)
    .bind(gateway_response)
    .bind(raw_payload)
    .bind(now)
    .bind(reference)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to update driver payment: {}", e))?;

    if user_result.rows_affected() == 0 && driver_result.rows_affected() == 0 {
        return Err("No payment found for reference".to_string());
    }

    Ok(())
}

// ============================================================================
// Payment List Services
// ============================================================================

pub async fn list_all_payments_service(pool: &PgPool) -> Result<Vec<PaymentResponse>, String> {
    debug!("Fetching all payments from database");

    let mut payments = Vec::new();

    // Fetch user payments
    let user_rows = sqlx::query(
        "SELECT id, user_id, email, amount, currency, status, reference, \
        authorization_url, access_code, gateway_response, created_at, updated_at \
        FROM user_payments ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Failed to fetch user payments");
        format!("Database fetch failed: {}", e)
    })?;

    for row in user_rows {
        payments.push(PaymentResponse {
            id: row.get("id"),
            payer_type: "user".to_string(),
            payer_id: row.get("user_id"),
            email: row.get("email"),
            amount: row.get("amount"),
            currency: row.get("currency"),
            status: row.get("status"),
            reference: row.get("reference"),
            authorization_url: row.get("authorization_url"),
            access_code: row.get("access_code"),
            gateway_response: row.get("gateway_response"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    // Fetch driver payments
    let driver_rows = sqlx::query(
        "SELECT id, driver_id, email, amount, currency, status, reference, \
        authorization_url, access_code, gateway_response, created_at, updated_at \
        FROM driver_payments ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Failed to fetch driver payments");
        format!("Database fetch failed: {}", e)
    })?;

    for row in driver_rows {
        payments.push(PaymentResponse {
            id: row.get("id"),
            payer_type: "driver".to_string(),
            payer_id: row.get("driver_id"),
            email: row.get("email"),
            amount: row.get("amount"),
            currency: row.get("currency"),
            status: row.get("status"),
            reference: row.get("reference"),
            authorization_url: row.get("authorization_url"),
            access_code: row.get("access_code"),
            gateway_response: row.get("gateway_response"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    // Sort by created_at descending
    payments.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    info!(count = payments.len(), "Payments fetched successfully");

    Ok(payments)
}

pub async fn list_user_payments_service(
    pool: &PgPool,
    user_id: String,
) -> Result<Vec<PaymentResponse>, String> {
    debug!(user_id = %user_id, "Fetching user payments from database");

    let rows = sqlx::query(
        "SELECT id, user_id, email, amount, currency, status, reference, \
        authorization_url, access_code, gateway_response, created_at, updated_at \
        FROM user_payments WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(&user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Failed to fetch user payments");
        format!("Database fetch failed: {}", e)
    })?;

    let payments: Vec<PaymentResponse> = rows
        .into_iter()
        .map(|row| PaymentResponse {
            id: row.get("id"),
            payer_type: "user".to_string(),
            payer_id: row.get("user_id"),
            email: row.get("email"),
            amount: row.get("amount"),
            currency: row.get("currency"),
            status: row.get("status"),
            reference: row.get("reference"),
            authorization_url: row.get("authorization_url"),
            access_code: row.get("access_code"),
            gateway_response: row.get("gateway_response"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    info!(count = payments.len(), user_id = %user_id, "User payments fetched successfully");

    Ok(payments)
}

pub async fn list_driver_payments_service(
    pool: &PgPool,
    driver_id: String,
) -> Result<Vec<PaymentResponse>, String> {
    debug!(driver_id = %driver_id, "Fetching driver payments from database");

    let rows = sqlx::query(
        "SELECT id, driver_id, email, amount, currency, status, reference, \
        authorization_url, access_code, gateway_response, created_at, updated_at \
        FROM driver_payments WHERE driver_id = $1 ORDER BY created_at DESC",
    )
    .bind(&driver_id)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!(error = %e, "Failed to fetch driver payments");
        format!("Database fetch failed: {}", e)
    })?;

    let payments: Vec<PaymentResponse> = rows
        .into_iter()
        .map(|row| PaymentResponse {
            id: row.get("id"),
            payer_type: "driver".to_string(),
            payer_id: row.get("driver_id"),
            email: row.get("email"),
            amount: row.get("amount"),
            currency: row.get("currency"),
            status: row.get("status"),
            reference: row.get("reference"),
            authorization_url: row.get("authorization_url"),
            access_code: row.get("access_code"),
            gateway_response: row.get("gateway_response"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    info!(count = payments.len(), driver_id = %driver_id, "Driver payments fetched successfully");

    Ok(payments)
}
