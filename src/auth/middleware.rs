use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use tracing::{debug, error};

use super::password_utils::verify_token;

/// Middleware to verify JWT token from Authorization header
pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, Response> {
    debug!("Running auth middleware");

    // Extract Authorization header
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let auth_header = match auth_header {
        Some(header) => header,
        None => {
            error!("Missing Authorization header");
            return Err((StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response());
        }
    };

    // Extract token from "Bearer <token>"
    let token = match auth_header.strip_prefix("Bearer ") {
        Some(token) => token,
        None => {
            error!("Invalid Authorization header format");
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header format. Expected: Bearer <token>",
            )
                .into_response());
        }
    };

    // Verify token
    let claims = match verify_token(token) {
        Ok(claims) => {
            debug!(user_id = %claims.sub, role = %claims.role, "Token verified successfully");
            claims
        }
        Err(e) => {
            error!(error = %e, "Token verification failed");
            return Err((StatusCode::UNAUTHORIZED, "Invalid or expired token").into_response());
        }
    };

    // Insert claims into request extensions so handlers can access it
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// Optional middleware that allows requests without auth but adds claims if token is present
pub async fn optional_auth_middleware(mut req: Request, next: Next) -> Result<Response, Response> {
    debug!("Running optional auth middleware");

    if let Some(auth_header) = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
    {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            if let Ok(claims) = verify_token(token) {
                debug!(user_id = %claims.sub, role = %claims.role, "Token verified successfully");
                req.extensions_mut().insert(claims);
            }
        }
    }

    Ok(next.run(req).await)
}
