use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub balance: String,
    pub currency_code: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateAccountRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub currency_code: String,
    pub initial_balance: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateAccountRequest {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub currency_code: Option<String>,
}
