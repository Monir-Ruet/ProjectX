use std::str::FromStr;

use axum::extract::FromRef;
use cookie::Key;

use crate::{repositories::Repository, services::Services};

#[derive(Clone)]
pub struct AppState {
    pub service: Services,
    pub key: Key,
}

impl AppState {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let database_url = std::env::var("DATABASE_URL")?;
        let options = sqlx::postgres::PgConnectOptions::from_str(&database_url)?;
        let pool = sqlx::PgPool::connect_with(options).await?;

        let repo = Repository::new(pool);
        let service = Services::new(repo.clone());

        Ok(Self {
            service: service,
            key: Key::generate(),
        })
    }
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}
