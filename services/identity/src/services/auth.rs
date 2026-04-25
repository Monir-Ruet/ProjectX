use crate::state::app_state::AppState;
use crate::utils::auth::{self, create_token};
use auth_service::{AccessTokenResponse, LoginRequest, auth_server::Auth};

pub mod auth_service {
    tonic::include_proto!("projectx");
}

#[derive(Clone)]
pub struct AuthService {
    state: AppState,
}

impl AuthService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(
        &self,
        request: tonic::Request<LoginRequest>,
    ) -> Result<tonic::Response<AccessTokenResponse>, tonic::Status> {
        let req = request.into_inner();
        let user = self
            .state
            .user_repo
            .get_user_by_email(&req.username)
            .await
            .map_err(|_| tonic::Status::unauthenticated("Invalid username or password"))?;
        println!("{:?}", user);

        bcrypt::verify(&req.password, &user.password)
            .map_err(|_| tonic::Status::unauthenticated("Invalid username or password"))?;

        let claims = auth::Claims {
            sub: req.username,
            exp: 900,
        };

        println!("{:?}", "Hello from auth service");
        let secret = "your_secret_key";
        let access_token = create_token(&claims, secret);
        let refresh_token = create_token(&claims, secret);

        let reply = AccessTokenResponse {
            access_token: access_token,
            refresh_token: refresh_token,
            expire_in: 900,
            r#type: String::from("Bearer"),
        };

        Ok(tonic::Response::new(reply))
    }
}
