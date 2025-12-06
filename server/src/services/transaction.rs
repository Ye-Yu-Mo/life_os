use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    Order, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{prelude::*, transaction};
use crate::errors::ServiceError;

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub from_account_id: Option<Uuid>,
    pub to_account_id: Option<Uuid>,
    pub txn_type: String,
    pub amount: Decimal,
    pub currency_code: String,
    pub to_amount: Option<Decimal>,
    pub to_currency_code: Option<String>,
    pub category: Option<String>,
    pub note: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub ref_transaction_id: Option<Uuid>,
    pub merchant: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTransactionRequest {
    pub category: Option<String>,
    pub note: Option<String>,
    pub occurred_at: Option<DateTime<Utc>>,
    pub merchant: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub from_account_id: Option<Uuid>,
    pub to_account_id: Option<Uuid>,
    pub txn_type: String,
    pub amount: Decimal,
    pub currency_code: String,
    pub to_amount: Option<Decimal>,
    pub to_currency_code: Option<String>,
    pub category: Option<String>,
    pub note: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub ref_transaction_id: Option<Uuid>,
    pub merchant: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<transaction::Model> for TransactionResponse {
    fn from(model: transaction::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            from_account_id: model.from_account_id,
            to_account_id: model.to_account_id,
            txn_type: model.txn_type,
            amount: model.amount,
            currency_code: model.currency_code,
            to_amount: model.to_amount,
            to_currency_code: model.to_currency_code,
            category: model.category,
            note: model.note,
            occurred_at: model.occurred_at.with_timezone(&Utc),
            ref_transaction_id: model.ref_transaction_id,
            merchant: model.merchant,
            created_at: model.created_at.with_timezone(&Utc),
            updated_at: model.updated_at.with_timezone(&Utc),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TransactionQuery {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub category: Option<String>,
    pub account_id: Option<Uuid>,
    pub min_amount: Option<Decimal>,
    pub max_amount: Option<Decimal>,
    pub keyword: Option<String>,
    pub txn_type: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

const VALID_TXN_TYPES: &[&str] = &["expense", "income", "transfer", "refund", "adjustment"];

fn validate_txn_type(t: &str) -> Result<(), ServiceError> {
    if !VALID_TXN_TYPES.contains(&t) {
        return Err(ServiceError::Validation(format!("Invalid transaction type: {}", t)));
    }
    Ok(())
}

fn validate_currency_code(code: &str) -> Result<(), ServiceError> {
    if code.len() != 3 || !code.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(ServiceError::Validation(
            "Currency code must be 3 letters".to_string(),
        ));
    }
    Ok(())
}

async fn load_owned_transaction(
    db: &DatabaseConnection,
    user_id: Uuid,
    txn_id: Uuid,
) -> Result<transaction::Model, ServiceError> {
    let txn = Transaction::find_by_id(txn_id)
        .one(db)
        .await?
        .ok_or(ServiceError::NotFound)?;

    if txn.user_id != user_id {
        return Err(ServiceError::Forbidden);
    }

    Ok(txn)
}

async fn verify_account_ownership(
    db: &DatabaseConnection,
    user_id: Uuid,
    account_id: Uuid,
) -> Result<(), ServiceError> {
    let account = Account::find_by_id(account_id)
        .one(db)
        .await?
        .ok_or(ServiceError::Validation(format!(
            "Account {} not found",
            account_id
        )))?;

    if account.user_id != user_id {
        return Err(ServiceError::Forbidden);
    }

    Ok(())
}

pub async fn create_transaction(
    db: &DatabaseConnection,
    user_id: Uuid,
    req: CreateTransactionRequest,
) -> Result<TransactionResponse, ServiceError> {
    if req.amount <= Decimal::ZERO {
        return Err(ServiceError::Validation("Amount must be positive".to_string()));
    }

    let txn_type = req.txn_type.trim().to_lowercase();
    validate_txn_type(&txn_type)?;

    let currency = req.currency_code.trim().to_uppercase();
    validate_currency_code(&currency)?;

    match txn_type.as_str() {
        "transfer" => {
            let from = req.from_account_id.ok_or(ServiceError::Validation(
                "Transfer must have from_account_id".to_string(),
            ))?;
            let to = req.to_account_id.ok_or(ServiceError::Validation(
                "Transfer must have to_account_id".to_string(),
            ))?;

            if from == to {
                return Err(ServiceError::Validation(
                    "Transfer from and to accounts must be different".to_string(),
                ));
            }

            verify_account_ownership(db, user_id, from).await?;
            verify_account_ownership(db, user_id, to).await?;

            let to_currency = req.to_currency_code.as_ref().map(|c| c.trim().to_uppercase());
            if let Some(ref tc) = to_currency {
                validate_currency_code(tc)?;
            }

            if (req.to_amount.is_some() && to_currency.is_none())
                || (req.to_amount.is_none() && to_currency.is_some())
            {
                return Err(ServiceError::Validation(
                    "to_amount and to_currency_code must both be present or both be absent".to_string(),
                ));
            }

            if let Some(to_amt) = req.to_amount {
                if to_amt <= Decimal::ZERO {
                    return Err(ServiceError::Validation(
                        "To amount must be positive".to_string(),
                    ));
                }
            }

            let now = Utc::now().into();
            let txn = transaction::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user_id),
                from_account_id: Set(Some(from)),
                to_account_id: Set(Some(to)),
                txn_type: Set(txn_type),
                amount: Set(req.amount),
                currency_code: Set(currency),
                to_amount: Set(req.to_amount),
                to_currency_code: Set(to_currency),
                category: Set(req.category),
                note: Set(req.note),
                occurred_at: Set(req.occurred_at.into()),
                ref_transaction_id: Set(None),
                merchant: Set(req.merchant),
                created_at: Set(now),
                updated_at: Set(now),
            };

            let model = txn.insert(db).await?;
            return Ok(TransactionResponse::from(model));
        }
        "refund" | "adjustment" => {
            if req.to_amount.is_some() || req.to_currency_code.is_some() {
                return Err(ServiceError::Validation(
                    "Refund/adjustment cannot have to_amount or to_currency_code".to_string(),
                ));
            }

            let ref_txn_id = req.ref_transaction_id.ok_or(ServiceError::Validation(
                format!("{} must have ref_transaction_id", txn_type),
            ))?;

            let ref_txn = load_owned_transaction(db, user_id, ref_txn_id).await?;

            if ref_txn.currency_code != currency {
                return Err(ServiceError::Validation(
                    "Refund/adjustment currency must match original transaction".to_string(),
                ));
            }

            if req.from_account_id.is_some() && req.to_account_id.is_some() {
                return Err(ServiceError::Validation(
                    "Refund/adjustment must have only one of from_account_id or to_account_id".to_string(),
                ));
            }

            if req.from_account_id.is_none() && req.to_account_id.is_none() {
                return Err(ServiceError::Validation(
                    "Refund/adjustment must have from_account_id or to_account_id".to_string(),
                ));
            }

            if let Some(from_id) = req.from_account_id {
                verify_account_ownership(db, user_id, from_id).await?;
            }
            if let Some(to_id) = req.to_account_id {
                verify_account_ownership(db, user_id, to_id).await?;
            }

            let now = Utc::now().into();
            let txn = transaction::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user_id),
                from_account_id: Set(req.from_account_id),
                to_account_id: Set(req.to_account_id),
                txn_type: Set(txn_type),
                amount: Set(req.amount),
                currency_code: Set(currency),
                to_amount: Set(None),
                to_currency_code: Set(None),
                category: Set(req.category),
                note: Set(req.note),
                occurred_at: Set(req.occurred_at.into()),
                ref_transaction_id: Set(Some(ref_txn_id)),
                merchant: Set(req.merchant),
                created_at: Set(now),
                updated_at: Set(now),
            };

            let model = txn.insert(db).await?;
            return Ok(TransactionResponse::from(model));
        }
        _ => {
            if req.to_amount.is_some() || req.to_currency_code.is_some() {
                return Err(ServiceError::Validation(
                    "Non-transfer transaction cannot have to_amount or to_currency_code".to_string(),
                ));
            }

            if req.ref_transaction_id.is_some() {
                return Err(ServiceError::Validation(
                    "Non-refund/adjustment transaction cannot have ref_transaction_id".to_string(),
                ));
            }

            if req.from_account_id.is_some() && req.to_account_id.is_some() {
                return Err(ServiceError::Validation(
                    "Non-transfer transaction cannot have both from and to accounts".to_string(),
                ));
            }

            if req.from_account_id.is_none() && req.to_account_id.is_none() {
                return Err(ServiceError::Validation(
                    "Transaction must have from_account_id or to_account_id".to_string(),
                ));
            }

            if let Some(from_id) = req.from_account_id {
                verify_account_ownership(db, user_id, from_id).await?;
            }
            if let Some(to_id) = req.to_account_id {
                verify_account_ownership(db, user_id, to_id).await?;
            }

            let now = Utc::now().into();
            let txn = transaction::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user_id),
                from_account_id: Set(req.from_account_id),
                to_account_id: Set(req.to_account_id),
                txn_type: Set(txn_type),
                amount: Set(req.amount),
                currency_code: Set(currency),
                to_amount: Set(None),
                to_currency_code: Set(None),
                category: Set(req.category),
                note: Set(req.note),
                occurred_at: Set(req.occurred_at.into()),
                ref_transaction_id: Set(None),
                merchant: Set(req.merchant),
                created_at: Set(now),
                updated_at: Set(now),
            };

            let model = txn.insert(db).await?;
            return Ok(TransactionResponse::from(model));
        }
    }
}

