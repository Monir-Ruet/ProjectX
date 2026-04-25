use crate::{
    services::{
        auth::{AuthService, auth_service::auth_server::AuthServer},
        user::{IdentityService, identity_service::identity_server::IdentityServer},
    },
    state::app_state::AppState,
};
use anyhow::Result;
use tonic::transport::Server;
use tracing::info;

pub mod auth;
pub mod user;

pub async fn init_services(state: AppState) -> Result<()> {
    let addr = "127.0.0.1:50051".parse()?;

    let auth = AuthService::new(state.clone());
    let user = IdentityService::new(state.clone());

    info!("Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(AuthServer::new(auth))
        .add_service(IdentityServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}
