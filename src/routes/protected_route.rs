use axum::{Extension, Router, middleware, response::IntoResponse, routing::get};
use tracing::info;

use crate::auth::{middleware::auth_middleware, password_utils::Claims};

// Protected handler - users can access their own profile
pub async fn get_profile(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    info!(user_id = %claims.sub, "User accessing profile");
    format!("Profile for user: {}, Role: {}", claims.sub, claims.role)
}

// Protected handler - only admins can access
pub async fn admin_dashboard(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    info!(user_id = %claims.sub, role = %claims.role, "Admin accessing dashboard");

    if claims.role != "admin" {
        return "Access denied. Admin role required.".to_string();
    }

    format!("Welcome to admin dashboard, {}", claims.sub)
}

// Protected handler - get user settings
pub async fn get_settings(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    info!(user_id = %claims.sub, "User accessing settings");
    format!("Settings for user: {}", claims.sub)
}

pub fn protected_routes() -> Router {
    Router::new()
        .route("/profile", get(get_profile))
        .route("/settings", get(get_settings))
        .route("/admin/dashboard", get(admin_dashboard))
        .layer(middleware::from_fn(auth_middleware))
}
