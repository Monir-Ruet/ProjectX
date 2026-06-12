use crate::endpoints::AppState;
use crate::error::AppError;
use crate::error::AppError::Unauthorized;
use crate::extractor::auth::AuthorizedUser;
use crate::models::users::passkey::{AuthenticationStartResponse, AuthenticationState, RegisterFinishRequest, RegisterStartResponse};
use crate::models::users::signin::AccessTokenResponse;
use crate::services::session::SessionService;
use crate::services::user::UserService;
use crate::utils::cookie::set_cookie;
use crate::utils::token;
use axum::extract::{ConnectInfo, Query, State};
use axum::http::HeaderMap;
use axum::{routing::post, Json, Router};
use axum_extra::extract::PrivateCookieJar;
use domain::entities::users::passkey::Passkey;
use redis::AsyncCommands;
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tracing::error;
use utoipa::IntoParams;
use uuid::Uuid;
use webauthn_rs::prelude::{Passkey as WebauthnPasskey, PasskeyAuthentication, PasskeyRegistration};

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/passkey",
              Router::new()
                  .nest("/register", Router::new()
                      .route("/options", post(start_register))
                      .route("/verify", post(finish_register)))
                  .nest("/signin", Router::new()
                      .route("/options", post(start_authentication))
                      .route("/verify", post(finish_authentication)),
                  ))
}

#[utoipa::path(
    post,
    path = "/passkey/register/options",
    responses(
        (status = 200, description = "registration started"),
        (status = 401, description = "invalid credentials")
    )
)]
pub async fn start_register(
    State(mut state): State<AppState>,
    auth_user: AuthorizedUser,
) -> Result<Json<RegisterStartResponse>, AppError> {
    let user = state.service.get_user_by_id(auth_user.id).await?;
    let keys = state.service.find_all_passkeys(user.email.clone()).await?;
    let excluded_passkeys = keys.iter().map(|passkey| {
        let key: WebauthnPasskey = serde_json::from_value(passkey.passkey.clone()).unwrap();
        key.cred_id().clone()
    }).collect::<Vec<_>>();

    let (ccr, reg_state) = state.web_auth
        .start_passkey_registration(
            user.id,
            &user.email,
            &user.name,
            Some(excluded_passkeys),
        )
        .unwrap();
    let serialized = serde_json::to_string(&reg_state)
        .map_err(|_| AppError::Internal("failed to serialize".into()))?;

    let state_id = format!("{0}:{1}", Uuid::new_v4().to_string(), user.id.to_string());
    let key = format!("webauthn:reg:{state_id}");

    let _: () = redis::pipe()
        .set(&key, serialized)
        .expire(&key, 120)
        .query_async(&mut state.redis)
        .await
        .map_err(|_| AppError::Internal("failed to set registration state".into()))?;

    Ok(Json(RegisterStartResponse {
        options: ccr,
        state_token: state_id,
    }))
}

#[utoipa::path(
    post,
    path = "/passkey/register/verify",
    responses(
        (status = 200, description = "registration started"),
        (status = 401, description = "invalid credentials")
    )
)]
pub async fn finish_register(
    State(mut state): State<AppState>,
    _: AuthorizedUser,
    Json(req): Json<RegisterFinishRequest>,
) -> Result<(), AppError> {
    let key = format!("webauthn:reg:{0}", req.state_token);
    let stored: Option<String> = state.redis.get(&key).await
        .map_err(|_| AppError::Internal("redis error".into()))?;
    let stored = stored.ok_or_else(|| Unauthorized("registration state not found".into()))?;

    let reg_state: PasskeyRegistration = serde_json::from_str(&stored)
        .map_err(|_| AppError::Internal("invalid registration state".into()))?;
    let parts: Vec<&str> = req.state_token.split(':').collect();
    if parts.len() != 2 {
        return Err(AppError::Internal("invalid state token".into()));
    }
    let user_id = parts[1];

    let passkey = state
        .web_auth
        .finish_passkey_registration(&req.credential, &reg_state)
        .map_err(|_| AppError::Internal("failed to finish registration".into()))?;
    state.service.add_passkey(Passkey::new(user_id.parse().unwrap(), passkey.cred_id().to_vec(), json!(passkey), None)).await?;

    Ok(())
}

