use axum::{extract::State, http::StatusCode, Json};
use networth_db::models::helpers::accounts::AccountsQueryOptions;

use super::types::AccountResponse;

pub async fn get_accounts(
    State(config): State<crate::config::types::Config>,
    Json(mut body): Json<AccountsQueryOptions>,
) -> Result<Json<Vec<AccountResponse>>, (StatusCode, String)> {
    if body.limit.is_none() {
        body.limit = Some(config.default_page_limit);
    }
    let accounts =
        networth_db::models::manage::joins::accounts_transactions::get_accounts_with_balance(
            &config.db, body,
        )
        .await
        .map_err(|error| {
            let now = chrono::Utc::now();
            tracing::error!(
                "error.fiscal_accounts.get_accounts.could_not_retrieve at {}, error: {}",
                now,
                error
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.fiscal_accounts.get_accounts.could_not_retrieve".to_string(),
            )
        })?;

    let response = accounts
        .into_iter()
        .map(|(account, balance)| AccountResponse {
            id: account.id,
            updated_at: account.updated_at,
            account_number: account.account_number,
            max_sequence_number: account.max_sequence_number,
            transaction_count: account.transaction_count,
            r#type: account.r#type,
            institution_name: account.institution_name,
            balance,
        })
        .collect();
    Ok(Json(response))
}
