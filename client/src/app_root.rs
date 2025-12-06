use gpui::*;
use std::sync::Arc;
use gpui_http_client::HttpClient;
use crate::state::AppState;
use crate::auth::AuthView;
use crate::workspace::Workspace;

pub struct AppRoot {
    http_client: Arc<dyn HttpClient>,
    auth_view: Entity<AuthView>,
    active_view: AnyView,
}

impl AppRoot {
    pub fn new(http_client: Arc<dyn HttpClient>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let auth_view = cx.new(|cx| AuthView::new(http_client.clone(), window, cx));
        
        let active_view = auth_view.clone().into();

        // Subscribe to global state changes to switch views
        cx.observe_global::<AppState>(|this: &mut Self, cx| {
             this.update_view(cx);
        }).detach();

        Self {
            http_client,
            auth_view,
            active_view,
        }
    }

    fn update_view(&mut self, cx: &mut Context<Self>) {
        let app_state = cx.global::<AppState>();
        if app_state.is_authenticated() {
             // Switch to Workspace
             let workspace = cx.new(|cx| Workspace::new(self.http_client.clone(), cx));
             self.active_view = workspace.into();
        } else {
             // Switch to Auth
             // We reuse the existing auth_view which has InputStates created with Window.
             // If we needed to recreate it, we would fail because we lack window here.
             // Luckily we stored it!
             self.active_view = self.auth_view.clone().into();
        }
        cx.notify();
    }
}

impl Render for AppRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.active_view.clone())
    }
}
