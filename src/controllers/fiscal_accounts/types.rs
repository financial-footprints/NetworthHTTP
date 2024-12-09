use networth_db::models::entities::sea_orm_active_enums::{AccountType, InstitutionName};
use sea_orm::entity::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub(super) struct AccountResponse {
    pub(super) id: Uuid,
    pub(super) updated_at: DateTime,
    pub(super) account_number: String,
    pub(super) max_sequence_number: i64,
    pub(super) transaction_count: i64,
    pub(super) r#type: AccountType,
    pub(super) institution_name: InstitutionName,
    pub(super) balance: f32,
}
