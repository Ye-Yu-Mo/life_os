use super::Notifier;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct FeishuNotifier {
    client: reqwest::Client,
    webhook_url: String,
}

impl FeishuNotifier {
    pub fn new(webhook_url: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to build reqwest client")?;

        Ok(Self {
            client,
            webhook_url,
        })
    }
}

#[derive(Serialize)]
struct FeishuMessage {
    msg_type: String,
    content: MessageContent,
}

#[derive(Serialize)]
struct MessageContent {
    text: String,
}

#[derive(Deserialize)]
struct FeishuResponse {
    code: i32,
}

#[async_trait::async_trait]
impl Notifier for FeishuNotifier {
    async fn send(&self, message: &str) -> Result<()> {
        let payload = FeishuMessage {
            msg_type: "text".to_string(),
            content: MessageContent {
                text: message.to_string(),
            },
        };

        let response = self
            .client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .await
            .context("Failed to send feishu webhook")?;

        if !response.status().is_success() {
            anyhow::bail!("Feishu webhook returned status: {}", response.status());
        }

        let body: FeishuResponse = response
            .json()
            .await
            .context("Failed to parse feishu response")?;

        if body.code != 0 {
            anyhow::bail!("Feishu webhook returned code: {}", body.code);
        }

        Ok(())
    }
}
