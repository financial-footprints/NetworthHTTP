use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use networth_db::models::entities::accounts;
use uuid::Uuid;

pub async fn get_account(
    State(config): State<crate::config::types::Config>,
    Path(id): Path<Uuid>,
) -> Result<Json<accounts::Model>, (StatusCode, String)> {
    let account = networth_db::models::manage::accounts::get_account(&config.db, id)
        .await
        .map_err(|error| {
            let now = chrono::Utc::now();
            tracing::error!(
                "error.fiscal_accounts.get_account.could_not_retrieve at {}, error: {}",
                now,
                error
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.fiscal_accounts.get_account.could_not_retrieve".to_string(),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            "error.fiscal_accounts.get_account.not_found".to_string(),
        ))?;

    Ok(Json(account))
}
