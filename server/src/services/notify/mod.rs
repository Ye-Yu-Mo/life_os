mod feishu;
mod email;
mod noop;
mod multi;

pub use feishu::FeishuNotifier;
pub use email::EmailNotifier;
pub use noop::NoopNotifier;
pub use multi::MultiNotifier;

use anyhow::Result;

#[async_trait::async_trait]
pub trait Notifier: Send + Sync {
    async fn send(&self, message: &str) -> Result<()>;
}
