use gpui::prelude::*;
use gpui::*;
use gpui_component::{button::{Button, ButtonVariants}, *};

pub struct ConfirmDialog {
    message: String,
}

impl ConfirmDialog {
    pub fn new(message: String, _cx: &mut Context<Self>) -> Self {
        Self { message }
    }
}

pub struct ConfirmEvent;
pub struct CancelEvent;

impl EventEmitter<ConfirmEvent> for ConfirmDialog {}
impl EventEmitter<CancelEvent> for ConfirmDialog {}

impl Render for ConfirmDialog {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .absolute()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::black().opacity(0.5)) // Overlay background
            .child(
                div() // Dialog box
                    .w(px(300.))
                    .bg(gpui::white())
                    .rounded_lg()
                    .p_4()
                    .v_flex()
                    .gap_4()
                    .child(div().text_lg().font_bold().child("Confirm Action"))
                    .child(div().child(self.message.clone()))
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                Button::new("cancel")
                                    .label("Cancel")
                                    .on_click(cx.listener(move |_this, _window, _, cx| {
                                        cx.emit(CancelEvent);
                                    })),
                            )
                            .child(
                                Button::new("confirm")
                                    .primary()
                                    .label("Confirm")
                                    .on_click(cx.listener(move |_this, _window, _, cx| {
                                        cx.emit(ConfirmEvent);
                                    })),
                            ),
                    ),
            )
    }
}
