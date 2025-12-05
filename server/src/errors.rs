use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Registration failed")]
    RegistrationFailed,

    #[error("Password hash error")]
    PasswordHashError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let error_msg = self.to_string();
        let (status, message) = match self {
            AuthError::AuthenticationFailed => (StatusCode::UNAUTHORIZED, "Authentication failed"),
            AuthError::RegistrationFailed => (StatusCode::BAD_REQUEST, "Registration failed"),
            AuthError::PasswordHashError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AuthError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        error!(error = %error_msg, status = %status, "request failed");

        (status, Json(json!({ "error": message }))).into_response()
    }
}
