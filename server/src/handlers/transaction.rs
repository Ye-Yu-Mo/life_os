use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use uuid::Uuid;

use crate::errors::ServiceError;
use crate::middleware::auth::AuthUser;
use crate::services::transaction::{
    self, CreateTransactionRequest, TransactionQuery, TransactionResponse,
    UpdateTransactionRequest,
};
use crate::state::AppState;

pub async fn create_transaction_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionResponse>, ServiceError> {
    let txn = transaction::create_transaction(&state.db, user.id, payload).await?;
    Ok(Json(txn))
}

pub async fn get_transaction_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(txn_id): Path<Uuid>,
) -> Result<Json<TransactionResponse>, ServiceError> {
    let txn = transaction::get_transaction(&state.db, user.id, txn_id).await?;
    Ok(Json(txn))
}

pub async fn list_transactions_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Query(filter): Query<TransactionQuery>,
) -> Result<Json<Vec<TransactionResponse>>, ServiceError> {
    let txns = transaction::list_transactions(&state.db, user.id, filter).await?;
    Ok(Json(txns))
}

pub async fn update_transaction_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(txn_id): Path<Uuid>,
    Json(payload): Json<UpdateTransactionRequest>,
) -> Result<Json<TransactionResponse>, ServiceError> {
    let txn =
        transaction::update_transaction(&state.db, user.id, txn_id, payload).await?;
    Ok(Json(txn))
}

pub async fn delete_transaction_handler(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Path(txn_id): Path<Uuid>,
) -> Result<Json<()>, ServiceError> {
    transaction::delete_transaction(&state.db, user.id, txn_id).await?;
    Ok(Json(()))
}
