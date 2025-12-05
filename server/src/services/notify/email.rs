use super::Notifier;
use anyhow::{Context, Result};
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use std::time::Duration;

pub struct EmailNotifier {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
    recipients: Vec<Mailbox>,
}

impl EmailNotifier {
    pub fn new(
        smtp_server: String,
        smtp_port: u16,
        smtp_username: String,
        smtp_password: String,
        from: String,
        recipients: Vec<String>,
    ) -> Result<Self> {
        let creds = Credentials::new(smtp_username, smtp_password);

        let transport = if smtp_port == 465 {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_server)
                .context("Failed to create SMTP transport")?
                .credentials(creds)
                .port(smtp_port)
                .timeout(Some(Duration::from_secs(30)))
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_server)
                .context("Failed to create SMTP transport")?
                .credentials(creds)
                .port(smtp_port)
                .timeout(Some(Duration::from_secs(30)))
                .build()
        };

        let from_mailbox = from
            .parse::<Mailbox>()
            .context("Invalid from email address")?;

        let mut recipient_mailboxes = Vec::new();
        for recipient in recipients {
            let mailbox = recipient
                .parse::<Mailbox>()
                .context("Invalid recipient email address")?;
            recipient_mailboxes.push(mailbox);
        }

        if recipient_mailboxes.is_empty() {
            anyhow::bail!("At least one recipient is required");
        }

        Ok(Self {
            transport,
            from: from_mailbox,
            recipients: recipient_mailboxes,
        })
    }
}

#[async_trait::async_trait]
impl Notifier for EmailNotifier {
    async fn send(&self, message: &str) -> Result<()> {
        let mut builder = Message::builder()
            .from(self.from.clone())
            .subject("Notification");

        for recipient in &self.recipients {
            builder = builder.to(recipient.clone());
        }

        let email = builder
            .body(message.to_string())
            .context("Failed to build email")?;

        self.transport
            .send(email)
            .await
            .context("Failed to send email")?;

        Ok(())
    }
}
