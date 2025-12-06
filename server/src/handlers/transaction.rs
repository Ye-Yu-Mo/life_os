use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::errors::ServiceError;
use crate::services::transaction::{
    self, CreateTransactionRequest, TransactionQuery, TransactionResponse,
    UpdateTransactionRequest,
};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct UserIdQuery {
    pub user_id: Uuid,
}

pub async fn create_transaction_handler(
    State(state): State<AppState>,
    Query(query): Query<UserIdQuery>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionResponse>, ServiceError> {
    let txn = transaction::create_transaction(&state.db, query.user_id, payload).await?;
    Ok(Json(txn))
}

pub async fn get_transaction_handler(
    State(state): State<AppState>,
    Query(query): Query<UserIdQuery>,
    Path(txn_id): Path<Uuid>,
) -> Result<Json<TransactionResponse>, ServiceError> {
    let txn = transaction::get_transaction(&state.db, query.user_id, txn_id).await?;
    Ok(Json(txn))
}

pub async fn list_transactions_handler(
    State(state): State<AppState>,
    Query(user_query): Query<UserIdQuery>,
    Query(filter): Query<TransactionQuery>,
) -> Result<Json<Vec<TransactionResponse>>, ServiceError> {
    let txns = transaction::list_transactions(&state.db, user_query.user_id, filter).await?;
    Ok(Json(txns))
}

pub async fn update_transaction_handler(
    State(state): State<AppState>,
    Query(query): Query<UserIdQuery>,
    Path(txn_id): Path<Uuid>,
    Json(payload): Json<UpdateTransactionRequest>,
) -> Result<Json<TransactionResponse>, ServiceError> {
    let txn =
        transaction::update_transaction(&state.db, query.user_id, txn_id, payload).await?;
    Ok(Json(txn))
}

pub async fn delete_transaction_handler(
    State(state): State<AppState>,
    Query(query): Query<UserIdQuery>,
    Path(txn_id): Path<Uuid>,
) -> Result<Json<()>, ServiceError> {
    transaction::delete_transaction(&state.db, query.user_id, txn_id).await?;
    Ok(Json(()))
}
