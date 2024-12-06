use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

pub(super) async fn create_import(
    State(config): State<crate::config::types::Config>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Uuid>), (StatusCode, String)> {
    let mut file_data = None;
    let mut file_secret = Some(String::new());

    while let Some(field) = match multipart.next_field().await {
        Ok(field) => field,
        Err(_) => {
            return Err((StatusCode::BAD_REQUEST, format!("user.invalid.input")));
        }
    } {
        let name = match field.name() {
            Some(name) => name.to_string(),
            None => {
                return Err((StatusCode::BAD_REQUEST, format!("user.invalid.input")));
            }
        };

        match name.as_str() {
            "file" => {
                file_data = match field.bytes().await {
                    Ok(data) => Some(data),
                    Err(_) => {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "user.form.file.not_provided".to_string(),
                        ));
                    }
                };
            }
            "secret" => {
                file_secret = match field.text().await {
                    Ok(text) => Some(text),
                    Err(_) => {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "user.form.secret.not_provided".to_string(),
                        ));
                    }
                };
            }
            _ => {}
        }
    }

    let file_data = match file_data {
        Some(data) => data,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "user.form.file.is_empty".to_string(),
            ));
        }
    };

    let file_data = file_data.to_vec();
    let secret = file_secret.unwrap_or_default();

    let statement = networth_db::readers::get_statement_from_file_content(file_data, &secret)
        .map_err(|error| {
            let now = chrono::Utc::now();
            tracing::error!(
                "error.import.could_not_parse at {}, error: {}",
                now,
                error
            );
            (
                StatusCode::BAD_REQUEST,
                "error.import.could_not_parse".to_string(),
            )
        })?;

    match networth_db::models::manage::imports::create_import(&config.db, &statement).await {
        Ok(id) => Ok((StatusCode::CREATED, Json(id))),
        Err(error) => {
            let now = chrono::Utc::now();
            tracing::error!(
                "error.import.could_not_save at {}, error: {}",
                now,
                error
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "error.import.could_not_save".to_string(),
            ))
        }
    }
}
