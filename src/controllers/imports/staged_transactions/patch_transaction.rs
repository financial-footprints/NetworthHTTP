use crate::config::types::Config;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use networth_db::models::entities::staged_transactions;
use sea_orm::prelude::Decimal;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UpdateTransactionDetails {
    date: Option<NaiveDateTime>,
    amount: Option<Decimal>,
    balance: Option<Decimal>,
    ref_no: Option<String>,
    description: Option<String>,
    sequence_number: Option<i64>,
}

pub async fn patch_transaction(
    State(config): State<Config>,
    Path((_, transaction_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateTransactionDetails>,
) -> Result<Json<staged_transactions::Model>, (StatusCode, String)> {
    let transaction = networth_db::models::manage::staged_transactions::update_staged_transaction(
        &config.db,
        transaction_id,
        payload.date,
        payload.amount,
        payload.balance,
        payload.ref_no,
        payload.description,
        payload.sequence_number,
    )
    .await
    .map_err(|error| {
        println!("error: {:?}", error);
        let now = chrono::Utc::now();
        tracing::error!(
            "error.transaction.could_not_update at {}, error: {}",
            now,
            error
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "error.transaction.could_not_update".to_string(),
        )
    })?;

    Ok(Json(transaction))
}
