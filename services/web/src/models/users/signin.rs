use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct PassKeySignInRequest {
    pub id: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct PassKeySignInResponse {
    pub passkey_token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct PassKeyVerifyRequest {
    pub passkey_token: String,
}


#[derive(Deserialize, ToSchema)]
pub struct SignInProviderRequest {
    pub email: String,
    pub name: String,
    pub account_id: String,
    pub image: String,
}

#[derive(Serialize, ToSchema)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}
