mod import;

use axum::{routing::post, Router};

pub(super) fn routes(config: crate::config::types::Config) -> Router {
    Router::new().nest(
        "/statements",
        Router::new()
            .route("/upload", post(import::import_from_file))
            .with_state(config),
    )
}
