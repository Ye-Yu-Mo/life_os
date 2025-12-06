use super::model::{AuthMode, AuthPayload};
use super::service::AuthService;
use crate::state::AppState;
use gpui::prelude::*;
use gpui::*;
use gpui_component::{button::*, form::*, input::*, *};
use gpui_http_client::HttpClient;
use std::sync::Arc;

pub struct AuthView {
    mode: AuthMode,
    username_input: Entity<InputState>,
    password_input: Entity<InputState>,
    service: Arc<AuthService>,
    in_flight: bool,
    error: Option<String>,
}

impl AuthView {
    pub fn new(http_client: Arc<dyn HttpClient>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let username_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("用户名")
                .clean_on_escape()
        });

        let password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("密码")
                .masked(true)
                .clean_on_escape()
        });

        let service = Arc::new(AuthService::new(
            http_client,
            "http://127.0.0.1:3000".to_string(),
        ));

        Self {
            mode: AuthMode::Login,
            username_input,
            password_input,
            service,
            in_flight: false,
            error: None,
        }
    }

    fn toggle_mode(&mut self, cx: &mut Context<Self>) {
        self.mode = self.mode.toggle();
        self.error = None;
        cx.notify();
    }

    fn submit(&mut self, cx: &mut Context<Self>) {
        let username = self.username_input.read(cx).value();
        let password = self.password_input.read(cx).value();

        if username.is_empty() || password.is_empty() {
            self.error = Some("用户名和密码不能为空".to_string());
            cx.notify();
            return;
        }

        self.in_flight = true;
        self.error = None;
        cx.notify();

        let service = self.service.clone();
        let endpoint = self.mode.endpoint().to_string();
        let payload = AuthPayload {
            username: username.to_string(),
            password: password.to_string(),
        };

        cx.spawn(async move |this, cx| {
            let result = service.authenticate(&endpoint, payload).await;

            cx.update(|cx| {
                this.update(cx, |this, cx| {
                    this.in_flight = false;
                    match result {
                        Ok(user) => {
                            if let Some(token) = user.token.clone() {
                                cx.update_global::<AppState, _>(|state, _| {
                                    state.set_auth(user, token);
                                });
                                this.error = None;
                            } else {
                                this.error = Some("登录成功但未返回 Token".to_string());
                            }
                        }
                        Err(err) => {
                            this.error = Some(err.to_string());
                        }
                    }
                    cx.notify();
                })
            })
            .ok();
        })
        .detach();
    }
}

impl Render for AuthView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = cx.global::<AppState>();
        
        if let Some(user) = &app_state.user {
            return div()
                .v_flex()
                .gap_4()
                .size_full()
                .items_center()
                .justify_center()
                .child(format!("欢迎, {}!", user.username))
                .child(format!("用户 ID: {}", user.id))
                .child(
                    Button::new("logout")
                        .label("退出登录")
                        .on_click(cx.listener(|_, _, _, cx| {
                            cx.update_global::<AppState, _>(|state, _| {
                                state.logout();
                            });
                        })),
                );
        }

        div()
            .v_flex()
            .gap_6()
            .size_full()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(400.))
                    .v_flex()
                    .gap_4()
                    .child(
                        div()
                            .text_2xl()
                            .font_bold()
                            .text_center()
                            .child(match self.mode {
                                AuthMode::Login => "登录",
                                AuthMode::Register => "注册",
                            }),
                    )
                    .child(
                        v_form()
                            .gap(px(16.))
                            .child(
                                field()
                                    .label("用户名")
                                    .required(true)
                                    .child(Input::new(&self.username_input).cleanable(true)),
                            )
                            .child(
                                field()
                                    .label("密码")
                                    .required(true)
                                    .child(Input::new(&self.password_input).mask_toggle()),
                            )
                            .when_some(self.error.as_ref(), |this, error| {
                                this.child(
                                    field()
                                        .label_indent(false)
                                        .child(
                                            div()
                                                .text_color(gpui::red())
                                                .text_sm()
                                                .child(error.clone()),
                                        ),
                                )
                            })
                            .child(
                                field()
                                    .label_indent(false)
                                    .child(
                                        Button::new("submit")
                                            .primary()
                                            .w_full()
                                            .label(self.mode.button_text())
                                            .disabled(self.in_flight)
                                            .loading(self.in_flight)
                                            .on_click(cx.listener(|this, _, _, cx| this.submit(cx))),
                                    ),
                            )
                            .child(
                                field()
                                    .label_indent(false)
                                    .child(
                                        div()
                                            .text_center()
                                            .text_sm()
                                            .child(
                                                Button::new("toggle")
                                                    .link()
                                                    .label(self.mode.toggle_text())
                                                    .on_click(cx.listener(|this, _, _, cx| this.toggle_mode(cx))),
                                            ),
                                    ),
                            ),
                    ),
            )
    }
}
