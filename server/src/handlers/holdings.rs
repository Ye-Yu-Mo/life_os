use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::errors::ServiceError;
use crate::middleware::auth::AuthUser;
use crate::services::holdings::{
    self, CreateHoldingsRequest, HoldingsResponse, UpdateHoldingsRequest,
};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct HoldingsQuery {
    pub account_id: Option<Uuid>,
    pub asset_type: Option<String>,
}

pub async fn create_holdings_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(payload): Json<CreateHoldingsRequest>,
) -> Result<Json<HoldingsResponse>, ServiceError> {
    let holding = holdings::create_holdings(&state.db, user.id, payload).await?;
    Ok(Json(holding))
}

pub async fn get_holdings_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(holdings_id): Path<Uuid>,
) -> Result<Json<HoldingsResponse>, ServiceError> {
    let holding = holdings::get_holdings(&state.db, user.id, holdings_id).await?;
    Ok(Json(holding))
}

pub async fn list_holdings_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Query(query): Query<HoldingsQuery>,
) -> Result<Json<Vec<HoldingsResponse>>, ServiceError> {
    let holdings_list =
        holdings::list_holdings(&state.db, user.id, query.account_id, query.asset_type)
            .await?;
    Ok(Json(holdings_list))
}

pub async fn update_holdings_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(holdings_id): Path<Uuid>,
    Json(payload): Json<UpdateHoldingsRequest>,
) -> Result<Json<HoldingsResponse>, ServiceError> {
    let holding =
        holdings::update_holdings(&state.db, user.id, holdings_id, payload).await?;
    Ok(Json(holding))
}

pub async fn delete_holdings_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(holdings_id): Path<Uuid>,
) -> Result<Json<()>, ServiceError> {
    holdings::delete_holdings(&state.db, user.id, holdings_id).await?;
    Ok(Json(()))
}
