use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{holdings, prelude::*};
use crate::errors::ServiceError;

#[derive(Debug, Deserialize)]
pub struct CreateHoldingsRequest {
    pub account_id: Uuid,
    pub asset_type: String,
    pub symbol: String,
    pub name: Option<String>,
    pub quantity: Decimal,
    pub cost_basis_total: Decimal,
    pub currency_code: String,
    pub last_price: Option<Decimal>,
    pub last_price_at: Option<DateTime<Utc>>,
    pub market_value: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHoldingsRequest {
    pub quantity: Option<Decimal>,
    pub cost_basis_total: Option<Decimal>,
    pub last_price: Option<Decimal>,
    pub last_price_at: Option<DateTime<Utc>>,
    pub market_value: Option<Decimal>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HoldingsResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub asset_type: String,
    pub symbol: String,
    pub name: Option<String>,
    pub quantity: Decimal,
    pub cost_basis_total: Decimal,
    pub currency_code: String,
    pub last_price: Option<Decimal>,
    pub last_price_at: Option<DateTime<Utc>>,
    pub market_value: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<holdings::Model> for HoldingsResponse {
    fn from(model: holdings::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            account_id: model.account_id,
            asset_type: model.asset_type,
            symbol: model.symbol,
            name: model.name,
            quantity: model.quantity,
            cost_basis_total: model.cost_basis_total,
            currency_code: model.currency_code,
            last_price: model.last_price,
            last_price_at: model.last_price_at.map(|dt| dt.with_timezone(&Utc)),
            market_value: model.market_value,
            created_at: model.created_at.with_timezone(&Utc),
            updated_at: model.updated_at.with_timezone(&Utc),
        }
    }
}

const VALID_ASSET_TYPES: &[&str] = &["stock", "fund", "crypto", "bond", "cash", "other"];

fn validate_asset_type(t: &str) -> Result<(), ServiceError> {
    if !VALID_ASSET_TYPES.contains(&t) {
        return Err(ServiceError::Validation(format!("Invalid asset type: {}", t)));
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

async fn load_owned_holdings(
    db: &DatabaseConnection,
    user_id: Uuid,
    holdings_id: Uuid,
) -> Result<holdings::Model, ServiceError> {
    let holding = Holdings::find_by_id(holdings_id)
        .one(db)
        .await?
        .ok_or(ServiceError::NotFound)?;

    if holding.user_id != user_id {
        return Err(ServiceError::Forbidden);
    }

    Ok(holding)
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

pub async fn create_holdings(
    db: &DatabaseConnection,
    user_id: Uuid,
    req: CreateHoldingsRequest,
) -> Result<HoldingsResponse, ServiceError> {
    verify_account_ownership(db, user_id, req.account_id).await?;

    if req.quantity < Decimal::ZERO {
        return Err(ServiceError::Validation("Quantity cannot be negative".to_string()));
    }

    if req.cost_basis_total < Decimal::ZERO {
        return Err(ServiceError::Validation(
            "Cost basis total cannot be negative".to_string(),
        ));
    }

    if let Some(price) = req.last_price {
        if price < Decimal::ZERO {
            return Err(ServiceError::Validation("Price cannot be negative".to_string()));
        }
    }

    if let Some(value) = req.market_value {
        if value < Decimal::ZERO {
            return Err(ServiceError::Validation("Market value cannot be negative".to_string()));
        }
    }

    let asset_type = req.asset_type.trim().to_lowercase();
    validate_asset_type(&asset_type)?;

    let currency = req.currency_code.trim().to_uppercase();
    validate_currency_code(&currency)?;

    let symbol = req.symbol.trim().to_uppercase();
    if symbol.is_empty() {
        return Err(ServiceError::Validation("Symbol cannot be empty".to_string()));
    }

    let now = Utc::now().into();
    let holding = holdings::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        account_id: Set(req.account_id),
        asset_type: Set(asset_type),
        symbol: Set(symbol),
        name: Set(req.name),
        quantity: Set(req.quantity),
        cost_basis_total: Set(req.cost_basis_total),
        currency_code: Set(currency),
        last_price: Set(req.last_price),
        last_price_at: Set(req.last_price_at.map(|dt| dt.into())),
        market_value: Set(req.market_value),
        created_at: Set(now),
        updated_at: Set(now),
    };

    match holding.insert(db).await {
        Ok(model) => Ok(HoldingsResponse::from(model)),
        Err(sea_orm::DbErr::Exec(_)) | Err(sea_orm::DbErr::Query(_)) => {
            Err(ServiceError::Conflict(
                "Holdings with same account, asset type and symbol already exists".to_string(),
            ))
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn get_holdings(
    db: &DatabaseConnection,
    user_id: Uuid,
    holdings_id: Uuid,
) -> Result<HoldingsResponse, ServiceError> {
    let holding = load_owned_holdings(db, user_id, holdings_id).await?;
    Ok(HoldingsResponse::from(holding))
}

pub async fn list_holdings(
    db: &DatabaseConnection,
    user_id: Uuid,
    account_id: Option<Uuid>,
    asset_type: Option<String>,
) -> Result<Vec<HoldingsResponse>, ServiceError> {
    let mut query = Holdings::find().filter(holdings::Column::UserId.eq(user_id));

    if let Some(acc_id) = account_id {
        query = query.filter(holdings::Column::AccountId.eq(acc_id));
    }
    if let Some(asset) = asset_type {
        let normalized = asset.trim().to_lowercase();
        validate_asset_type(&normalized)?;
        query = query.filter(holdings::Column::AssetType.eq(normalized));
    }

    let holdings_list = query.all(db).await?;

    Ok(holdings_list.into_iter().map(HoldingsResponse::from).collect())
}

pub async fn update_holdings(
    db: &DatabaseConnection,
    user_id: Uuid,
    holdings_id: Uuid,
    req: UpdateHoldingsRequest,
) -> Result<HoldingsResponse, ServiceError> {
    let holding = load_owned_holdings(db, user_id, holdings_id).await?;

    if let Some(qty) = req.quantity {
        if qty < Decimal::ZERO {
            return Err(ServiceError::Validation("Quantity cannot be negative".to_string()));
        }
    }

    if let Some(cost) = req.cost_basis_total {
        if cost < Decimal::ZERO {
            return Err(ServiceError::Validation(
                "Cost basis total cannot be negative".to_string(),
            ));
        }
    }

    if let Some(price) = req.last_price {
        if price < Decimal::ZERO {
            return Err(ServiceError::Validation("Price cannot be negative".to_string()));
        }
    }

    if let Some(value) = req.market_value {
        if value < Decimal::ZERO {
            return Err(ServiceError::Validation("Market value cannot be negative".to_string()));
        }
    }

    let mut active: holdings::ActiveModel = holding.into();

    if let Some(qty) = req.quantity {
        active.quantity = Set(qty);
    }
    if let Some(cost) = req.cost_basis_total {
        active.cost_basis_total = Set(cost);
    }
    if let Some(price) = req.last_price {
        active.last_price = Set(Some(price));
    }
    if let Some(price_at) = req.last_price_at {
        active.last_price_at = Set(Some(price_at.into()));
    }
    if let Some(value) = req.market_value {
        active.market_value = Set(Some(value));
    }
    if let Some(name) = req.name {
        active.name = Set(Some(name));
    }
    active.updated_at = Set(Utc::now().into());

    let model = active.update(db).await?;
    Ok(HoldingsResponse::from(model))
}

pub async fn delete_holdings(
    db: &DatabaseConnection,
    user_id: Uuid,
    holdings_id: Uuid,
) -> Result<(), ServiceError> {
    let holding = load_owned_holdings(db, user_id, holdings_id).await?;

    let active: holdings::ActiveModel = holding.into();
    active.delete(db).await?;

    Ok(())
}
