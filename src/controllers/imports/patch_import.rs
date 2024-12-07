use crate::config::types::Config;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use networth_db::models::entities::imports;
use sea_orm::DbErr;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UpdateImportDetails {
    pub account_number: Option<String>,
}

pub(super) async fn patch_import(
    State(config): State<Config>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateImportDetails>,
) -> Result<Json<imports::Model>, (StatusCode, String)> {
    let import =
        networth_db::models::manage::imports::update_import(&config.db, id, payload.account_number)
            .await
            .map_err(|error| {
                if matches!(error, DbErr::RecordNotFound(_)) {
                    return (
                        StatusCode::NOT_FOUND,
                        "error.imports.patch_import.not_found".to_string(),
                    );
                }

                let now = chrono::Utc::now();
                tracing::error!(
                    "error.imports.patch_import.could_not_update at {}, error: {}",
                    now,
                    error
                );
                println!("error: {:?}", error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "error.imports.patch_import.could_not_update".to_string(),
                )
            })?;

    Ok(Json(import))
}
