use gpui::{prelude::FluentBuilder, *};
use gpui_component::{button::{Button, ButtonVariants}, gray, StyledExt};
use std::sync::Arc;
use gpui_http_client::HttpClient;
use crate::state::AppState;
use crate::accounts::{service::AccountService, view::AccountsView};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Accounts,
    Transactions,
    Holdings,
}

impl Tab {
    fn label(&self) -> &'static str {
        match self {
            Tab::Accounts => "Accounts",
            Tab::Transactions => "Transactions",
            Tab::Holdings => "Holdings",
        }
    }
}

pub struct Workspace {
    http_client: Arc<dyn HttpClient>,
    current_tab: Tab,
    accounts_view: Entity<AccountsView>,
    // placeholders for other views
}

impl Workspace {
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut Context<Self>) -> Self {
        let app_state = cx.global::<AppState>();
        let token = app_state.token.clone().expect("Workspace created without token");
        
        let account_service = Arc::new(AccountService::new(
            http_client.clone(), 
            "http://127.0.0.1:3000".to_string(), 
            token
        ));

        let accounts_view = cx.new(|cx| AccountsView::new(account_service, cx));

        Self {
            http_client,
            current_tab: Tab::Accounts,
            accounts_view,
        }
    }

    fn switch_tab(&mut self, tab: Tab, cx: &mut Context<Self>) {
        self.current_tab = tab;
        cx.notify();
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(
                div() // Sidebar
                    .w_64()
                    .h_full()
                    .bg(gray(100).opacity(0.1))
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(div().text_lg().font_bold().child("Life OS").pb_4())
                    .child(self.render_sidebar_item(Tab::Accounts, cx))
                    .child(self.render_sidebar_item(Tab::Transactions, cx))
                    .child(self.render_sidebar_item(Tab::Holdings, cx))
            )
            .child(
                div() // Main Content
                    .flex_1()
                    .h_full()
                    .p_4()
                    .child(match self.current_tab {
                        Tab::Accounts => self.accounts_view.clone().into_any_element(),
                        Tab::Transactions => div().child("Transactions (Coming Soon)").into_any_element(),
                        Tab::Holdings => div().child("Holdings (Coming Soon)").into_any_element(),
                    })
            )
    }
}

impl Workspace {
    fn render_sidebar_item(&self, tab: Tab, cx: &mut Context<Self>) -> impl IntoElement {
        let is_active = self.current_tab == tab;
        Button::new(tab.label())
            .label(tab.label())
            .w_full()
            .ghost()
            .when(is_active, |b| b.bg(gray(700)).text_color(white()))
            .on_click(cx.listener(move |this, _, _, cx| this.switch_tab(tab, cx)))
    }
}
