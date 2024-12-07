use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::DbErr;
use uuid::Uuid;

pub async fn delete_import(
    State(config): State<crate::config::types::Config>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = networth_db::models::manage::imports::delete_import(&config.db, id)
        .await
        .map_err(|error| {
            let now = chrono::Utc::now();
            if matches!(error, DbErr::RecordNotFound(_)) {
                return (StatusCode::NOT_FOUND, error.to_string());
            }

            tracing::error!(
                "error.imports.delete_import.could_not_delete at {}, error: {}",
                now,
                error
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.imports.delete_import.could_not_delete".to_string(),
            );
        })
        .map(|_| StatusCode::NO_CONTENT);

    return result;
}
