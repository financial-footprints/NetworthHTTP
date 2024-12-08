mod create_transaction;
mod delete_transaction;
mod get_transaction;
mod get_transactions;
mod patch_transaction;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub(super) fn routes(config: crate::config::types::Config) -> Router {
    Router::new()
        .route("/", post(get_transactions::get_transactions))
        .route("/create", post(create_transaction::create_transaction))
        .route(
            "/:transaction_id",
            patch(patch_transaction::patch_transaction),
        )
        .route("/:transaction_id", get(get_transaction::get_transaction))
        .route(
            "/:transaction_id",
            delete(delete_transaction::delete_transaction),
        )
        .with_state(config)
}
