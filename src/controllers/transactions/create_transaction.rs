use crate::config::types::Config;
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::sqlx;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateTransactionDetails {
    pub description: String,
    pub amount: f32,
    pub account_id: Uuid,
    pub date: chrono::NaiveDateTime,
    pub sequence_number: i64,
    pub ref_no: String,
}

pub async fn create_transaction(
    State(config): State<Config>,
    Json(payload): Json<CreateTransactionDetails>,
) -> Result<(StatusCode, Json<Uuid>), (StatusCode, String)> {
    let transaction = networth_db::models::helpers::transactions::build_transaction(
        payload.amount,
        payload.account_id,
        payload.date,
        payload.sequence_number,
        payload.ref_no,
        payload.description,
    );

    let transaction =
        networth_db::models::manage::transactions::create_transaction(&config.db, transaction)
            .await
            .map_err(|error| {
                if let sea_orm::DbErr::Exec(sea_orm::RuntimeErr::SqlxError(
                    sqlx::Error::Database(db_error),
                )) = &error
                {
                    if let Some(constraint) = db_error.constraint() {
                        if constraint == "uniq_accountid_sequencenumber" {
                            return (
                                StatusCode::BAD_REQUEST,
                                "error.transactions.duplicate_sequence_number".to_string(),
                            );
                        }
                    }
                }

                let now = chrono::Utc::now();
                tracing::error!(
                    "error.transactions.could_not_create at {}, error: {}",
                    now,
                    error
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error.transactions.could_not_create".to_string(),
                )
            })?;

    Ok((StatusCode::CREATED, Json(transaction.id)))
}
