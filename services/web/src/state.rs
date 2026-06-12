use crate::{repositories::Repository, services::Services};
use axum::extract::FromRef;
use cookie::Key;
use redis::aio::MultiplexedConnection;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::broadcast;
use webauthn_rs::prelude::Url;
use webauthn_rs::{Webauthn, WebauthnBuilder};

#[derive(Clone)]
pub struct AppState {
    pub service: Services,
    pub key: Key,
    pub(crate) tx: broadcast::Sender<String>,
    pub web_auth: Arc<Webauthn>,
    pub redis: MultiplexedConnection,
}

impl AppState {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let database_url = std::env::var("DATABASE_URL")?;
        let redis_url = std::env::var("REDIS_URL")?;
        let rp_id = std::env::var("RP_ID")?;
        let rp_origin = std::env::var("RP_ORIGIN")?;
        let options = sqlx::postgres::PgConnectOptions::from_str(&database_url)?;
        let pool = sqlx::PgPool::connect_lazy_with(options);

        let repo = Repository::new(pool);
        let service = Services::new(repo.clone());
        let (tx, _rx) = broadcast::channel(100);

        let rp_origin = Url::parse(&rp_origin).expect("Invalid URL");
        let builder = WebauthnBuilder::new(&rp_id, &rp_origin).expect("Invalid configuration");
        let builder = builder.rp_name("Axum Webauthn-rs");
        let webauthn = Arc::new(builder.build().expect("Invalid configuration"));

        let client = redis::Client::open(redis_url)?;
        let con = client.get_multiplexed_async_connection().await?;

        Ok(Self {
            service,
            key: Key::generate(),
            tx,
            web_auth: webauthn,
            redis: con,
        })
    }
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}
