use axum::{extract::State, http::StatusCode, Json};
use networth_db::models::{entities::accounts, helpers::accounts::AccountsQueryOptions};

pub async fn get_accounts(
    State(config): State<crate::config::types::Config>,
    Json(mut body): Json<AccountsQueryOptions>,
) -> Result<Json<Vec<accounts::Model>>, (StatusCode, String)> {
    if body.limit.is_none() {
        body.limit = Some(config.default_page_limit);
    }
    let accounts = networth_db::models::manage::accounts::get_accounts(&config.db, body)
        .await
        .map_err(|error| {
            let now = chrono::Utc::now();
            tracing::error!(
                "error.fiscal_accounts.get_accounts.could_not_retrieve at {}, error: {}",
                now,
                error
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.fiscal_accounts.get_accounts.could_not_retrieve".to_string(),
            )
        })?;

    Ok(Json(accounts))
}
