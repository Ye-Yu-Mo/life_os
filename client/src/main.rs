mod auth;
mod state;
mod surf_client;

use auth::AuthView;
use gpui::*;
use gpui_component::*;
use state::AppState;
use surf_client::SurfClient;
use tracing::error;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    init_tracing();

    let app = Application::new()
        .with_assets(gpui_component_assets::Assets)
        .with_http_client(SurfClient::new());

    app.run(move |cx| {
        gpui_component::init(cx);
        
        // Initialize Global AppState
        cx.set_global(AppState::default());

        let http_client = cx.http_client();

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), move |window, cx| {
                let view = cx.new(|cx| AuthView::new(http_client.clone(), window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .map_err(|err| {
                error!(error = ?err, "failed to open window");
                err
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
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
