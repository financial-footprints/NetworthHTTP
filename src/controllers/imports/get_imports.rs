use crate::config::types::Config;
use axum::{extract::State, http::StatusCode, Json};
use networth_db::models::{entities::imports, helpers::imports::ImportsQueryOptions};

pub(super) async fn get_imports(
    State(config): State<Config>,
    Json(mut body): Json<ImportsQueryOptions>,
) -> Result<Json<Vec<imports::Model>>, (StatusCode, String)> {
    if body.limit.is_none() {
        body.limit = Some(config.default_page_limit);
    }
    match networth_db::models::manage::imports::get_imports(&config.db, body).await {
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
