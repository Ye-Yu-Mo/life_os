use gpui::*;
use gpui_component::gray;
use std::sync::Arc;
use gpui_http_client::HttpClient;
use crate::state::AppState;
use crate::accounts::{service::AccountService, view::AccountsView};

pub struct Workspace {
    accounts_view: Entity<AccountsView>,
}

impl Workspace {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut Context<Self>) -> Self {
        let app_state = cx.global::<AppState>();
        let token = app_state.token.clone().expect("Workspace created without token");
        
        let account_service = Arc::new(AccountService::new(
            http_client, 
            "http://127.0.0.1:3000".to_string(), 
            token
        ));

        let accounts_view = cx.new(|cx| AccountsView::new(account_service, cx));

        Self {
            accounts_view,
        }
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(
                div() // Sidebar
                    .w_64()
                    .h_full()
                    .bg(gray(100).opacity(0.1))
                    .p_4()
                    .child("Sidebar")
            )
            .child(
                div() // Main Content
                    .flex_1()
                    .h_full()
                    .p_4()
                    .child(self.accounts_view.clone())
            )
    }
}
