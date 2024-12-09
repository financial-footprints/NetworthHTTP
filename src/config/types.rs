#[derive(Clone)]
pub struct Config {
    pub db: sea_orm::DatabaseConnection,
    pub default_page_limit: u64,
    pub cors_allowed_origins: String,
}
