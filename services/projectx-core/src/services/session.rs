use crate::{error::AppError, repositories::session::SessionRepository, services::Services};
use async_trait::async_trait;
use chrono::Utc;
use domain::entities::users::session::Session;
use uuid::Uuid;

#[async_trait]
pub trait SessionService {
    async fn create_session(
        &self,
        user_id: Uuid,
        user_agent: String,
        ip: String,
    ) -> Result<Uuid, AppError>;
    async fn delete_session(&self, id: Uuid) -> Result<(), AppError>;
    async fn find_session_by_id(&self, id: Uuid) -> Result<Session, AppError>;
}

#[async_trait]
impl SessionService for Services {
    async fn create_session(
        &self,
        user_id: Uuid,
        user_agent: String,
        ip: String,
    ) -> Result<Uuid, AppError> {
        let config = crate::config::get_or_init();

        let session = Session::new(
            ip,
            user_agent,
            user_id,
            Utc::now() + chrono::Duration::seconds(config.refresh_token_expiry),
        );

        let session = self
            .repo
            .create_session(session)
            .await
            .map_err(|_| AppError::Internal("failed to create session".into()))?;
        Ok(session.id)
    }

    async fn delete_session(&self, id: Uuid) -> Result<(), AppError> {
        self.repo
            .delete_session_by_id(id)
            .await
            .map_err(|_| AppError::Internal("failed to delete session".into()))?;
        Ok(())
    }

    async fn find_session_by_id(&self, id: Uuid) -> Result<Session, AppError> {
        let session = self
            .repo
            .find_session_by_id(id)
            .await
            .map_err(|_| AppError::NotFound("session not found".into()))?;
        Ok(session)
    }
}
