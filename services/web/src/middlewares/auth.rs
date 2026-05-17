use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use uuid::Uuid;

use crate::{error::AppError, utils::jwt::AccessClaims};
use crate::{state::AppState, utils::token};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
}

impl From<AccessClaims> for AuthUser {
    fn from(claims: AccessClaims) -> Self {
        Self {
            id: claims.sub,
            email: claims.email,
            role: claims.role,
        }
    }
}

impl AuthUser {
    pub fn has_role(&self, role: &str) -> bool {
        self.role == role
    }

    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|role| self.has_role(role))
    }
}

impl FromRequestParts<AppState> for AuthUser
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
            .map_err(|_| AppError::Unauthorized("a".into()))?;

        let bearer = t.0;

        let claims = token::validate_access_token(bearer.token())
            .map_err(|_| AppError::Unauthorized("b".into()))?;

        Ok(AuthUser::from(claims))
    }
}

pub struct OptionalAuthUser(pub Option<AuthUser>);

impl FromRequestParts<AppState> for OptionalAuthUser
where
    AppState: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        match AuthUser::from_request_parts(parts, state).await {
            Ok(user) => Ok(OptionalAuthUser(Some(user))),
            Err(_) => Ok(OptionalAuthUser(None)),
        }
    }
}

pub struct RequireRole {
    pub role: String,
}

impl FromRequestParts<AppState> for RequireRole
where
    AppState: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user = AuthUser::from_request_parts(parts, state).await?;

        Ok(RequireRole {
            role: user.role.clone(),
        })
    }
}
