use axum::{extract::State, http::StatusCode, Json};
use networth_db::models::entity::sea_orm_active_enums::AccountType;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AccountDetails {
    pub account_number: String,
    pub institution: String,
    pub r#type: AccountType,
}

pub async fn create(
    State(config): State<crate::config::types::Config>,
    Json(payload): Json<AccountDetails>,
) -> Result<(StatusCode, Json<Uuid>), (StatusCode, String)> {
    // let account_id = networth_db::models::writers::account::create_account(
    //     &config.db,
    //     &payload.account_number,
    //     &payload.institution,
    //     &payload.r#type,
    // )
    // .await
    // .map_err(|e| {
    //     (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         format!("Could not create fiscal account"),
    //     )
    // })?;

    Ok((StatusCode::CREATED, Json(Uuid::new_v4())))
}
