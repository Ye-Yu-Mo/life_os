use super::Notifier;
use anyhow::Result;

pub struct NoopNotifier;

#[async_trait::async_trait]
impl Notifier for NoopNotifier {
    async fn send(&self, _message: &str) -> Result<()> {
        Ok(())
    }
}
