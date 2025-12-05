use super::Notifier;
use anyhow::Result;
use std::sync::Arc;
use tracing::error;

pub struct MultiNotifier {
    notifiers: Vec<Arc<dyn Notifier>>,
}

impl MultiNotifier {
    pub fn new(notifiers: Vec<Arc<dyn Notifier>>) -> Self {
        Self { notifiers }
    }
}

#[async_trait::async_trait]
impl Notifier for MultiNotifier {
    async fn send(&self, message: &str) -> Result<()> {
        let mut success_count = 0;
        let mut errors = Vec::new();

        for notifier in &self.notifiers {
            match notifier.send(message).await {
                Ok(_) => success_count += 1,
                Err(e) => {
                    error!(error = %e, "Notifier failed");
                    errors.push(e.to_string());
                }
            }
        }

        if success_count == 0 && !errors.is_empty() {
            anyhow::bail!("All notifiers failed: {}", errors.join(", "));
        }

        Ok(())
    }
}
