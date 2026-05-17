use crate::config::get_or_init;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
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
            jti: jti,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }
}

pub fn encode_token(claims: &AccessClaims) -> String {
    let config = get_or_init();
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret_key.as_bytes()),
    )
    .unwrap();
    token
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_token() {
        dotenv::dotenv().ok();

        let now = 1_700_000_000usize;

        let claims = AccessClaims {
            sub: Uuid::new_v4(),
            exp: now as i64 + 3600,
            iat: now as i64,
            typ: "access_token".to_string(),
            email: "john.doe@example.com".to_string(),
            role: "user".to_string(),
            jti: Uuid::new_v4(),
        };

        let token = encode_token(&claims);

        assert!(!token.is_empty());
        assert!(token.split('.').count() == 3); // basic JWT structure check
    }
}
