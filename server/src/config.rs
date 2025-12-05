use std::env;
use tracing::warn;

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub struct NotificationConfig {
    pub feishu_webhook_url: Option<String>,
    pub smtp_config: Option<SmtpConfig>,
}

pub struct SmtpConfig {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
    pub recipients: Vec<String>,
}

impl NotificationConfig {
    pub fn from_env() -> Self {
        let feishu_webhook_url = env::var("FEISHU_WEBHOOK_URL").ok();

        let smtp_config = match (
            env::var("SMTP_SERVER").ok(),
            env::var("SMTP_PORT").ok(),
            env::var("SMTP_USERNAME").ok(),
            env::var("SMTP_PASSWORD").ok(),
            env::var("SMTP_FROM").ok(),
            env::var("SMTP_RECIPIENTS").ok(),
        ) {
            (Some(server), Some(port_str), Some(username), Some(password), Some(from), Some(recipients_str)) => {
                let port = match port_str.parse::<u16>() {
                    Ok(p) => p,
                    Err(_) => {
                        warn!(
                            port_str = %port_str,
                            "Invalid SMTP_PORT, using default 587"
                        );
                        587
                    }
                };

                let recipients: Vec<String> = recipients_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                if recipients.is_empty() {
                    warn!("SMTP_RECIPIENTS is empty, email notification disabled");
                    None
                } else {
                    Some(SmtpConfig {
                        server,
                        port,
                        username,
                        password,
                        from,
                        recipients,
                    })
                }
            }
            _ => None,
        };

        Self {
            feishu_webhook_url,
            smtp_config,
        }
    }
}

