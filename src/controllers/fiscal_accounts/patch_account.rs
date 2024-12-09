use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use networth_db::models::entities::{
    accounts,
    sea_orm_active_enums::{AccountType, InstitutionName},
};
use sea_orm::DbErr;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UpdateAccountDetails {
    pub account_number: Option<String>,
    pub r#type: Option<AccountType>,
    pub institution_name: Option<InstitutionName>,
}

pub async fn patch_account(
    State(config): State<crate::config::types::Config>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateAccountDetails>,
) -> Result<Json<accounts::Model>, (StatusCode, String)> {
    let account = networth_db::models::manage::accounts::update_account(
        &config.db,
        id,
        payload.r#type,
        payload.account_number,
        payload.institution_name,
    )
    .await
    .map_err(|error| {
        if matches!(error, DbErr::RecordNotFound(_)) {
            return (
                StatusCode::NOT_FOUND,
                "error.fiscal_accounts.patch_account.not_found".to_string(),
            );
        } else {
            let now = chrono::Utc::now();
            tracing::error!(
                "error.fiscal_accounts.patch_account.could_not_update at {}, error: {}",
                now,
                error
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.fiscal_accounts.patch_account.could_not_update".to_string(),
            )
        }
    })?;

    Ok(Json(account))
}
