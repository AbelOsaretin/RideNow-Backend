use crate::handlers::login_handler::login_handler;
use axum::{Router, routing::post};

pub fn login_route() -> Router {
    Router::new().route("/", post(login_handler))
}
