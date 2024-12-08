use axum::{
    routing::{delete, get, patch, post},
    Router,
};

mod create_account;
mod delete_account;
mod get_account;
mod get_accounts;
mod patch_account;

pub(super) fn routes(config: crate::config::types::Config) -> Router {
    Router::new()
        .route("/", post(get_accounts::get_accounts))
        .route("/create", post(create_account::create_account))
        .route("/:id", get(get_account::get_account))
        .route("/:id", patch(patch_account::patch_account))
        .route("/:id", delete(delete_account::delete_account))
        .with_state(config)
}
