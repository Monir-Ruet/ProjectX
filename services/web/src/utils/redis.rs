use crate::error::AppError;
use deadpool_redis::Connection;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};

pub async fn store<T: Serialize>(
    redis: &mut Connection,
    key: &str,
    value: &T,
    ttl_seconds: i64,
) -> Result<(), AppError> {
    let serialized = serde_json::to_string(value)
        .map_err(|_| AppError::Internal("serialization failed".into()))?;

    redis::pipe()
        .set(key, serialized)
        .expire(key, ttl_seconds)
        .query_async(redis)
        .await
        .map_err(|_| AppError::Internal("failed to save state".into()))
}

pub async fn load<T: DeserializeOwned>(
    redis: &mut Connection,
    key: &str,
) -> Result<T, AppError> {
    let stored: Option<String> = redis
        .get(key)
        .await
        .map_err(|_| AppError::Internal("redis error".into()))?;

    let stored = stored.ok_or_else(|| {
        AppError::Unauthorized("state not found".into())
    })?;

    serde_json::from_str(&stored)
        .map_err(|_| AppError::Internal("invalid state".into()))
}

pub async fn delete(
    redis: &mut Connection,
    key: &str,
) -> Result<(), AppError> {
    redis
        .del(key)
        .await
        .map_err(|_| AppError::Internal("failed to delete state".into()))
}