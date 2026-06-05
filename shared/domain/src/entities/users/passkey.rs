use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Passkey {
    pub id: Uuid,
    pub credential_id: String,
    pub public_key: String,
    pub counter: i64,
    pub device_type: Option<String>,
    pub backed_up: bool,
    pub transports: Option<String>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}