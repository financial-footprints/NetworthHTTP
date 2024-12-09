pub mod types;

use std::env;

pub(crate) async fn get_config() -> types::Config {
    dotenvy::dotenv().ok();

    if let Ok(env_file_path) = env::var("ENV_FILE_PATH") {
        let reason = "Failed to load environment variables from specified path";
        dotenvy::from_path(&env_file_path).expect(&format!(
            "error.config.get_config.cannot_load_with_path: {}",
            reason
        ));
    }

    let database_url =
        env::var("DATABASE_URL").expect("error.config.get_config.database_url_not_found");

    let default_page_limit = env::var("DEFAULT_PAGE_LIMIT")
        .unwrap_or_else(|_| "50".to_string())
        .parse()
        .expect("error.config.get_config.invalid_default_page_limit");

    let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "".to_string());

    return types::Config {
        db: get_database_connection(&database_url).await,
        default_page_limit,
        cors_allowed_origins,
    };
}

async fn get_database_connection(database_url: &str) -> sea_orm::DatabaseConnection {
    sea_orm::Database::connect(database_url)
        .await
        .expect("error.config.get_config.cannot_connect_to_database")
}