#[derive(Deserialize, IntoParams)]
pub struct SignInQuery {
    pub email: String,
}

#[utoipa::path(
    post,
    path = "/passkey/signin/options",
    params(
        ("email" = String, Query, description = "The registered email address of the user initiating login")
    ),
    responses(
        (status = 200, description = "registration started"),
        (status = 500, description = "invalid signin options")
    )
)]
pub async fn start_authentication(
    State(mut state): State<AppState>,
    Query(params): Query<SignInQuery>,
) -> Result<Json<AuthenticationStartResponse>, AppError> {
    let keys = state.service.find_all_passkeys(params.email).await?;
    let passkeys = keys.iter().map(|passkey| {
        let key: WebauthnPasskey = serde_json::from_value(passkey.passkey.clone()).unwrap();
        key
    }).collect::<Vec<_>>();
    if passkeys.is_empty() {
        return Err(AppError::NotFound("passkeys not found".into()));
    }

    let (rcr, auth_state) = state
        .web_auth
        .start_passkey_authentication(&passkeys).map_err(|_| AppError::Internal("failed to start passkey authentication".into()))?;
    let state_id = Uuid::new_v4().to_string();
    let key = format!("webauthn:signin:{state_id}");
    let serialized = serde_json::to_string(&auth_state)
        .map_err(|_| AppError::Internal("failed to serialize".into()))?;

    let _: () = redis::pipe()
        .set(&key, serialized)
        .expire(&key, 300)
        .query_async(&mut state.redis)
        .await
        .map_err(|_| AppError::Internal("failed to set registration state".into()))?;

    Ok(Json(AuthenticationStartResponse {
        options: rcr,
        state_token: state_id,
    }))
}

#[utoipa::path(
    post,
    path = "/passkey/signin/verify",
    responses(
        (status = 200, description = "registration started"),
        (status = 500, description = "invalid signin options")
    )
)]
pub async fn finish_authentication(
    State(mut state): State<AppState>,
    mut jar: PrivateCookieJar,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(req): Json<AuthenticationState>,
) -> Result<(PrivateCookieJar, Json<AccessTokenResponse>), AppError> {
    let key = format!("webauthn:signin:{0}", req.state_token);
    let stored: Option<String> = state.redis.get(&key).await
        .map_err(|_| AppError::Internal("redis error".into()))?;
    let stored = stored.ok_or_else(|| Unauthorized("registration state not found".into()))?;

    let auth_state: PasskeyAuthentication = serde_json::from_str(&stored)
        .map_err(|_| AppError::Internal("invalid registration state".into()))?;

    let res = state
        .web_auth
        .finish_passkey_authentication(&req.cred, &auth_state)
        .map_err(|e| {
            error!("failed to finish authentication: {}", e);
            AppError::Internal("failed to finish authentication".into())
        })?;

    let cred_id = res.cred_id().to_vec();

    let mut passkey = state
        .service
        .find_passkey(cred_id)
        .await?;

    let user = state
        .service
        .get_user_by_id(passkey.user_id)
        .await?;

    let mut saved_passkey: WebauthnPasskey =
        serde_json::from_value(passkey.passkey.clone())
            .map_err(|_| AppError::Internal("failed to deserialize passkey".into()))?;

    saved_passkey.update_credential(&res);

    passkey.passkey = serde_json::to_value(&saved_passkey)
        .map_err(|_| AppError::Internal("failed to serialize passkey".into()))?;

    state.service.update_passkey(passkey).await?;

    let _: () = state.redis.del(&key).await
        .map_err(|_| AppError::Internal("failed to delete auth state".into()))?;

    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let ip = addr.ip().to_string();

    let jti = state
        .service
        .create_session(user.id, user_agent.into(), ip)
        .await?;

    let token = token::generate_token_pair(
        user.id,
        user.email.clone(),
        user.role,
        jti,
    ).map_err(|_| Unauthorized("failed to generate token".into()))?;

    let g_id = set_cookie("g_id".into(), serde_json::to_string(&token).unwrap(), 900);
    jar = jar.add(g_id);

    Ok((jar, Json(token)))
}
