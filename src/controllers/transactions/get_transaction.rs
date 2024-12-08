use crate::config::types::Config;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use networth_db::models::{
    entities::transactions,
    helpers::transactions::{TransactionFilter, TransactionsQueryOptions},
};
use uuid::Uuid;

pub async fn get_transaction(
    State(config): State<Config>,
    Path(transaction_id): Path<Uuid>,
) -> Result<Json<transactions::Model>, (StatusCode, String)> {
    let transaction = networth_db::models::manage::transactions::get_transaction(
        &config.db,
        TransactionsQueryOptions {
            filter: Some(TransactionFilter {
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
            "error.transaction.could_not_fetch at {}, error: {}",
            now,
            error
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "error.transaction.could_not_fetch".to_string(),
        )
    })?
    .ok_or((
        StatusCode::NOT_FOUND,
        "error.transaction.not_found".to_string(),
    ))?;

    Ok(Json(transaction))
}
