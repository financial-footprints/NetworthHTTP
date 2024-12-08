use crate::config::types::Config;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use sea_orm::sqlx;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateTransactionDetails {
    pub description: String,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub balance: f32,
    pub sequence_number: i64,
    pub ref_no: String,
}

pub async fn create_transaction(
    State(config): State<Config>,
    Path(import_id): Path<Uuid>,
    Json(payload): Json<CreateTransactionDetails>,
) -> Result<(StatusCode, Json<Uuid>), (StatusCode, String)> {
    let transaction = networth_db::models::helpers::staged_transactions::build_staged_transaction(
        payload.amount,
        import_id,
        payload.date,
        payload.balance,
        payload.sequence_number,
        payload.ref_no,
        payload.description,
    );

    let transaction = networth_db::models::manage::staged_transactions::create_staged_transaction(
        &config.db,
        transaction,
    )
    .await
    .map_err(|error| {
        if let sea_orm::DbErr::Exec(sea_orm::RuntimeErr::SqlxError(sqlx::Error::Database(
            db_error,
        ))) = &error
        {
            if let Some(constraint) = db_error.constraint() {
                if constraint == "uniq_importid_sequencenumber" {
                    return (
                        StatusCode::BAD_REQUEST,
                        "error.staged_transactions.duplicate_sequence_number".to_string(),
                    );
                }
            }
        }

        let now = chrono::Utc::now();
        tracing::error!(
            "error.staged_transactions.could_not_create at {}, error: {}",
            now,
            error
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "error.staged_transactions.could_not_create".to_string(),
        )
    })?;

    Ok((StatusCode::CREATED, Json(transaction.id)))
}
