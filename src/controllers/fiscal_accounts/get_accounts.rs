use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use networth_db::models::{entities::accounts, helpers::accounts::AccountsQueryOptions};
use std::collections::HashMap;

pub async fn get_accounts(
    State(config): State<crate::config::types::Config>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<accounts::Model>>, (StatusCode, String)> {
    let limit = params
        .get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(config.default_page_limit.clone());
    let offset = params
        .get("offset")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let accounts = networth_db::models::manage::accounts::get_accounts(
        &config.db,
        AccountsQueryOptions {
            limit: Some(limit),
            offset: Some(offset),
            ..Default::default()
        },
    )
    .await
    .map_err(|error| {
        let now = chrono::Utc::now();
        tracing::error!(
            "error.fiscal_accounts.get_accounts.could_not_retrieve at {}, error: {}",
            now,
            error
        );
        println!("error: {:?}", error);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "error.fiscal_accounts.get_accounts.could_not_retrieve".to_string(),
        )
    })?;

    Ok(Json(accounts))
}
