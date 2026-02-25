use crate::handlers::payment_handlers::{
    health_check, initialize_payment, initialize_payment_redirect, paystack_webhook, verify_payment,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn payment_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/initialize", post(initialize_payment))
        .route("/initialize/redirect", post(initialize_payment_redirect))
        .route("/verify/{reference}", get(verify_payment))
        .route("/webhook", post(paystack_webhook))
}
