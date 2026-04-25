pub mod identity_service {
    tonic::include_proto!("projectx");
}

use crate::{
    config,
    endpoints::{
        AppState,
        user::identity_service::{GUser, LoginRequest, auth_client::AuthClient},
    },
    error::Error,
    models::identity::user::{
        LoginRequestDto, LoginResponseDto, UserQuery, UserRequest, UserResponse,
    },
};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{delete, get, post, put},
};

pub fn routes() -> Router<AppState> {
    Router::new().nest(
        "/users",
        Router::new()
            .route("/", post(create_user))
            .route("/", get(find_user_by_email))
            .route("/", put(update_user))
            .route("/", delete(delete_user_by_email))
            .route("/login", post(login)),
    )
}

async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<UserRequest>,
) -> Result<(), Error> {
    state
        .identity_client
        .clone()
        .create_user(GUser {
            email: request.email.clone(),
            name: request.name.clone(),
            password: request.password.clone(),
            id: "".into(),
        })
        .await?;

    return Ok(());
}

async fn find_user_by_email(
    State(state): State<AppState>,
    Query(params): Query<UserQuery>,
) -> Result<Json<UserResponse>, Error> {
    let user = state
        .identity_client
        .clone()
        .find_user_by_email(params.email)
        .await?
        .into_inner();

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        name: user.name,
        email: user.email,
    }))
}

async fn update_user(
    State(state): State<AppState>,
    Json(request): Json<UserRequest>,
) -> Result<(), Error> {
    state
        .identity_client
        .clone()
        .update_user(GUser {
            email: request.email.clone(),
            name: request.name.clone(),
            password: request.password.clone(),
            id: "".into(),
        })
        .await?;

    return Ok(());
}

async fn delete_user_by_email(
    State(state): State<AppState>,
    Query(params): Query<UserQuery>,
) -> Result<(), Error> {
    state
        .identity_client
        .clone()
        .delete_user_by_email(params.email)
        .await?;

    return Ok(());
}

async fn login(Json(request): Json<LoginRequestDto>) -> Result<Json<LoginResponseDto>, Error> {
    let grpc_req = LoginRequest {
        username: request.username,
        password: request.password,
    };
    let cfg = config::get_or_init();
    let client = AuthClient::connect(cfg.grpc_url.clone())
        .await
        .map_err(|e| Error::GrpcConnection(format!("Failed to connect to gRPC: {}", e)))?;

    let request = tonic::Request::new(grpc_req);
    let response = client.clone().login(request).await?;
    let message = response.into_inner();
    let response = LoginResponseDto {
        access_token: message.access_token,
        refresh_token: message.refresh_token,
        expire_in: message.expire_in,
        r#type: message.r#type,
    };

    Ok(Json(response))
}
