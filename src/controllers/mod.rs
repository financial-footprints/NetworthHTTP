mod fiscal_accounts;
mod imports;
mod transactions;
use axum::Router;

pub(super) fn routes(config: crate::config::types::Config) -> Router {
    Router::new().nest(
        "/api/v1",
        Router::new()
            .nest("/fiscal_accounts", fiscal_accounts::routes(config.clone()))
            .nest("/import", imports::routes(config.clone()))
            .nest("/transactions", transactions::routes(config.clone())),
    )
}
