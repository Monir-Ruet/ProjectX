use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub role: String,
    pub email_verified: bool,
    pub image: Option<String>,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub is_active: bool,
    pub two_factor: bool,
    pub lockout_end: Option<DateTime<Utc>>,
    pub concurrency_stamp: Option<String>,
    pub failed_login_count: i32,
    pub last_failed_attempted: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        name: String,
        email: String,
        password: Option<String>,
        image: Option<String>,
        role: String,
    ) -> Self {
        Self {
            id: Uuid::nil(),
            name,
            email,
            password,
            role,
            email_verified: false,
            image,
            phone: None,
            phone_verified: false,
            is_active: true,
            two_factor: false,
            lockout_end: None,
            concurrency_stamp: None,
            failed_login_count: 0,
            last_failed_attempted: None,
            created_at: None,
            updated_at: None,
        }
    }
}
