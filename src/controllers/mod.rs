mod fiscal_accounts;
mod imports;
mod transactions;

use crate::config::types::Config;
use axum::{
    http::{
        self,
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    Router,
};
use tower_http::cors::CorsLayer;

pub(super) fn routes(config: Config) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            config
                .cors_allowed_origins
                .parse::<http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION])
        .max_age(std::time::Duration::from_secs(3600));

    Router::new().nest(
        "/api/v1",
        Router::new()
            .nest("/fiscal_accounts", fiscal_accounts::routes(config.clone()))
            .nest("/import", imports::routes(config.clone()))
            .nest("/transactions", transactions::routes(config.clone()))
            .layer(cors),
    )
}
