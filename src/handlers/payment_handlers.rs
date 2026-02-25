use axum::{
    Json,
    body::Bytes,
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::Redirect,
    Extension,
};
use sqlx::PgPool;
use tracing::{error, info};

use crate::models::payment_model::{InitializeRequest, InitializeResponse, PaymentResponse, VerifyResponse};
use crate::services::payment_service::{
    get_payment_redirect_url, initialize_payment_service, list_all_payments_service,
    list_driver_payments_service, list_user_payments_service, process_webhook_event,
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
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<InitializeRequest>,
) -> Result<Json<InitializeResponse>, StatusCode> {
    info!(email = %payload.email, amount = %payload.amount, "Initializing payment");

    match initialize_payment_service(&pool, payload).await {
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
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<InitializeRequest>,
) -> Result<Redirect, StatusCode> {
    info!(email = %payload.email, amount = %payload.amount, "Getting payment redirect");

    match get_payment_redirect_url(&pool, payload).await {
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

pub async fn verify_payment(
    Extension(pool): Extension<PgPool>,
    Path(reference): Path<String>,
) -> (StatusCode, Json<VerifyResponse>) {
    info!(reference = %reference, "Verifying payment");

    match verify_payment_service(&pool, reference).await {
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

pub async fn paystack_webhook(
    Extension(pool): Extension<PgPool>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
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
            if let Err(e) = process_webhook_event(&pool, payload).await {
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

// ============================================================================
// Payment List Handlers
// ============================================================================

pub async fn list_all_payments(
    Extension(pool): Extension<PgPool>,
) -> (StatusCode, Json<Vec<PaymentResponse>>) {
    info!("Fetching all payments");

    match list_all_payments_service(&pool).await {
        Ok(payments) => {
            info!(count = payments.len(), "All payments fetched successfully");
            (StatusCode::OK, Json(payments))
        }
        Err(e) => {
            error!(error = %e, "Failed to fetch payments");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn list_user_payments(
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<String>,
) -> (StatusCode, Json<Vec<PaymentResponse>>) {
    info!(user_id = %user_id, "Fetching user payments");

    match list_user_payments_service(&pool, user_id.clone()).await {
        Ok(payments) => {
            info!(count = payments.len(), user_id = %user_id, "User payments fetched successfully");
            (StatusCode::OK, Json(payments))
        }
        Err(e) => {
            error!(error = %e, user_id = %user_id, "Failed to fetch user payments");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn list_driver_payments(
    Extension(pool): Extension<PgPool>,
    Path(driver_id): Path<String>,
) -> (StatusCode, Json<Vec<PaymentResponse>>) {
    info!(driver_id = %driver_id, "Fetching driver payments");

    match list_driver_payments_service(&pool, driver_id.clone()).await {
        Ok(payments) => {
            info!(count = payments.len(), driver_id = %driver_id, "Driver payments fetched successfully");
            (StatusCode::OK, Json(payments))
        }
        Err(e) => {
            error!(error = %e, driver_id = %driver_id, "Failed to fetch driver payments");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}
