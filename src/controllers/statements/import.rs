use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};

pub(super) async fn import_from_file(
    State(config): State<crate::config::types::Config>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut file_data = None;
    let mut file_secret = Some(String::new());

    while let Some(field) = match multipart.next_field().await {
        Ok(field) => field,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    } {
        let name = match field.name() {
            Some(name) => name.to_string(),
            None => return StatusCode::BAD_REQUEST.into_response(),
        };

        match name.as_str() {
            "file" => {
                file_data = match field.bytes().await {
                    Ok(data) => Some(data),
                    Err(_) => return StatusCode::BAD_REQUEST.into_response(),
                };
            }
            "secret" => {
                file_secret = match field.text().await {
                    Ok(text) => Some(text),
                    Err(_) => return StatusCode::BAD_REQUEST.into_response(),
                };
            }
            _ => {}
        }
    }
    let file_data = match file_data {
        Some(data) => data,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };

    let file_data = file_data.to_vec();

    let secret = file_secret.unwrap_or_default();

    let statement = networth_db::readers::get_statement_from_file_content(file_data, &secret)
        .map_err(|_| return StatusCode::BAD_REQUEST.into_response())
        .unwrap();

    match networth_db::models::writers::statement::set_stage_statement(&config.db, &statement).await
    {
        Ok(_) => (),
        Err(error) => panic!("{}", error),
    };

    StatusCode::CREATED.into_response()
}
