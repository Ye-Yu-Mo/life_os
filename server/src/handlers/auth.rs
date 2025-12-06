use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AuthError;
use crate::services::auth::{self, LoginRequest, RegisterRequest};
use crate::state::AppState;
use crate::utils::jwt;

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponsePayload {
    pub id: Uuid,
    pub username: String,
    pub token: String,
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<UserResponsePayload>, AuthError> {
    let user = auth::register(
        &state.db,
        RegisterRequest {
            username: payload.username,
            password: payload.password,
        },
    )
    .await?;

    let token = jwt::sign(user.id)?;

    Ok(Json(UserResponsePayload {
        id: user.id,
        username: user.username,
        token,
    }))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<UserResponsePayload>, AuthError> {
    let user = auth::login(
        &state.db,
        LoginRequest {
            username: payload.username,
            password: payload.password,
        },
    )
    .await?;

    let token = jwt::sign(user.id)?;

    Ok(Json(UserResponsePayload {
        id: user.id,
        username: user.username,
        token,
    }))
}
