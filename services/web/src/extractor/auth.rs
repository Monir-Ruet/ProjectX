use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use uuid::Uuid;

use crate::{error::AppError, utils::jwt::AccessClaims};
use crate::{state::AppState, utils::token};

#[derive(Debug, Clone)]
pub struct AuthorizedUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub jti: Uuid,
}

impl From<AccessClaims> for AuthorizedUser {
    fn from(claims: AccessClaims) -> Self {
        Self {
            id: claims.sub,
            email: claims.email,
            role: claims.role,
            jti: claims.jti
        }
    }
}

impl AuthorizedUser {
    pub fn has_role(&self, role: &str) -> bool {
        self.role == role
    }

    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|role| self.has_role(role))
    }
}

impl FromRequestParts<AppState> for AuthorizedUser
where
    AppState: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let t = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized("Access denied".into()))?;

        let bearer = t.0;

        let claims = token::validate_access_token(bearer.token())
            .map_err(|_| AppError::Unauthorized("Access denied".into()))?;

        Ok(AuthorizedUser::from(claims))
    }
}

pub struct OptionalAuthUser(pub Option<AuthorizedUser>);

impl FromRequestParts<AppState> for OptionalAuthUser
where
    AppState: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        match AuthorizedUser::from_request_parts(parts, state).await {
            Ok(user) => Ok(OptionalAuthUser(Some(user))),
            Err(_) => Ok(OptionalAuthUser(None)),
        }
    }
}
