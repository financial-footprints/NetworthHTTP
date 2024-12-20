use crate::config::types::Config;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

pub async fn delete_transaction(
    State(config): State<Config>,
    Path(transaction_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    networth_db::models::manage::transactions::delete_transaction(&config.db, transaction_id)
        .await
        .map_err(|error| {
            let now = chrono::Utc::now();
            tracing::error!(
                "error.transaction.could_not_delete at {}, error: {}",
                now,
                error
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.transaction.could_not_delete".to_string(),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
