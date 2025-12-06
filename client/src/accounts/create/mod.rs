use super::model::CreateAccountRequest;
use super::service::AccountService;
use gpui::prelude::*;
use gpui::*;
use gpui_component::{button::*, form::*, input::*, *};
use std::sync::Arc;

pub struct CreateAccountView {
    service: Arc<AccountService>,
    name_input: Entity<InputState>,
    type_input: Entity<InputState>,
    balance_input: Entity<InputState>,
    currency_input: Entity<InputState>,
    in_flight: bool,
    error: Option<String>,
    on_success: Option<Box<dyn Fn(&mut Window, &mut Context<Self>) + Send + Sync>>,
}

impl CreateAccountView {
    pub fn new(
        service: Arc<AccountService>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name_input = cx.new(|cx| InputState::new(window, cx).placeholder("Account Name"));
        let type_input = cx.new(|cx| InputState::new(window, cx).placeholder("Type (e.g., bank_card)"));
        let balance_input = cx.new(|cx| InputState::new(window, cx).placeholder("Initial Balance (e.g., 0.00)"));
        let currency_input = cx.new(|cx| InputState::new(window, cx).placeholder("Currency (e.g., USD)"));

        Self {
            service,
            name_input,
            type_input,
            balance_input,
            currency_input,
            in_flight: false,
            error: None,
            on_success: None,
        }
    }

    pub fn on_success(
        mut self,
        callback: impl Fn(&mut Window, &mut Context<Self>) + Send + Sync + 'static,
    ) -> Self {
        self.on_success = Some(Box::new(callback));
        self
    }

    fn submit(&mut self, cx: &mut Context<Self>) {
        let name = self.name_input.read(cx).value();
        let r#type = self.type_input.read(cx).value();
        let balance_str = self.balance_input.read(cx).value();
        let currency = self.currency_input.read(cx).value();

        if name.is_empty() || r#type.is_empty() || currency.is_empty() {
            self.error = Some("Name, Type and Currency are required".to_string());
            cx.notify();
            return;
        }

        self.in_flight = true;
        self.error = None;
        cx.notify();

        let req = CreateAccountRequest {
            name: name.to_string(),
            r#type: r#type.to_string(),
            currency_code: currency.to_string(),
            initial_balance: if balance_str.is_empty() { None } else { Some(balance_str.to_string()) },
        };

        let service = self.service.clone();

        cx.spawn(async move |this, mut cx| {
            let result = service.create_account(req).await;

            this.update(cx, |this, cx| {
                this.in_flight = false;
                match result {
                    Ok(_) => {
                        cx.emit(super::view::CreateAccountSuccess);
                    }
                    Err(e) => {
                        this.error = Some(e.to_string());
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}

impl Render for CreateAccountView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(400.))
            .v_flex()
            .gap_4()
            .child(
                div()
                    .text_lg()
                    .font_bold()
                    .child("Create New Account")
            )
            .child(
                v_form()
                    .gap(px(16.))
                    .child(
                        field()
                            .label("Name")
                            .child(Input::new(&self.name_input)),
                    )
                    .child(
                        field()
                            .label("Type")
                            .child(Input::new(&self.type_input)),
                    )
                    .child(
                        field()
                            .label("Initial Balance")
                            .child(Input::new(&self.balance_input)),
                    )
                    .child(
                        field()
                            .label("Currency")
                            .child(Input::new(&self.currency_input)),
                    )
                    .when_some(self.error.as_ref(), |this, error| {
                        this.child(
                            field().label_indent(false).child(
                                div()
                                    .text_color(gpui::red())
                                    .text_sm()
                                    .child(error.clone()),
                            )
                        )
                    })
                    .child(
                        field()
                            .label_indent(false)
                            .child(
                                Button::new("create")
                                    .primary()
                                    .w_full()
                                    .label("Create")
                                    .disabled(self.in_flight)
                                    .on_click(cx.listener(|this, _, _, cx| this.submit(cx))),
                            ),
                    ),
            )
    }
}
