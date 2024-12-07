use crate::config::types::Config;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use networth_db::models::{
    entities::imports,
    helpers::imports::{ImportFilter, ImportsQueryOptions},
};
use uuid::Uuid;

pub(super) async fn get_import(
    State(config): State<Config>,
    Path(id): Path<Uuid>,
) -> Result<Json<imports::Model>, (StatusCode, String)> {
    let import = networth_db::models::manage::imports::get_import(
        &config.db,
        ImportsQueryOptions {
            filter: Some(ImportFilter {
                id: Some(id),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .await
    .map_err(|error| {
        let now = chrono::Utc::now();
        tracing::error!(
            "error.import.could_not_fetch_by_id at {}, error: {}",
            now,
            error
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "error.import.could_not_fetch_by_id".to_string(),
        )
    })?
    .ok_or((StatusCode::NOT_FOUND, "error.import.not_found".to_string()))?;

    Ok(Json(import))
}
