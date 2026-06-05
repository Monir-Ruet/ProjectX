use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Challenge {
    pub user_id: Uuid,
    pub challenge: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>
}

impl Challenge {
    pub fn new(user_id: Uuid, challenge: String, expires_at: Option<DateTime<Utc>>) -> Self {
        Challenge {
            user_id,
            challenge,
            created_at: Default::default(),
            expires_at,
        }
    }
}
