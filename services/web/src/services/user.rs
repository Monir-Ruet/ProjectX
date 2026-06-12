use crate::models::users::signin::SignInProviderRequest;
use crate::models::users::user::UserUpdateRequest;
use crate::repositories::provider::ProviderRepository;
use crate::{
    config, error::AppError, repositories::user::UserRepository, services::Services,
    utils::password,
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use domain::entities::users::passkey::Passkey;
use domain::entities::users::provider::Provider;
use domain::entities::users::user::User;
use sqlx::Error;
use uuid::Uuid;

#[async_trait]
pub trait UserService {
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, AppError>;
    async fn get_user_by_email(&self, email: String) -> Result<User, AppError>;
    async fn add_user(&self, user: User) -> Result<(), AppError>;
    async fn delete_user_by_id(&self, id: Uuid) -> Result<(), AppError>;
    async fn update_user(&self, id: Uuid, request: UserUpdateRequest) -> Result<(), AppError>;
    async fn signin(&self, email: String, password: String) -> Result<User, AppError>;
    async fn signin_provider(&self, provider_name: String, credentials: SignInProviderRequest) -> Result<User, AppError>;
    async fn add_passkey(&self, request: Passkey) -> Result<(), AppError>;
    async fn find_all_passkeys(&self, email: String) -> Result<Vec<Passkey>, AppError>;
    async fn find_passkey(&self, cred_id: Vec<u8>) -> Result<Passkey, AppError>;
    async fn update_passkey(&self, passkey: Passkey) -> Result<(), AppError>;
}

#[async_trait]
impl UserService for Services {
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, AppError> {
        let user = self
            .repo
            .find_user_by_id(id)
            .await
            .map_err(|_| AppError::NotFound("user not found".into()))?;
        Ok(user)
    }

    async fn get_user_by_email(&self, email: String) -> Result<User, AppError> {
        let user = self
            .repo
            .find_user_by_email(email)
            .await
            .map_err(|_| AppError::NotFound("user not found".into()))?;
        Ok(user)
    }

    async fn add_user(&self, mut user: User) -> Result<(), AppError> {
        if let Some(password) = user.password {
            let hashed_password = tokio::task::spawn_blocking({
                move || password::hash_password(&password)
            })
                .await
                .map_err(|_| AppError::Internal("failed to create user".into()))?
                .map_err(|_| AppError::Internal("failed to create user".into()))?;

            user.password = Some(hashed_password);
        }
        match self.repo.add_user(user).await {
            Ok(_) => Ok(()),
            Err(Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
                Err(AppError::Conflict("already exists".into()))
            }
            Err(e) => {
                tracing::error!("Failed to create user: {:?}", e);
                Err(AppError::BadRequest("failed to create user".into()))
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

    async fn update_user(&self, id: Uuid, request: UserUpdateRequest) -> Result<(), AppError> {
        let curr_user = self
            .repo
            .find_user_by_id(id)
            .await
            .map_err(|_| AppError::NotFound("User not found".into()))?;
        let updated_user = User {
            name: request.name,
            ..curr_user
        };
        self.repo.update_user(updated_user).await.map_err(|e| {
            tracing::error!("Failed to update user {:?}", e);
            AppError::BadRequest("Failed to update user".into())
        })?;
        Ok(())
    }

    async fn signin(&self, email: String, password: String) -> Result<User, AppError> {
        let config = config::get_or_init();
        let mut user = self.repo.find_user_by_email(email).await.map_err(|e| {
            tracing::error!("Failed to find user: {:?}", e);
            AppError::Unauthorized("invalid credentials".into())
        })?;
        let locked_end_time = user
            .lockout_end
            .unwrap_or(Utc::now() - Duration::seconds(1));
        if locked_end_time > Utc::now() {
            return Err(AppError::Forbidden(
                format!(
                    "user locked for {:?} seconds",
                    (locked_end_time - Utc::now()).as_seconds_f64()
                )
                    .into(),
            ));
        }
        let password_valid = tokio::task::spawn_blocking({
            let password = password.to_string();
            let hash = user.password.clone().unwrap();
            move || password::verify_password(&password, &hash)
        })
            .await
            .map_err(|_| AppError::Unauthorized("invalid credentials".into()))?
            .map_err(|_| AppError::Unauthorized("invalid credentials".into()))?;

        if !password_valid {
            if user.last_failed_attempted != None && Utc::now() - user.last_failed_attempted.unwrap() > Duration::minutes(1) {
                user.failed_login_count = 0;
            }
            user.failed_login_count += 1;
            if user.failed_login_count == config.max_lockout_num {
                user.lockout_end = Some(Utc::now() + Duration::minutes(1));
                user.failed_login_count = 0;
            }
            self.repo
                .update_user(user)
                .await
                .map_err(|_| AppError::Internal("failed to signin".into()))?;
            return Err(AppError::Unauthorized("invalid credentials".into()));
        }
        Ok(user)
    }

    async fn signin_provider(
        &self,
        provider_name: String,
        credentials: SignInProviderRequest,
    ) -> Result<User, AppError> {
        match self
            .repo
            .find_provider_by_account_id(credentials.account_id.clone())
            .await
        {
            Ok(provider) => {
                let user = self
                    .repo
                    .find_user_by_id(provider.user_id)
                    .await
                    .map_err(|_| AppError::NotFound("User not found".into()))?;
                Ok(user)
            }
            Err(Error::RowNotFound) => {
                let mut new_user = User::new(
                    credentials.name.clone(),
                    credentials.email.clone(),
                    None,
                    Some(credentials.image),
                    String::from("user"),
                );
                new_user.email_verified = true;
                _ = self.add_user(new_user).await;
                let user = self
                    .repo
                    .find_user_by_email(credentials.email.to_string())
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to find user: {:?}", e);
                        AppError::NotFound("User not found".into())
                    })?;
                let new_provider = Provider::new(
                    provider_name.clone(),
                    credentials.account_id.clone(),
                    user.id,
                );
                self.repo.create_provider(new_provider).await.map_err(|e| {
                    tracing::error!("Failed to create provider: {:?}", e);
                    AppError::Internal("failed to create user".into())
                })?;
                Ok(user)
            }
            _ => Err(AppError::Internal("provider not found".into())),
        }
    }

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
