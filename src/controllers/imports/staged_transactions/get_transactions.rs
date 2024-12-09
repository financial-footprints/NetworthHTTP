use crate::config::types::Config;
use axum::{extract::State, http::StatusCode, Json};
use networth_db::models::{
    entities::staged_transactions, helpers::staged_transactions::StagedTransactionsQueryOptions,
};
pub async fn get_transactions(
    State(config): State<Config>,
    Json(mut body): Json<StagedTransactionsQueryOptions>,
) -> Result<Json<Vec<staged_transactions::Model>>, (StatusCode, String)> {
    if body.limit.is_none() {
        body.limit = Some(config.default_page_limit);
    }

    match networth_db::models::manage::staged_transactions::get_staged_transactions(
        &config.db, body,
    )
    .await
    {
        Ok(transactions) => Ok(Json(transactions)),
        Err(error) => {
            let now = chrono::Utc::now();
            tracing::error!(
                "error.staged_transactions.could_not_fetch at {}, error: {}",
                now,
                error
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.staged_transactions.could_not_fetch".to_string(),
            ))
        }
    }
}
