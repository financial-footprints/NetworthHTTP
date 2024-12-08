use crate::config::types::Config;
use axum::{extract::State, http::StatusCode, Json};
use networth_db::models::{
    entities::transactions, helpers::transactions::TransactionsQueryOptions,
};

pub async fn get_transactions(
    State(config): State<Config>,
    Json(mut body): Json<TransactionsQueryOptions>,
) -> Result<Json<Vec<transactions::Model>>, (StatusCode, String)> {
    if body.limit.is_none() {
        body.limit = Some(config.default_page_limit);
    }

    match networth_db::models::manage::transactions::get_transactions(&config.db, body).await {
        Ok(transactions) => Ok(Json(transactions)),
        Err(error) => {
            let now = chrono::Utc::now();
            tracing::error!(
                "error.transactions.could_not_fetch at {}, error: {}",
                now,
                error
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.transactions.could_not_fetch".to_string(),
            ))
        }
    }
}
