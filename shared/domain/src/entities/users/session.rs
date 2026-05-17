use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub ip: String,
    pub user_agent: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub fn new(ip: String, user_agent: String, user_id: Uuid, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::nil(),
            ip,
            user_agent,
            user_id,
            expires_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        assert!(self.id != Uuid::nil(), "Session ID should not be nil");
        Utc::now() > self.expires_at
    }
}
