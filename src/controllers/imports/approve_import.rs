use crate::config::types::Config;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::sqlx;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ApproveImportDetails {
    pub account_id: Uuid,
}

pub(super) async fn approve_import(
    State(config): State<Config>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ApproveImportDetails>,
) -> Result<StatusCode, (StatusCode, String)> {
    match networth_db::models::manage::imports::approve_import(&config.db, id, payload.account_id)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(error) => {
            if let sea_orm::DbErr::Exec(sea_orm::RuntimeErr::SqlxError(sqlx::Error::Database(
                db_error,
            ))) = &error
            {
                if let Some(constraint) = db_error.constraint() {
                    if constraint == "fk_txn_account_id" {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "error.import.account_not_found".to_string(),
                        ));
                    }
                }
            }

            let now = chrono::Utc::now();
            tracing::error!(
                "error.import.could_not_approve at {}, error: {}",
                now,
                error
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.import.could_not_approve".to_string(),
            ))
        }
    }
}
