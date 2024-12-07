use crate::config::types::Config;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use networth_db::models::{
    entities::staged_transactions,
    helpers::staged_transactions::{StagedTransactionFilter, StagedTransactionsQueryOptions},
};
use uuid::Uuid;

pub async fn get_transaction(
    State(config): State<Config>,
    Path((_, transaction_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<staged_transactions::Model>, (StatusCode, String)> {
    let transaction = networth_db::models::manage::staged_transactions::get_staged_transaction(
        &config.db,
        StagedTransactionsQueryOptions {
            filter: Some(StagedTransactionFilter {
                id: Some(transaction_id),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .await
    .map_err(|error| {
        let now = chrono::Utc::now();
        tracing::error!(
            "error.staged_transaction.could_not_fetch at {}, error: {}",
            now,
            error
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "error.staged_transaction.could_not_fetch".to_string(),
        )
    })?
    .ok_or((
        StatusCode::NOT_FOUND,
        "error.staged_transaction.not_found".to_string(),
    ))?;

    Ok(Json(transaction))
}
