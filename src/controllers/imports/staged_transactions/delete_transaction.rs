use crate::config::types::Config;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

pub async fn delete_transaction(
    State(config): State<Config>,
    Path((_, transaction_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, String)> {
    networth_db::models::manage::staged_transactions::delete_staged_transaction(
        &config.db,
        transaction_id,
    )
    .await
    .map_err(|error| {
        let now = chrono::Utc::now();
        tracing::error!(
            "error.staged_transactions.could_not_delete at {}, error: {}",
            now,
            error
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "error.staged_transactions.could_not_delete".to_string(),
        )
    })?;

    Ok(StatusCode::NO_CONTENT)
}
