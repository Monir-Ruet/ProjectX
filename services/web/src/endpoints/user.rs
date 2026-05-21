use crate::extractor::auth::AuthorizedUser;
use crate::extractor::role::{Admin, AppUser, RequireRole};
use crate::models::users::refresh::RefreshRequest;
use crate::models::users::register::RegisterRequest;
use crate::models::users::signin::{AccessTokenResponse, SignInProviderRequest, SignInRequest};
use crate::models::users::user::{UserResponse, UserUpdateRequest};
use crate::{
    endpoints::AppState,
    error::AppError,
    extractor::validated::Validated,
    services::{session::SessionService, user::UserService},
    utils::token::{self},
};
use axum::extract::Path;
use axum::routing::{delete, put};
use axum::{
    Json, Router,
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use std::net::SocketAddr;
use uuid::Uuid;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register/", post(register_user))
        .route("/users/{id}", get(find_user_by_id))
        .route("/register", post(register_user))
        .route("/signin", post(signin))
        .route("/signin/{provider_name}", post(signin_provider))
        .route("/refresh", post(refresh))
        .route("/sign_out", post(sign_out))
        .route("/me", get(me))
        .route("/me", put(update_me))
        .route("/me", delete(delete_me))
        .route("/ping", get(is_authenticated))
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
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(request): Json<SignInRequest>,
) -> Result<(StatusCode, Json<AccessTokenResponse>), AppError> {
    let user = state
        .service
        .signin(request.email.to_string(), request.password.to_string())
        .await?;

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

    Ok((StatusCode::OK, Json(token)))
}

#[utoipa::path(
    post,
    path = "/signin/{provider_name}",
    request_body = SignInProviderRequest,
    responses(
        (status = 200, description = "signed in"),
        (status = 401, description = "invalid credentials")
    )
)]
pub async fn signin_provider(
    State(state): State<AppState>,
    Path(provider_name): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(request): Json<SignInProviderRequest>,
) -> Result<(StatusCode, Json<AccessTokenResponse>), AppError> {
    let provider = provider_name.as_str();
    match provider {
        "google" | "github" => Ok(()),
        _ => Err(AppError::Unauthorized("provider not supported".into())),
    }?;
    let user = state
        .service
        .signin_provider(provider.to_string(), request)
        .await?;
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

    Ok((StatusCode::OK, Json(token)))
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User created")
    )
)]
pub async fn register_user(
    State(state): State<AppState>,
    Validated(request): Validated<RegisterRequest>,
) -> Result<StatusCode, AppError> {
    state.service.create_user(request.into()).await?;
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
    Json(request): Json<RefreshRequest>,
) -> Result<(StatusCode, Json<AccessTokenResponse>), AppError> {
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

    Ok((StatusCode::OK, Json(token)))
}

#[utoipa::path(
    post,
    path = "/sign_out",
    responses(
        (status = 200, description = "Session logged out"),
    )
)]
pub async fn sign_out(
    State(state): State<AppState>,
    RequireRole::<Admin>(user, _): RequireRole<Admin>,
) -> Result<StatusCode, AppError> {
    state.service.delete_session(user.jti).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    put,
    path = "/me",
    request_body = UserUpdateRequest,
    responses(
        (status = 200, body = ()),
        (status = 404, description = "User not found"),
        (status = 500, description = "Failed to update user")
    )
)]
pub async fn update_me(
    State(state): State<AppState>,
    RequireRole::<AppUser>(user, _): RequireRole<AppUser>,
    Json(request): Json<UserUpdateRequest>,
) -> Result<StatusCode, AppError> {
    state.service.update_user(user.id, request.into()).await?;
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
    auth_user: AuthorizedUser,
    RequireRole::<AppUser>(_, _): RequireRole<AppUser>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state.service.get_user_by_id(auth_user.id).await?;

    Ok(Json(user.into()))
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
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state.service.get_user_by_id(id).await?;
    Ok(Json(user.into()))
}

#[utoipa::path(
    delete,
    path = "/me",
    params(
        ("id" = String, Path)
    ),
    responses(
        (status = 204, description = "User deleted")
    )
)]
pub async fn delete_me(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.service.delete_user_by_id(id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = 200, description = "authorized"),
        (status = 401, description = "unauthorized access")
    )
)]
pub async fn is_authenticated(_: AuthorizedUser) -> Result<StatusCode, AppError> {
    Ok(StatusCode::OK)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_user() {
        assert_eq!(1 + 1, 2);
    }
}
