mod auth;
mod surf_client;

use auth::AuthView;
use gpui::*;
use gpui_component::*;
use surf_client::SurfClient;

fn main() {
    let app = Application::new()
        .with_assets(gpui_component_assets::Assets)
        .with_http_client(SurfClient::new());

    app.run(move |cx| {
        gpui_component::init(cx);

        let http_client = cx.http_client();

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), move |window, cx| {
                let view = cx.new(|cx| AuthView::new(http_client.clone(), window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
