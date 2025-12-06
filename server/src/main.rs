use server::{config, db, routes, services, state};
use services::notify::{EmailNotifier, FeishuNotifier, MultiNotifier, NoopNotifier, Notifier};
use state::AppState;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init_tracing();

    let database_url = config::get_database_url();
    let db = db::establish_connection(&database_url).await?;

    let notifier = build_notifier();

    let state = AppState { db, notifier };
    let app = routes::create_router(state);

    let addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| {
        error!(address = %addr, error = %e, "failed to bind listener");
        e
    })?;

    axum::serve(listener, app).await.map_err(|e| {
        error!(error = %e, "server stopped unexpectedly");
        e
    })?;

    Ok(())
}

fn build_notifier() -> Arc<dyn Notifier> {
    let config = config::NotificationConfig::from_env();
    let mut notifiers: Vec<Arc<dyn Notifier>> = Vec::new();

    if let Some(webhook_url) = config.feishu_webhook_url {
        match FeishuNotifier::new(webhook_url) {
            Ok(notifier) => {
                info!("Feishu notification enabled");
                notifiers.push(Arc::new(notifier));
            }
            Err(e) => {
                error!(error = %e, "Failed to create feishu notifier");
            }
        }
    }

    if let Some(smtp_config) = config.smtp_config {
        let server = smtp_config.server.clone();
        let port = smtp_config.port;

        match EmailNotifier::new(
            smtp_config.server,
            smtp_config.port,
            smtp_config.username,
            smtp_config.password,
            smtp_config.from,
            smtp_config.recipients,
        ) {
            Ok(notifier) => {
                info!(
                    server = %server,
                    port = port,
                    "Email notification enabled"
                );
                notifiers.push(Arc::new(notifier));
            }
            Err(e) => {
                error!(error = %e, "Failed to create email notifier");
            }
        }
    }

    if notifiers.is_empty() {
        info!("No notification channels configured, using noop");
        Arc::new(NoopNotifier)
    } else if notifiers.len() == 1 {
        notifiers.into_iter().next().unwrap()
    } else {
        info!("Multiple notification channels enabled");
        Arc::new(MultiNotifier::new(notifiers))
    }
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("error"));

    if std::env::var("APP_ENV").ok().as_deref() == Some("production") {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt::layer())
            .init();
    }
}