pub async fn get_transaction(
    db: &DatabaseConnection,
    user_id: Uuid,
    txn_id: Uuid,
) -> Result<TransactionResponse, ServiceError> {
    let txn = load_owned_transaction(db, user_id, txn_id).await?;
    Ok(TransactionResponse::from(txn))
}

pub async fn list_transactions(
    db: &DatabaseConnection,
    user_id: Uuid,
    filter: TransactionQuery,
) -> Result<Vec<TransactionResponse>, ServiceError> {
    let mut query = Transaction::find().filter(transaction::Column::UserId.eq(user_id));

    if let Some(start) = filter.start {
        query = query.filter(transaction::Column::OccurredAt.gte(start));
    }
    if let Some(end) = filter.end {
        query = query.filter(transaction::Column::OccurredAt.lte(end));
    }
    if let Some(category) = filter.category {
        query = query.filter(transaction::Column::Category.eq(category));
    }
    if let Some(account_id) = filter.account_id {
        query = query.filter(
            Condition::any()
                .add(transaction::Column::FromAccountId.eq(account_id))
                .add(transaction::Column::ToAccountId.eq(account_id)),
        );
    }
    if let Some(min) = filter.min_amount {
        query = query.filter(transaction::Column::Amount.gte(min));
    }
    if let Some(max) = filter.max_amount {
        query = query.filter(transaction::Column::Amount.lte(max));
    }
    if let Some(keyword) = filter.keyword {
        if keyword.len() > 100 {
            return Err(ServiceError::Validation("Keyword too long".to_string()));
        }
        let pattern = format!("%{}%", keyword.replace('%', "\\%").replace('_', "\\_"));
        query = query.filter(
            Condition::any()
                .add(transaction::Column::Note.like(&pattern))
                .add(transaction::Column::Merchant.like(&pattern)),
        );
    }
    if let Some(txn_type) = filter.txn_type {
        query = query.filter(transaction::Column::TxnType.eq(txn_type.to_lowercase()));
    }

    query = query.order_by(transaction::Column::OccurredAt, Order::Desc);

    let page = filter.offset.unwrap_or(0) / filter.limit.unwrap_or(100);
    let limit = filter.limit.unwrap_or(100).min(500);

    let paginator = query.paginate(db, limit);
    let transactions = paginator.fetch_page(page).await?;

    Ok(transactions.into_iter().map(TransactionResponse::from).collect())
}

