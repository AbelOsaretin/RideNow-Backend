use crate::handlers::payment_handlers::{
    health_check, initialize_payment, initialize_payment_redirect, list_all_payments,
    list_driver_payments, list_user_payments, paystack_webhook, verify_payment,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn payment_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/", get(list_all_payments))
        .route("/initialize", post(initialize_payment))
        .route("/initialize/redirect", post(initialize_payment_redirect))
        .route("/verify/{reference}", get(verify_payment))
        .route("/webhook", post(paystack_webhook))
        .route("/user/{user_id}", get(list_user_payments))
        .route("/driver/{driver_id}", get(list_driver_payments))
}
