use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Deserialize)]
pub struct TestNotificationRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct TestNotificationResponse {
    pub success: bool,
    pub error: Option<String>,
}

pub async fn test_notification_handler(
    State(state): State<AppState>,
    Json(payload): Json<TestNotificationRequest>,
) -> (StatusCode, Json<TestNotificationResponse>) {
    match state.notifier.send(&payload.message).await {
        Ok(_) => (
            StatusCode::OK,
            Json(TestNotificationResponse {
                success: true,
                error: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(TestNotificationResponse {
                success: false,
                error: Some(e.to_string()),
            }),
        ),
    }
}
