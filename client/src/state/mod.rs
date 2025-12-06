use crate::auth::model::User;
use gpui::Global;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub user: Option<User>,
    pub token: Option<String>,
}

impl Global for AppState {}

impl AppState {
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some() && self.token.is_some()
    }

    pub fn set_auth(&mut self, user: User, token: String) {
        self.user = Some(user);
        self.token = Some(token);
    }

    pub fn logout(&mut self) {
        self.user = None;
        self.token = None;
    }
}
