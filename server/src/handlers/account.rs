use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::errors::ServiceError;
use crate::services::account::{
    self, AccountResponse, CreateAccountRequest, UpdateAccountRequest,
};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct UserIdQuery {
    pub user_id: Uuid,
}

pub async fn create_account_handler(
    State(state): State<AppState>,
    Query(query): Query<UserIdQuery>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<Json<AccountResponse>, ServiceError> {
    let account = account::create_account(&state.db, query.user_id, payload).await?;
    Ok(Json(account))
}

pub async fn get_account_handler(
    State(state): State<AppState>,
    Query(query): Query<UserIdQuery>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<AccountResponse>, ServiceError> {
    let account = account::get_account(&state.db, query.user_id, account_id).await?;
    Ok(Json(account))
}

pub async fn list_accounts_handler(
    State(state): State<AppState>,
    Query(query): Query<UserIdQuery>,
) -> Result<Json<Vec<AccountResponse>>, ServiceError> {
    let accounts = account::list_accounts(&state.db, query.user_id).await?;
    Ok(Json(accounts))
}

pub async fn update_account_handler(
    State(state): State<AppState>,
    Query(query): Query<UserIdQuery>,
    Path(account_id): Path<Uuid>,
    Json(payload): Json<UpdateAccountRequest>,
) -> Result<Json<AccountResponse>, ServiceError> {
    let account =
        account::update_account(&state.db, query.user_id, account_id, payload).await?;
    Ok(Json(account))
}

pub async fn delete_account_handler(
    State(state): State<AppState>,
    Query(query): Query<UserIdQuery>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<()>, ServiceError> {
    account::delete_account(&state.db, query.user_id, account_id).await?;
    Ok(Json(()))
}
