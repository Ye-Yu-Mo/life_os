use super::model::{Account, CreateAccountRequest, UpdateAccountRequest};
use crate::auth::model::ErrorResponse;
use gpui_http_client::{AsyncBody, HttpClient, Method, Request};
use std::sync::Arc;
use anyhow::Result;
use futures::AsyncReadExt;

pub struct AccountService {
    client: Arc<dyn HttpClient>,
    base_url: String,
    token: String,
}

impl AccountService {
    pub fn new(client: Arc<dyn HttpClient>, base_url: String, token: String) -> Self {
        Self { client, base_url, token }
    }

    pub async fn list_accounts(&self) -> Result<Vec<Account>> {
        let url = format!("{}/accounts", self.base_url);

        let request = Request::builder()
            .method(Method::GET)
            .uri(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .body(AsyncBody::empty())?;

        let mut response = self.client.send(request).await.map_err(|e| anyhow::anyhow!(e))?;
        
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await.map_err(|e| anyhow::anyhow!(e))?;

        if !response.status().is_success() {
             if let Ok(err_resp) = serde_json::from_slice::<ErrorResponse>(&body) {
                 anyhow::bail!("{}", err_resp.error);
             }
             anyhow::bail!("Request failed with status: {}", response.status());
        }

        let accounts: Vec<Account> = serde_json::from_slice(&body)?;

        Ok(accounts)
    }

    pub async fn create_account(&self, req: CreateAccountRequest) -> Result<Account> {
        let url = format!("{}/accounts", self.base_url);
        let body = serde_json::to_vec(&req)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(body))?;

        let mut response = self.client.send(request).await.map_err(|e| anyhow::anyhow!(e))?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await.map_err(|e| anyhow::anyhow!(e))?;

         if !response.status().is_success() {
             if let Ok(err_resp) = serde_json::from_slice::<ErrorResponse>(&body) {
                 anyhow::bail!("{}", err_resp.error);
             }
             anyhow::bail!("Request failed with status: {}", response.status());
        }

        let account: Account = serde_json::from_slice(&body)?;

        Ok(account)
    }

    pub async fn update_account(&self, id: &str, req: UpdateAccountRequest) -> Result<Account> {
        let url = format!("{}/accounts/{}", self.base_url, id);
        let body = serde_json::to_vec(&req)?;

        let request = Request::builder()
            .method(Method::PUT)
            .uri(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(body))?;

        let mut response = self.client.send(request).await.map_err(|e| anyhow::anyhow!(e))?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await.map_err(|e| anyhow::anyhow!(e))?;

         if !response.status().is_success() {
             if let Ok(err_resp) = serde_json::from_slice::<ErrorResponse>(&body) {
                 anyhow::bail!("{}", err_resp.error);
             }
             anyhow::bail!("Request failed with status: {}", response.status());
        }

        let account: Account = serde_json::from_slice(&body)?;

        Ok(account)
    }

    pub async fn delete_account(&self, id: &str) -> Result<()> {
        let url = format!("{}/accounts/{}", self.base_url, id);

        let request = Request::builder()
            .method(Method::DELETE)
            .uri(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .body(AsyncBody::empty())?;

        let mut response = self.client.send(request).await.map_err(|e| anyhow::anyhow!(e))?;

        if !response.status().is_success() {
             let mut body = Vec::new();
             response.body_mut().read_to_end(&mut body).await.map_err(|e| anyhow::anyhow!(e))?;
             if let Ok(err_resp) = serde_json::from_slice::<ErrorResponse>(&body) {
                 anyhow::bail!("{}", err_resp.error);
             }
             anyhow::bail!("Request failed with status: {}", response.status());
        }

        Ok(())
    }
}
