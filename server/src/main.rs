mod config;
mod db;
mod entities;
mod errors;
mod handlers;
mod routes;
mod services;
mod state;

use state::AppState;
use tracing::error;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init_tracing();

    let database_url = config::get_database_url();
    let db = db::establish_connection(&database_url).await?;

    let state = AppState { db };
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
