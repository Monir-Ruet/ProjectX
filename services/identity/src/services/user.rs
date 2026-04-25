use crate::utils::error::map_error;
use crate::{repositories::user::User, state::app_state::AppState};
use identity_service::{GUser, identity_server::Identity};

pub mod identity_service {
    tonic::include_proto!("projectx");
}

#[derive(Clone)]
pub struct IdentityService {
    state: AppState,
}

impl IdentityService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl Identity for IdentityService {
    async fn find_user_by_email(
        &self,
        request: tonic::Request<::prost::alloc::string::String>,
    ) -> std::result::Result<tonic::Response<GUser>, tonic::Status> {
        let req = request.into_inner();
        let user = self
            .state
            .user_repo
            .get_user_by_email(&req)
            .await
            .map(|user| {
                tonic::Response::new(GUser {
                    id: user
                        .id
                        .clone()
                        .unwrap_or_else(|| uuid::Uuid::max().to_string()),
                    name: user.name.clone(),
                    email: user.email.clone(),
                    password: user.password.clone(),
                })
            })
            .map_err(|_| tonic::Status::not_found("User not found"))?;

        Ok(user)
    }

    async fn create_user(
        &self,
        request: tonic::Request<GUser>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();

        self.state
            .user_repo
            .create_user(User::new(req.name, req.email, req.password))
            .await
            .map_err(map_error)?;

        Ok(tonic::Response::new(()))
    }

    async fn update_user(
        &self,
        request: tonic::Request<GUser>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();
        self.state
            .user_repo
            .update_user(req.id, User::new(req.name, req.email, req.password))
            .await
            .map_err(map_error)?;
        Ok(tonic::Response::new(()))
    }

    async fn delete_user_by_email(
        &self,
        request: tonic::Request<::prost::alloc::string::String>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let req = request.into_inner();
        self.state
            .user_repo
            .delete_user_by_email(&req)
            .await
            .map_err(map_error)?;
        Ok(tonic::Response::new(()))
    }
}
