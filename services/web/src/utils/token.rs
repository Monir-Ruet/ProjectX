use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use thiserror::Error;
use uuid::Uuid;

use crate::models::users::signin::AccessTokenResponse;
use crate::utils::jwt::PassKeyClaims;
use crate::{
    config::{self},
    utils::jwt::{AccessClaims, RefreshClaims},
};

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Token encoding failed")]
    EncodingFailed(#[from] jsonwebtoken::errors::Error),

    #[error("Token validation failed")]
    ValidationFailed,

    #[error("Token expired")]
    Expired,

    #[error("Invalid token type")]
    InvalidType,
}

pub fn generate_access_token(
    user_id: Uuid,
    email: String,
    role: String,
    jti: Uuid,
) -> Result<String, TokenError> {
    let config = config::get_or_init();
    let encoding_key = EncodingKey::from_secret(config.secret_key.as_bytes());
    let claims = AccessClaims::new(user_id, email, role, Duration::minutes(15), jti);

    encode(&Header::default(), &claims, &encoding_key).map_err(TokenError::EncodingFailed)
}

pub fn generate_refresh_token(user_id: Uuid, jti: Uuid) -> Result<String, TokenError> {
    let config = config::get_or_init();
    let encoding_key = EncodingKey::from_secret(config.secret_key.as_bytes());
    let claims = RefreshClaims::new(user_id, Duration::days(30), jti);

    encode(&Header::default(), &claims, &encoding_key).map_err(TokenError::EncodingFailed)
}

pub fn generate_token_pair(
    user_id: Uuid,
    email: String,
    role: String,
    jti: Uuid,
) -> Result<AccessTokenResponse, TokenError> {
    Ok(AccessTokenResponse {
        access_token: generate_access_token(user_id, email, role, jti)?,
        refresh_token: generate_refresh_token(user_id, jti.clone())?,
        token_type: "Bearer".to_string(),
        expires_in: Duration::minutes(15).num_seconds() as u64,
    })
}

pub fn validate_access_token(token: &str) -> Result<AccessClaims, TokenError> {
    let mut validation = Validation::default();
    validation.validate_exp = true;

    let config = config::get_or_init();
    let decoding_key = DecodingKey::from_secret(config.secret_key.as_bytes());
    let token_data: TokenData<AccessClaims> =
        decode(token, &decoding_key, &validation).map_err(|_| TokenError::ValidationFailed)?;

    if token_data.claims.typ != "access_token" {
        return Err(TokenError::InvalidType);
    } else if token_data.claims.is_expired() {
        return Err(TokenError::Expired);
    }

    Ok(token_data.claims)
}

pub fn validate_refresh_token(token: &str) -> Result<RefreshClaims, TokenError> {
    let mut validation = Validation::default();
    validation.validate_exp = true;

    let config = config::get_or_init();
    let decoding_key = DecodingKey::from_secret(config.secret_key.as_bytes());
    let token_data: TokenData<RefreshClaims> =
        decode(token, &decoding_key, &validation).map_err(|_| TokenError::ValidationFailed)?;

    if token_data.claims.typ != "refresh_token" {
        return Err(TokenError::InvalidType);
    } else if token_data.claims.is_expired() {
        return Err(TokenError::Expired);
    }

    Ok(token_data.claims)
}

pub fn generate_passkey_token(id: Uuid) -> Result<String, TokenError> {
    let config = config::get_or_init();
    let encoding_key = EncodingKey::from_secret(config.secret_key.as_bytes());
    let claims = PassKeyClaims {
        id,
        exp: (Utc::now() + Duration::seconds(30)).timestamp(),
    };

    encode(&Header::default(), &claims, &encoding_key).map_err(TokenError::EncodingFailed)
}

pub fn validate_passkey_token(token: &str) -> Result<PassKeyClaims, TokenError> {
    let mut validation = Validation::default();
    validation.validate_exp = true;

    let config = config::get_or_init();
    let decoding_key = DecodingKey::from_secret(config.secret_key.as_bytes());
    let token_data: TokenData<PassKeyClaims> =
        decode(token, &decoding_key, &validation).map_err(|_| TokenError::ValidationFailed)?;
    if token_data.claims.is_expired() {
        return Err(TokenError::Expired);
    }
    Ok(token_data.claims)
}