use super::types::AccountResponse;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use networth_db::models::helpers::accounts::{AccountFilter, AccountsQueryOptions};
use uuid::Uuid;

pub async fn get_account(
    State(config): State<crate::config::types::Config>,
    Path(id): Path<Uuid>,
) -> Result<Json<AccountResponse>, (StatusCode, String)> {
    let (account, balance) =
        networth_db::models::manage::joins::accounts_transactions::get_account_with_balance(
            &config.db,
            AccountsQueryOptions {
                filter: Some(AccountFilter {
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

    let response = AccountResponse {
        id: account.id,
        updated_at: account.updated_at,
        account_number: account.account_number,
        max_sequence_number: account.max_sequence_number,
        transaction_count: account.transaction_count,
        r#type: account.r#type,
        institution_name: account.institution_name,
        balance,
    };

    Ok(Json(response))
}
