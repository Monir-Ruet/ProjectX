use async_trait::async_trait;
use domain::entities::users::user::User;
use uuid::Uuid;

use crate::{
    error::AppError, repositories::user::UserRepository, services::Services, utils::password,
};

#[async_trait]
pub trait UserService {
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, AppError>;
    async fn create_user(&self, user: User) -> Result<(), AppError>;
    async fn delete_user_by_id(&self, id: Uuid) -> Result<(), AppError>;
    async fn update_user(&self, id: Uuid, user: User) -> Result<(), AppError>;
    async fn signin(&self, email: &str, password: &str) -> Result<User, AppError>;
}

#[async_trait]
impl UserService for Services {
    async fn signin(&self, email: &str, password: &str) -> Result<User, AppError> {
        let mut user = self
            .repo
            .find_user_by_email(email)
            .await
            .map_err(|_| AppError::Unauthorized("invalid credentials".into()))?;
        let password_valid = tokio::task::spawn_blocking({
            let password = password.to_string();
            let hash = user.password.unwrap();
            move || password::verify_password(&password, &hash)
        })
        .await
        .map_err(|_| AppError::Unauthorized("invalid credentials".into()))?
        .map_err(|_| AppError::Unauthorized("invalid credentials".into()))?;

        if !password_valid {
            return Err(AppError::Unauthorized("invalid credentials".into()));
        }
        user.password = None;
        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<User, AppError> {
        let user = self.repo.find_user_by_id(id).await;
        match user {
            Ok(user) => Ok(user),
            Err(_) => Err(AppError::NotFound("user not found".into())),
        }
    }

    async fn create_user(&self, mut user: User) -> Result<(), AppError> {
        let hashed_password = tokio::task::spawn_blocking({
            let password = user.password.as_ref().unwrap().to_string();
            move || password::hash_password(&password)
        })
        .await
        .map_err(|_| AppError::Internal("failed to create user".into()))?
        .map_err(|_| AppError::Internal("failed to create user".into()))?;

        user.password = Some(hashed_password);
        match self.repo.create_user(user).await {
            Ok(_) => return Ok(()),
            Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
                tracing::warn!("Attempt to create user with existing email");
                Err(AppError::Conflict("already exists".into()))
            }
            Err(e) => {
                tracing::error!("Failed to create user: {:?}", e);
                return Err(AppError::BadRequest("failed to create user".into()));
            }
        }
    }

    async fn delete_user_by_id(&self, id: Uuid) -> Result<(), AppError> {
        self.repo
            .find_user_by_id(id)
            .await
            .map_err(|_| AppError::NotFound("User not found".into()))?;
        self.repo.delete_user_by_id(id).await.map_err(|e| {
            tracing::error!("Failed to delete user with id: {}: {:?}", id, e);
            AppError::Internal("Failed to delete user".into())
        })?;
        Ok(())
    }

    async fn update_user(&self, id: Uuid, user: User) -> Result<(), AppError> {
        self.repo.update_user(id, user).await.map_err(|e| {
            tracing::error!("Failed to update user {:?}", e);
            AppError::BadRequest("Failed to update user".into())
        })?;
        Ok(())
    }
}
