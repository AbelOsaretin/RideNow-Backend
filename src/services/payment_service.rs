use crate::models::payment_model::{
    InitializeDataResponse, InitializeRequest, InitializeResponse, VerifyResponse,
    VerifyResponseData,
};
use hex::encode;
use hmac::{Hmac, Mac};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use sha2::Sha512;
use tracing::{debug, error, info};

type HmacSha512 = Hmac<Sha512>;

// ============================================================================
// Payment Initialization Services
// ============================================================================

pub async fn initialize_payment_service(
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
    let response = client
        .post(&url)
        .headers(headers)
        .json(&payload)
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

pub async fn get_payment_redirect_url(payload: InitializeRequest) -> Result<String, String> {
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
    let response = client
        .post(&url)
        .headers(headers)
        .json(&payload)
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

    Ok(paystack_res.data.authorization_url)
}

// ============================================================================
// Payment Verification Service
// ============================================================================

pub async fn verify_payment_service(reference: String) -> Result<VerifyResponse, String> {
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

pub fn process_webhook_event(payload: serde_json::Value) -> Result<(), String> {
    if payload["event"] == "charge.success" {
        let reference = payload["data"]["reference"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        info!(reference = %reference, "Payment successful via webhook");

        // TODO: Add your business logic here (e.g., update database, send notifications)

        Ok(())
    } else {
        debug!(event = %payload["event"], "Unhandled webhook event");
        Ok(())
    }
}
