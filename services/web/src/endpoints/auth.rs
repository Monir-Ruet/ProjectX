use crate::{config, error::AppError};
use auth_client::{LoginRequest, auth_client::AuthClient};
use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};

mod auth_client {
    tonic::include_proto!("projectx");
}

#[derive(Debug, Deserialize)]
pub struct LoginRequestDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponseDto {
    pub access_token: String,
    pub refresh_token: String,
    pub expire_in: i32,
    pub r#type: String,
}

pub fn routes() -> Router {
    Router::new().nest("/auth", Router::new().route("/login", post(login)))
}

async fn login(Json(request): Json<LoginRequestDto>) -> Result<Json<LoginResponseDto>, AppError> {
    let grpc_req = LoginRequest {
        username: request.username,
        password: request.password,
    };
    let cfg = config::get_or_init();

    let client = AuthClient::connect(cfg.grpc_url.clone())
        .await
        .map_err(|e| AppError::GrpcConnection(format!("Failed to connect to gRPC: {}", e)))?;

    let request = tonic::Request::new(grpc_req);

    let response = client
        .clone()
        .login(request)
        .await
        .map_err(|e| AppError::GrpcCall(format!("gRPC call failed: {}", e)))?;

    let message = response.into_inner();

    let response = LoginResponseDto {
        access_token: message.access_token,
        refresh_token: message.refresh_token,
        expire_in: message.expire_in,
        r#type: message.r#type,
    };

    Ok(Json(response))
}
