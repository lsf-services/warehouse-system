#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub app_name: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://user:pass@localhost/db".to_string()),
            app_name: std::env::var("APP_NAME")
                .unwrap_or_else(|_| "warehouse-app".to_string()),
        }
    }
}
