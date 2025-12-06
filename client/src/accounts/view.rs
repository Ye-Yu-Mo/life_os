use super::create::CreateAccountView;
use super::model::Account;
use super::service::AccountService;
use gpui::prelude::*;
use gpui::*;
use gpui_component::{button::Button, gray, StyledExt};
use std::sync::Arc;

pub struct AccountsView {
    service: Arc<AccountService>,
    accounts: Vec<Account>,
    loading: bool,
    error: Option<String>,
    create_view: Option<Entity<CreateAccountView>>,
    show_create: bool,
}

impl AccountsView {
    pub fn new(service: Arc<AccountService>, cx: &mut Context<Self>) -> Self {
        let view = Self {
            service,
            accounts: Vec::new(),
            loading: true,
            error: None,
            create_view: None,
            show_create: false,
        };
        cx.spawn(async move |this, mut cx| {
             this.update(cx, |this, cx| this.fetch_accounts(cx)).ok();
        }).detach();
        view
    }

    fn fetch_accounts(&mut self, cx: &mut Context<Self>) {
        self.loading = true;
        self.error = None;
        cx.notify();

        let service = self.service.clone();
        cx.spawn(async move |this, mut cx| {
            let result = service.list_accounts().await;
            
            this.update(cx, |this, cx| {
                this.loading = false;
                match result {
                    Ok(accounts) => this.accounts = accounts,
                    Err(e) => this.error = Some(e.to_string()),
                }
                cx.notify();
            }).ok();
        }).detach();
    }

    fn toggle_create(&mut self, cx: &mut Context<Self>) {
        self.show_create = !self.show_create;
        if !self.show_create {
            self.create_view = None;
        }
        cx.notify();
    }
}

pub struct CreateAccountSuccess;

impl EventEmitter<CreateAccountSuccess> for CreateAccountView {}

impl Render for AccountsView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Lazy creation of CreateAccountView because it needs &mut Window
        if self.show_create && self.create_view.is_none() {
             let service = self.service.clone();
             let view = cx.new(|cx| {
                CreateAccountView::new(service, window, cx)
            });
            cx.subscribe(&view, |this, _, _: &CreateAccountSuccess, cx| {
                 this.show_create = false;
                 this.create_view = None;
                 this.fetch_accounts(cx);
            }).detach();
            self.create_view = Some(view);
        }

        div()
            .v_flex()
            .gap_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_xl()
                            .font_bold()
                            .child("Accounts")
                    )
                    .child(
                        Button::new("create_account")
                            .label(if self.show_create { "Cancel" } else { "Create Account" })
                            .on_click(cx.listener(move |this, _, _, cx| this.toggle_create(cx)))
                    )
            )
            .child(
                 if let Some(create_view) = &self.create_view {
                     div().child(create_view.clone())
                 } else if self.loading {
                    div().child("Loading accounts...")
                } else if let Some(error) = &self.error {
                    div().text_color(red()).child(format!("Error: {}", error))
                } else {
                    div()
                        .v_flex()
                        .gap_2()
                        .children(
                            self.accounts.iter().map(|account| {
                                div()
                                    .p_4()
                                    .border_1()
                                    .border_color(gray(300))
                                    .rounded_md()
                                    .child(format!("{} ({}) - {}", account.name, account.currency_code, account.r#type))
                            })
                        )
                }
            )
    }
}
