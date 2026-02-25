use axum::{
    Json,
    body::Bytes,
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::Redirect,
};
use tracing::{error, info};

use crate::models::payment_model::{InitializeRequest, InitializeResponse, VerifyResponse};
use crate::services::payment_service::{
    get_payment_redirect_url, initialize_payment_service, process_webhook_event,
    verify_payment_service, verify_webhook_signature,
};

// ============================================================================
// Health Check Handler
// ============================================================================

pub async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Payment Service OK 🦀")
}

// ============================================================================
// Payment Initialization Handlers
// ============================================================================

pub async fn initialize_payment(
    Json(payload): Json<InitializeRequest>,
) -> Result<Json<InitializeResponse>, StatusCode> {
    info!(email = %payload.email, amount = %payload.amount, "Initializing payment");

    match initialize_payment_service(payload).await {
        Ok(response) => {
            info!(reference = %response.data.reference, "Payment initialization successful");
            Ok(Json(response))
        }
        Err(e) => {
            error!(error = %e, "Failed to initialize payment");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn initialize_payment_redirect(
    Json(payload): Json<InitializeRequest>,
) -> Result<Redirect, StatusCode> {
    info!(email = %payload.email, amount = %payload.amount, "Getting payment redirect");

    match get_payment_redirect_url(payload).await {
        Ok(url) => {
            info!(url = %url, "Redirecting to payment page");
            Ok(Redirect::to(&url))
        }
        Err(e) => {
            error!(error = %e, "Failed to get redirect URL");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ============================================================================
// Payment Verification Handler
// ============================================================================

pub async fn verify_payment(Path(reference): Path<String>) -> (StatusCode, Json<VerifyResponse>) {
    info!(reference = %reference, "Verifying payment");

    match verify_payment_service(reference).await {
        Ok(response) => {
            info!(
                reference = %response.data.reference,
                status = %response.data.status,
                "Payment verification successful"
            );
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            error!(error = %e, "Failed to verify payment");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(VerifyResponse {
                status: false,
                message: "Payment verification failed".to_string(),
                data: crate::models::payment_model::VerifyResponseData {
                    status: "failed".to_string(),
                    amount: 0,
                    reference: "".to_string(),
                    gateway_response: "".to_string(),
                },
            }))
        }
    }
}

// ============================================================================
// Webhook Handler
// ============================================================================

pub async fn paystack_webhook(headers: HeaderMap, body: Bytes) -> StatusCode {
    info!("Received webhook event");

    // 1. Get the signature from headers
    let signature = match headers.get("x-paystack-signature") {
        Some(sig) => match sig.to_str() {
            Ok(s) => s,
            Err(_) => {
                error!("Invalid signature header format");
                return StatusCode::BAD_REQUEST;
            }
        },
        None => {
            error!("Missing x-paystack-signature header");
            return StatusCode::BAD_REQUEST;
        }
    };

    // 2. Verify signature
    match verify_webhook_signature(signature, &body) {
        Ok(is_valid) => {
            if !is_valid {
                error!("Invalid webhook signature");
                return StatusCode::UNAUTHORIZED;
            }
        }
        Err(e) => {
            error!(error = %e, "Failed to verify webhook signature");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }

    // 3. Parse and process the webhook payload
    match serde_json::from_slice::<serde_json::Value>(&body) {
        Ok(payload) => {
            if let Err(e) = process_webhook_event(payload) {
                error!(error = %e, "Failed to process webhook event");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            info!("Webhook processed successfully");
            StatusCode::OK
        }
        Err(e) => {
            error!(error = %e, "Failed to parse webhook payload");
            StatusCode::BAD_REQUEST
        }
    }
}
