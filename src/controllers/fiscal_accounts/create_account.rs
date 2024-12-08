use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use networth_db::models::entities::sea_orm_active_enums::AccountType;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub account_number: String,
    pub r#type: AccountType,
}

pub async fn create_account(
    State(config): State<crate::config::types::Config>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<Uuid>), (StatusCode, String)> {
    let account_id = networth_db::models::manage::accounts::create_account(
        &config.db,
        &payload.account_number,
        &payload.r#type,
    )
    .await;

    match account_id {
        Ok(account) => Ok((StatusCode::CREATED, Json(account.id))),
        Err(err) => {
            let now = chrono::Utc::now();
            tracing::error!(
                "error.fiscal_accounts.create_account.could_not_save at {}, error: {}",
                now,
                err
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.fiscal_accounts.create_account.could_not_save".to_string(),
            ))
        }
    }
}
