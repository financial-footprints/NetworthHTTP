use std::collections::HashMap;

use crate::config::types::Config;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use networth_db::models::{entities::imports, helpers::imports::ImportsQueryOptions};

pub(super) async fn get_imports(
    State(config): State<Config>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<imports::Model>>, (StatusCode, String)> {
    let limit = params
        .get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(config.default_page_limit.clone());
    let offset = params
        .get("offset")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    match networth_db::models::manage::imports::get_imports(
        &config.db,
        ImportsQueryOptions {
            limit: Some(limit),
            offset: Some(offset),
            ..Default::default()
        },
    )
    .await
    {
        Ok(imports) => Ok(Json(imports)),
        Err(error) => {
            let now = chrono::Utc::now();
            tracing::error!("error.import.could_not_fetch at {}, error: {}", now, error);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.import.could_not_fetch".to_string(),
            ))
        }
    }
}
