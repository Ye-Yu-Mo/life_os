use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::entities::{account, holdings, prelude::*, transaction};
use crate::errors::ServiceError;

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub name: String,
    pub r#type: String,
    pub currency_code: String,
    pub initial_balance: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountRequest {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub currency_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub name: String,
    pub r#type: String,
    pub balance: Decimal,
    pub currency_code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<account::Model> for AccountResponse {
    fn from(model: account::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            r#type: model.r#type,
            balance: model.balance,
            currency_code: model.currency_code,
            created_at: model.created_at.with_timezone(&Utc),
            updated_at: model.updated_at.with_timezone(&Utc),
        }
    }
}

fn validate_account_type(t: &str) -> Result<(), ServiceError> {
    if t.trim().is_empty() {
        return Err(ServiceError::Validation(
            "Account type cannot be empty".to_string()
        ));
    }
    Ok(())
}

fn validate_currency_code(code: &str) -> Result<(), ServiceError> {
    if code.trim().is_empty() {
        return Err(ServiceError::Validation(
            "Currency code cannot be empty".to_string()
        ));
    }
    if code.len() > 10 {
        return Err(ServiceError::Validation(
            "Currency code too long (max 10 chars)".to_string()
        ));
    }
    if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(ServiceError::Validation(
            "Currency code must be alphanumeric".to_string()
        ));
    }
    Ok(())
}

async fn load_owned_account(
    db: &DatabaseConnection,
    user_id: Uuid,
    account_id: Uuid,
) -> Result<account::Model, ServiceError> {
    let account = Account::find_by_id(account_id)
        .one(db)
        .await?
        .ok_or(ServiceError::NotFound)?;

    if account.user_id != user_id {
        return Err(ServiceError::Forbidden);
    }

    Ok(account)
}

pub async fn create_account(
    db: &DatabaseConnection,
    user_id: Uuid,
    req: CreateAccountRequest,
) -> Result<AccountResponse, ServiceError> {
    let name = req.name.trim();
    if name.is_empty() {
        return Err(ServiceError::Validation("Account name cannot be empty".to_string()));
    }

    let account_type = req.r#type.trim().to_lowercase();
    validate_account_type(&account_type)?;

    let currency = req.currency_code.trim().to_uppercase();
    validate_currency_code(&currency)?;

    let initial_balance = req.initial_balance.unwrap_or(Decimal::ZERO);

    let now = Utc::now().into();
    let account = account::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        name: Set(name.to_string()),
        r#type: Set(account_type),
        balance: Set(initial_balance),
        currency_code: Set(currency),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let model = account.insert(db).await?;
    Ok(AccountResponse::from(model))
}

pub async fn get_account(
    db: &DatabaseConnection,
    user_id: Uuid,
    account_id: Uuid,
) -> Result<AccountResponse, ServiceError> {
    let account = load_owned_account(db, user_id, account_id).await?;
    Ok(AccountResponse::from(account))
}

pub async fn list_accounts(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Vec<AccountResponse>, ServiceError> {
    let accounts = Account::find()
        .filter(account::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    Ok(accounts.into_iter().map(AccountResponse::from).collect())
}

pub async fn update_account(
    db: &DatabaseConnection,
    user_id: Uuid,
    account_id: Uuid,
    req: UpdateAccountRequest,
) -> Result<AccountResponse, ServiceError> {
    let account = load_owned_account(db, user_id, account_id).await?;

    if let Some(ref t) = req.r#type {
        let normalized = t.trim().to_lowercase();
        validate_account_type(&normalized)?;
    }
    if let Some(ref c) = req.currency_code {
        let normalized = c.trim().to_uppercase();
        validate_currency_code(&normalized)?;
    }
    if let Some(ref n) = req.name {
        if n.trim().is_empty() {
            return Err(ServiceError::Validation("Account name cannot be empty".to_string()));
        }
    }

    let mut active: account::ActiveModel = account.into();

    if let Some(name) = req.name {
        active.name = Set(name.trim().to_string());
    }
    if let Some(t) = req.r#type {
        active.r#type = Set(t.trim().to_lowercase());
    }
    if let Some(c) = req.currency_code {
        active.currency_code = Set(c.trim().to_uppercase());
    }
    active.updated_at = Set(Utc::now().into());

    let model = active.update(db).await?;
    Ok(AccountResponse::from(model))
}

pub async fn delete_account(
    db: &DatabaseConnection,
    user_id: Uuid,
    account_id: Uuid,
) -> Result<(), ServiceError> {
    let account = load_owned_account(db, user_id, account_id).await?;

    let txn_count = Transaction::find()
        .filter(
            Condition::any()
                .add(transaction::Column::FromAccountId.eq(account_id))
                .add(transaction::Column::ToAccountId.eq(account_id))
        )
        .count(db)
        .await?;

    if txn_count > 0 {
        return Err(ServiceError::Conflict(
            "Cannot delete account with existing transactions".to_string()
        ));
    }

    let holdings_count = Holdings::find()
        .filter(holdings::Column::AccountId.eq(account_id))
        .count(db)
        .await?;

    if holdings_count > 0 {
        return Err(ServiceError::Conflict(
            "Cannot delete account with existing holdings".to_string()
        ));
    }

    let active: account::ActiveModel = account.into();
    active.delete(db).await?;

    Ok(())
}