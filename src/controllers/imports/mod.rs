mod approve_import;
mod create_import;
mod delete_import;
mod get_import;
mod get_imports;
mod patch_import;
mod staged_transactions;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

pub(super) fn routes(config: crate::config::types::Config) -> Router {
    Router::new()
        .route("/", get(get_imports::get_imports))
        .route("/", post(create_import::create_import))
        .route("/:id", get(get_import::get_import))
        .route("/:id", patch(patch_import::patch_import))
        .route("/:id", post(approve_import::approve_import))
        .route("/:id", delete(delete_import::delete_import))
        .with_state(config.clone())
        .nest(
            "/:id/transactions",
            staged_transactions::routes(config.clone()),
        )
}
