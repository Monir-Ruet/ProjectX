use std::{env, sync::OnceLock};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub grpc_url: String,
}

impl AppConfig {
    fn load() -> Self {
        AppConfig {
            grpc_url: env::var("GRPC_URL").unwrap_or_else(|_| "http://127.0.0.1:50051".into()),
        }
    }
}

static APP_CONFIGURATION: OnceLock<AppConfig> = OnceLock::new();

pub fn get_or_init() -> &'static AppConfig {
    APP_CONFIGURATION.get_or_init(AppConfig::load)
}