pub async fn update_transaction(
    db: &DatabaseConnection,
    user_id: Uuid,
    txn_id: Uuid,
    req: UpdateTransactionRequest,
) -> Result<TransactionResponse, ServiceError> {
    let txn = load_owned_transaction(db, user_id, txn_id).await?;

    let mut active: transaction::ActiveModel = txn.into();

    if let Some(category) = req.category {
        active.category = Set(Some(category));
    }
    if let Some(note) = req.note {
        active.note = Set(Some(note));
    }
    if let Some(occurred_at) = req.occurred_at {
        active.occurred_at = Set(occurred_at.into());
    }
    if let Some(merchant) = req.merchant {
        active.merchant = Set(Some(merchant));
    }
    active.updated_at = Set(Utc::now().into());

    let model = active.update(db).await?;
    Ok(TransactionResponse::from(model))
}

pub async fn delete_transaction(
    db: &DatabaseConnection,
    user_id: Uuid,
    txn_id: Uuid,
) -> Result<(), ServiceError> {
    let txn = load_owned_transaction(db, user_id, txn_id).await?;

    let refund_count = Transaction::find()
        .filter(transaction::Column::RefTransactionId.eq(txn_id))
        .count(db)
        .await?;

    if refund_count > 0 {
        return Err(ServiceError::Conflict(
            "Cannot delete transaction with existing refunds".to_string(),
        ));
    }

    let active: transaction::ActiveModel = txn.into();
    active.delete(db).await?;

    Ok(())
}
