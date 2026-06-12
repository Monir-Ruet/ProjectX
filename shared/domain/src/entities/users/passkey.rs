use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Passkey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub cred_id: Vec<u8>,
    pub passkey: serde_json::Value,
    pub device_type: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Passkey {
    pub fn new(user_id: Uuid, cred_id: Vec<u8>, passkey: serde_json::Value, device_type: Option<String>) -> Self {
        Passkey {
            id: Default::default(),
            user_id,
            cred_id,
            passkey,
            device_type,
            created_at: Default::default(),
        }
    }
}