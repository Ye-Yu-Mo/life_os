use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers::account::{
    create_account_handler, delete_account_handler, get_account_handler, list_accounts_handler,
    update_account_handler,
};
use crate::handlers::auth::{login_handler, register_handler};
use crate::handlers::holdings::{
    create_holdings_handler, delete_holdings_handler, get_holdings_handler,
    list_holdings_handler, update_holdings_handler,
};
use crate::handlers::test::test_notification_handler;
use crate::handlers::transaction::{
    create_transaction_handler, delete_transaction_handler, get_transaction_handler,
    list_transactions_handler, update_transaction_handler,
};
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/test/notification", post(test_notification_handler))
        .route("/accounts", post(create_account_handler))
        .route("/accounts", get(list_accounts_handler))
        .route("/accounts/:account_id", get(get_account_handler))
        .route("/accounts/:account_id", put(update_account_handler))
        .route("/accounts/:account_id", delete(delete_account_handler))
        .route("/transactions", post(create_transaction_handler))
        .route("/transactions", get(list_transactions_handler))
        .route("/transactions/:txn_id", get(get_transaction_handler))
        .route("/transactions/:txn_id", put(update_transaction_handler))
        .route("/transactions/:txn_id", delete(delete_transaction_handler))
        .route("/holdings", post(create_holdings_handler))
        .route("/holdings", get(list_holdings_handler))
        .route("/holdings/:holdings_id", get(get_holdings_handler))
        .route("/holdings/:holdings_id", put(update_holdings_handler))
        .route("/holdings/:holdings_id", delete(delete_holdings_handler))
        .with_state(state)
}
