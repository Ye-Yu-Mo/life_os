use sea_orm::DatabaseConnection;
use std::sync::Arc;
use crate::services::notify::Notifier;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub notifier: Arc<dyn Notifier>,
}
