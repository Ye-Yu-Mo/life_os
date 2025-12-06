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

    #[error("Invalid token")]
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let error_msg = self.to_string();
        let (status, message) = match self {
            AuthError::AuthenticationFailed => (StatusCode::UNAUTHORIZED, "Authentication failed"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::RegistrationFailed => (StatusCode::BAD_REQUEST, "Registration failed"),
            AuthError::PasswordHashError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AuthError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        error!(error = %error_msg, status = %status, "request failed");

        (status, Json(json!({ "error": message }))).into_response()
    }
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Resource not found")]
    NotFound,

    #[error("Access forbidden")]
    Forbidden,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ServiceError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            ServiceError::Forbidden => (StatusCode::FORBIDDEN, "Access forbidden".to_string()),
            ServiceError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ServiceError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            ServiceError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        error!(error = %self, status = %status, "request failed");

        (status, Json(json!({ "error": message }))).into_response()
    }
}

