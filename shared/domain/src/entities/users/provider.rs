use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct Provider {
    pub id: Uuid,
    pub name: String,
    pub account_id: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl Provider {
    pub fn new(name: String, account_id: String, user_id: Uuid) -> Self {
        Provider {
            id: Uuid::nil(),
            name,
            account_id,
            user_id,
            created_at: Utc::now(),
        }
    }
}
