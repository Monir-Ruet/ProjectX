use serde::{Deserialize, Serialize};
use uuid::Uuid;
use webauthn_rs::prelude::{CreationChallengeResponse, PasskeyAuthentication, PublicKeyCredential, RegisterPublicKeyCredential, RequestChallengeResponse};

#[derive(Deserialize)]
pub struct RegisterFinishRequest {
    pub credential: RegisterPublicKeyCredential,
    pub state_token: String,
}

#[derive(Serialize)]
pub struct RegisterStartResponse {
    pub options: CreationChallengeResponse,
    pub state_token: String,
}

#[derive(Serialize)]
pub struct AuthenticationStartResponse {
    pub options: RequestChallengeResponse,
    pub state_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationState {
    pub state_token: String,
    pub cred: PublicKeyCredential,
}

pub struct LoginFinishRequest {
    pub user_id: Uuid,
    pub credential: serde_json::Value,
}