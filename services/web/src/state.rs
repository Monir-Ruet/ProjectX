use crate::{repositories::Repository, services::Services};
use axum::extract::FromRef;
use cookie::Key;
use std::str::FromStr;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub service: Services,
    pub key: Key,
    pub(crate) tx: broadcast::Sender<String>,
}

impl AppState {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let database_url = std::env::var("DATABASE_URL")?;
        let options = sqlx::postgres::PgConnectOptions::from_str(&database_url)?;
        let pool = sqlx::PgPool::connect_lazy_with(options);

        let repo = Repository::new(pool);
        let service = Services::new(repo.clone());
        let (tx, _rx) = broadcast::channel(100);

        Ok(Self {
            service,
            key: Key::generate(),
            tx,
        })
    }
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}
