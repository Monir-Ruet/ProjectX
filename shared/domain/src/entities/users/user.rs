use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub role: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(name: String, email: String, password: String, role: String) -> Self {
        Self {
            id: Uuid::nil(),
            name,
            email,
            password: Some(password),
            role,
            created_at: None,
            updated_at: None,
        }
    }
}
