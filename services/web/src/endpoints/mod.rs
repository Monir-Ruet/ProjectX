mod health;
mod user;

use crate::endpoints::{
    health::{liveness, readiness},
    user::identity_service::identity_client::IdentityClient,
};
use axum::{Router, routing::get};
use tonic::transport::Channel;

#[derive(Clone)]
pub struct AppState {
    identity_client: IdentityClient<Channel>,
}

impl AppState {
    pub async fn new() -> Self {
        let cfg = crate::config::get_or_init();
        let identity_grpc_client = IdentityClient::connect(cfg.grpc_url.clone())
            .await
            .expect("Failed to connect to gRPC service");
        AppState {
            identity_client: identity_grpc_client,
        }
    }
}

pub async fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Welcome to ProjectX Web Service!" }))
        .merge(user::routes())
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
        .into()
}
