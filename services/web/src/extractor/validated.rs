use std::collections::HashMap;

use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

pub struct Validated<T>(pub T);

impl<S, T> FromRequest<S> for Validated<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(body) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| AppError::BadRequest("Failed to parse JSON".into()))?;

        body.validate().map_err(|errors| {
            let mut map = HashMap::new();
            for (field, field_errors) in errors.field_errors() {
                let messages = field_errors
                    .iter()
                    .map(|error| {
                        error
                            .message
                            .clone()
                            .unwrap_or_else(|| std::borrow::Cow::from(error.code.to_string()))
                            .to_string()
                    })
                    .collect::<Vec<_>>();

                map.insert(field.to_string(), messages);
            }

            AppError::BadRequest(serde_json::json!({ "validation_errors": map }))
        })?;
        Ok(Validated(body))
    }
}
