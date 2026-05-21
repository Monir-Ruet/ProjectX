use axum::{extract::FromRequestParts, http::request::Parts};

use crate::error::AppError;
use crate::extractor::auth::AuthorizedUser;
use crate::state::AppState;

pub trait Role {
    const ROLE: &'static str;
}

pub struct AppUser;

impl Role for AppUser {
    const ROLE: &'static str = "user";
}

pub struct Admin;

impl Role for Admin {
    const ROLE: &'static str = "admin";
}

pub struct Moderator;

impl Role for Moderator {
    const ROLE: &'static str = "moderator";
}

pub struct RequireRole<R: Role>(pub AuthorizedUser, pub std::marker::PhantomData<R>);

impl<R> FromRequestParts<AppState> for RequireRole<R>
where
    R: Role,
    AppState: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user = AuthorizedUser::from_request_parts(parts, state).await?;

        if user.role != R::ROLE {
            return Err(AppError::Forbidden("Forbidden".into()));
        }

        Ok(Self(user, std::marker::PhantomData))
    }
}
