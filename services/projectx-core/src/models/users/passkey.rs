use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::{CreationChallengeResponse, PublicKeyCredential, RegisterPublicKeyCredential, RequestChallengeResponse};

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