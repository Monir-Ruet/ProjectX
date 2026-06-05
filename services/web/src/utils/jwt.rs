use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub typ: String,
    pub email: String,
    pub role: String,
    pub jti: Uuid,
}

impl AccessClaims {
    pub fn new(
        user_id: Uuid,
        email: String,
        role: String,
        expires_in: Duration,
        jti: Uuid,
    ) -> Self {
        let now = Utc::now();

        Self {
            sub: user_id,
            exp: (now + expires_in).timestamp(),
            iat: now.timestamp(),
            typ: "access_token".to_string(),
            email,
            role,
            jti,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub typ: String,
    pub family: Uuid,
    pub jti: Uuid,
}

impl RefreshClaims {
    pub fn new(user_id: Uuid, expires_in: Duration, jti: Uuid) -> Self {
        let now = Utc::now();

        Self {
            sub: user_id,
            exp: (now + expires_in).timestamp(),
            iat: now.timestamp(),
            typ: "refresh_token".to_string(),
            family: Uuid::new_v4(),
            jti,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PassKeyClaims {
    pub id: Uuid,
    pub exp: i64,
}

impl PassKeyClaims {
    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }
}