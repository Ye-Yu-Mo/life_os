use axum::{routing::post, Router};

use crate::handlers::auth::{login_handler, register_handler};
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(state)
}
