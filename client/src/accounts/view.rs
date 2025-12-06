use super::create::CreateAccountView;
use super::model::Account;
use super::service::AccountService;
use crate::components::confirm_dialog::{CancelEvent, ConfirmDialog, ConfirmEvent};
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
    show_confirm_dialog: bool,
    account_to_delete: Option<String>,
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
            show_confirm_dialog: false,
            account_to_delete: None,
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

    fn delete_account_prompt(&mut self, account_id: String, cx: &mut Context<Self>) {
        self.show_confirm_dialog = true;
        self.account_to_delete = Some(account_id);
        cx.notify();
    }

    fn confirm_delete_account(&mut self, cx: &mut Context<Self>) {
        if let Some(id) = self.account_to_delete.take() {
            self.show_confirm_dialog = false;
            self.loading = true;
            cx.notify();
            
            let service = self.service.clone();
            cx.spawn(async move |this, mut cx| {
                let result = service.delete_account(&id).await;
                
                this.update(cx, |this, cx| {
                    this.loading = false;
                    match result {
                        Ok(_) => {
                            this.fetch_accounts(cx);
                        }
                        Err(e) => {
                            this.error = Some(e.to_string());
                            cx.notify();
                        }
                    }
                }).ok();
            }).detach();
        }
    }

    fn cancel_delete_account(&mut self, cx: &mut Context<Self>) {
        self.show_confirm_dialog = false;
        self.account_to_delete = None;
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

        let main_content = div()
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
                                let account_id = account.id.clone();
                                div()
                                    .p_4()
                                    .border_1()
                                    .border_color(gray(300))
                                    .rounded_md()
                                    .flex()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        div()
                                            .v_flex()
                                            .child(format!("{} ({}) - {}", account.name, account.currency_code, account.r#type))
                                            .child(div().text_sm().text_color(gray(500)).child(format!("Balance: {}", account.balance)))
                                    )
                                    .child(
                                        Button::new("delete")
                                            .label("Delete")
                                            .on_click(cx.listener(move |this, _, _, cx| this.delete_account_prompt(account_id.clone(), cx)))
                                    )
                            })
                        )
                }
            );
        
        if self.show_confirm_dialog {
            let dialog = cx.new(|cx| {
                ConfirmDialog::new("Are you sure you want to delete this account?".to_string(), cx)
            });
            cx.subscribe(&dialog, |this, _, _: &ConfirmEvent, cx| {
                this.confirm_delete_account(cx);
            }).detach();
            cx.subscribe(&dialog, |this, _, _: &CancelEvent, cx| {
                this.cancel_delete_account(cx);
            }).detach();

            div().size_full().child(main_content).child(dialog)
        } else {
            main_content
        }
    }
}
