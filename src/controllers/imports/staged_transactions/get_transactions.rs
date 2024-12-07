use crate::config::types::Config;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use networth_db::models::{
    entities::staged_transactions,
    helpers::{
        staged_transactions::{
            StagedTransactionFilter, StagedTransactionSort, StagedTransactionsQueryOptions,
        },
        SortDirection,
    },
};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get_transactions(
    State(config): State<Config>,
    Path(import_id): Path<Uuid>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<staged_transactions::Model>>, (StatusCode, String)> {
    let limit = params
        .get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(config.default_page_limit.clone());
    let offset = params
        .get("offset")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let id = match params.get("id") {
        Some(id_str) => Some(
            Uuid::parse_str(id_str)
                .map_err(|error| (StatusCode::BAD_REQUEST, format!("Invalid UUID: {}", error)))?,
        ),
        None => None,
    };

    match networth_db::models::manage::staged_transactions::get_staged_transactions(
        &config.db,
        StagedTransactionsQueryOptions {
            filter: Some(StagedTransactionFilter {
                id,
                import_id: Some(import_id),
                ..Default::default()
            }),
            limit: Some(limit),
            offset: Some(offset),
            sort: Some(StagedTransactionSort {
                column: staged_transactions::Column::SequenceNumber,
                direction: SortDirection::Asc,
            }),
            ..Default::default()
        },
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
