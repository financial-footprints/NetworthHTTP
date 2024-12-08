use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::DbErr;
use uuid::Uuid;

pub async fn delete_account(
    State(config): State<crate::config::types::Config>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = networth_db::models::manage::accounts::delete_account(&config.db, id)
        .await
        .map_err(|error| {
            let now = chrono::Utc::now();
            if matches!(error, DbErr::RecordNotFound(_)) {
                return (
                    StatusCode::BAD_REQUEST,
                    "user.input.incorrect_account_id".to_string(),
                );
            }

            tracing::error!(
                "error.fiscal_accounts.delete_account.could_not_delete at {}, error: {}",
                now,
                error
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.fiscal_accounts.delete_account.could_not_delete".to_string(),
            );
        })
        .map(|_| StatusCode::NO_CONTENT);

    return result;
}
