mod statements;
use axum::Router;

pub(super) fn routes(config: crate::config::types::Config) -> Router {
    Router::new().nest("/api/v1", statements::routes(config))
}
