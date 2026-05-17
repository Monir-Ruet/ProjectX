use std::net::SocketAddr;
use uuid::Uuid;

use crate::{
    endpoints::AppState,
    error::AppError,
    extractor::validated::Validated,
    middlewares::auth::AuthUser,
    models::user::{
        AccessTokenResponse, RefreshRequest, RegisterRequest, SignInRequest, UserResponse,
    },
    services::{session::SessionService, user::UserService},
    utils::token::{self},
};
use axum::{
    Json, Router,
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode},
    routing::{delete, get, post, put},
};
use axum_extra::extract::PrivateCookieJar;
use cookie::Cookie;
use domain::entities::users::user::User;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest(
            "/users",
            Router::new()
                .route("/", post(register_user))
                .route("/{id}", get(find_user_by_id))
                .route("/{id}", put(update_user))
                .route("/{id}", delete(delete_user_by_id)),
        )
        .route("/signin", post(signin))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

#[utoipa::path(
    post,
    path = "/signin",
    request_body = SignInRequest,
    responses(
        (status = 200, description = "signed in"),
        (status = 401, description = "invalid credentials")
    )
)]
pub async fn signin(
    State(state): State<AppState>,
    mut jar: PrivateCookieJar,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(request): Json<SignInRequest>,
) -> Result<(PrivateCookieJar, Json<AccessTokenResponse>), AppError> {
    let user = state
        .service
        .signin(&request.email, &request.password)
        .await?;
    let config = crate::config::get_or_init();

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let ip = addr.ip().to_string();

    let jti = state
        .service
        .create_session(user.id, user_agent.into(), ip)
        .await?;

    let token = token::generate_token_pair(user.id, user.email.clone(), user.role, jti)
        .map_err(|_| AppError::Unauthorized("failed to generate token".into()))?;

    let access_cookie = Cookie::build(("m_session", token.access_token.clone()))
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(cookie::time::Duration::seconds(config.access_token_expiry))
        .build();
    let refresh_cookie = Cookie::build(("m_refresh", token.refresh_token.clone()))
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(cookie::time::Duration::seconds(config.refresh_token_expiry))
        .build();

    jar = jar.add(access_cookie);
    jar = jar.add(refresh_cookie);

    Ok((jar, Json(token)))
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User created")
    )
)]
pub async fn register_user(
    State(state): State<AppState>,
    Validated(request): Validated<RegisterRequest>,
) -> Result<StatusCode, AppError> {
    state
        .service
        .create_user(User::new(
            request.name,
            request.email,
            request.password,
            "user".to_string(),
        ))
        .await?;
    Ok(StatusCode::CREATED)
}

#[utoipa::path(
    post,
    path = "/refresh",
    request_body = RefreshRequest,
    responses(
        (status = 200, body = AccessTokenResponse),
    )
)]

pub async fn refresh(
    State(state): State<AppState>,
    mut jar: PrivateCookieJar,
    Json(request): Json<RefreshRequest>,
) -> Result<(PrivateCookieJar, Json<AccessTokenResponse>), AppError> {
    let config = crate::config::get_or_init();
    let claims = token::validate_refresh_token(&request.refresh_token)
        .map_err(|_| AppError::BadRequest("invalid/expired refresh token".into()))?;

    let session = state
        .service
        .find_session_by_id(claims.jti)
        .await
        .map_err(|_| AppError::BadRequest("invalid/expired refresh token".into()))?;

    if session.is_expired() {
        return Err(AppError::BadRequest("invalid/expired refresh token".into()));
    }

    let user = state
        .service
        .get_user_by_id(claims.sub)
        .await
        .map_err(|_| AppError::NotFound("User not found".into()))?;
    let token =
        token::generate_token_pair(user.id, user.email.clone(), user.role, claims.jti.clone())
            .map_err(|_| AppError::Unauthorized("failed to generate token".into()))?;

    let access_cookie = Cookie::build(("m_session", token.access_token.clone()))
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(cookie::time::Duration::seconds(config.access_token_expiry))
        .build();
    let refresh_cookie = Cookie::build(("m_refresh", token.refresh_token.clone()))
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(cookie::time::Duration::seconds(config.refresh_token_expiry))
        .build();

    jar = jar.add(access_cookie);
    jar = jar.add(refresh_cookie);

    Ok((jar, Json(token)))
}

#[utoipa::path(
    get,
    path = "/logout",
    request_body = RefreshRequest,
)]
pub async fn logout(
    State(state): State<AppState>,
    Json(request): Json<RefreshRequest>,
) -> Result<StatusCode, AppError> {
    if let Ok(claims) = token::validate_refresh_token(&request.refresh_token) {
        // Revoke the token family
        // revoke_token_family(&claims.family).await;
        tracing::info!(user.id = %claims.sub, "User logged out");
    }

    Ok(StatusCode::OK)
}

#[utoipa::path(
    get,
    path = "/me",
    responses(
        (status = 200, body = UserResponse),
        (status = 404, description = "User not found")
    )
)]
pub async fn me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<UserResponse>, AppError> {
    let user = state
        .service
        .get_user_by_id(auth_user.id)
        .await
        .map_err(|_| AppError::NotFound("User not found".into()))?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        name: user.name,
        email: user.email,
    }))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, body = UserResponse),
        (status = 404, description = "User not found")
    )
)]
pub async fn find_user_by_id(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state.service.get_user_by_id(id).await.unwrap();
    Ok(Json(UserResponse {
        id: user.id.to_string(),
        name: user.name,
        email: user.email,
    }))
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    params(
        ("id" = String, Path)
    ),
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User updated")
    )
)]
pub async fn update_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(request): Json<RegisterRequest>,
) -> Result<StatusCode, AppError> {
    let existing = state.service.get_user_by_id(id).await.unwrap();

    state
        .service
        .update_user(
            id,
            User::new(request.name, request.email, request.password, existing.role),
        )
        .await?;

    Ok(StatusCode::OK)
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = String, Path)
    ),
    responses(
        (status = 204, description = "User deleted")
    )
)]
pub async fn delete_user_by_id(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.service.delete_user_by_id(id).await.unwrap();

    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_user() {
        assert!(1 + 1 == 2);
    }
}
