use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub secret_key: String,
    pub access_token_expiry: i64,
    pub refresh_token_expiry: i64,
    pub max_lockout_num: i32,
}

impl AppConfig {
    fn load() -> Self {
        AppConfig {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            secret_key: std::env::var("SECRET_KEY").expect("SECRET_KEY must be set"),
            access_token_expiry: 15 * 60,            // 15 minutes
            refresh_token_expiry: 30 * 24 * 60 * 60, // 30 days
            max_lockout_num: 10,
        }
    }
}

static APP_CONFIGURATION: OnceLock<AppConfig> = OnceLock::new();

pub fn get_or_init() -> &'static AppConfig {
    APP_CONFIGURATION.get_or_init(AppConfig::load)
}
