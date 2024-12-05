use axum::{routing::post, Router};
mod create;

pub(super) fn routes(config: crate::config::types::Config) -> Router {
    Router::new()
        .route("/fiscal_accounts", post(create::create))
        .with_state(config)
}
