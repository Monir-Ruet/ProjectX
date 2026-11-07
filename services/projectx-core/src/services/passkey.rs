use crate::repositories::passkey::PasskeyRepository;
use crate::{
    error::AppError, services::Services
    ,
};
use async_trait::async_trait;
use domain::entities::users::passkey::Passkey;

#[async_trait]
pub trait PasskeyService {
    async fn add_passkey(&self, request: Passkey) -> Result<(), AppError>;
    async fn find_all_passkeys(&self, email: String) -> Result<Vec<Passkey>, AppError>;
    async fn find_passkey(&self, cred_id: Vec<u8>) -> Result<Passkey, AppError>;
    async fn update_passkey(&self, passkey: Passkey) -> Result<(), AppError>;
}

#[async_trait]
impl PasskeyService for Services {
    async fn add_passkey(&self, request: Passkey) -> Result<(), AppError> {
        self.repo.add_passkey(request).await.map_err(|e| {
            tracing::error!("Failed to add passkey: {:?}", e);
            AppError::BadRequest("failed to add passkey".into())
        })
    }

    async fn find_all_passkeys(&self, email: String) -> Result<Vec<Passkey>, AppError> {
        self.repo.find_all_passkeys(email).await.map_err(|e| {
            tracing::error!("Failed to find passkey: {:?}", e);
            AppError::NotFound("Passkey not found".into())
        })
    }

    async fn find_passkey(&self, cred_id: Vec<u8>) -> Result<Passkey, AppError> {
        self.repo.find_passkey(cred_id).await.map_err(|e| {
            tracing::error!("Failed to find passkey: {:?}", e);
            AppError::NotFound("Passkey not found".into())
        })
    }

    async fn update_passkey(&self, passkey: Passkey) -> Result<(), AppError> {
        self.repo.update_passkey(passkey).await.map_err(|e| {
            tracing::error!("Failed to update passkey: {:?}", e);
            AppError::Internal("failed to update passkey".into())
        })
    }
}