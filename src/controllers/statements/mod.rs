mod import;

use axum::{routing::post, Router};

pub(super) fn routes(config: crate::config::types::Config) -> Router {
    Router::new()
        .route("/import", post(import::statement_import))
        .with_state(config)
}
