use chrono::Utc;
use domain::entities::users::passkey::Passkey;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct ChallengeRequest {
    pub user_id: Uuid,
    pub challenge: String,
}

#[derive(Deserialize)]
pub struct ChallengeQuery {
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct PasskeyQuery {
    pub credential_id: String,
}

#[derive(Deserialize, ToSchema)]
pub struct PassKeyRequest {
    pub credential_id: String,
    pub public_key: String,
    pub counter: i64,
    pub device_type: Option<String>,
    pub backed_up: bool,
    pub transports: Option<String>,
    pub user_id: Uuid,
}

impl From<PassKeyRequest> for Passkey {
    fn from(request: PassKeyRequest) -> Self {
        Passkey {
            id: Default::default(),
            credential_id: request.credential_id,
            public_key: request.public_key,
            counter: request.counter,
            device_type: request.device_type,
            backed_up: request.backed_up,
            transports: request.transports,
            user_id: request.user_id,
            created_at: Utc::now(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct PasskeyResponse {
    pub id: Uuid,
    pub credential_id: String,
    pub public_key: String,
    pub counter: i64,
    pub device_type: Option<String>,
    pub backed_up: bool,
    pub transports: Option<String>,
    pub user_id: Uuid,
}

impl From<Passkey> for PasskeyResponse {
    fn from(passkey: Passkey) -> Self {
        PasskeyResponse {
            id: passkey.id,
            credential_id: passkey.credential_id,
            public_key: passkey.public_key,
            counter: passkey.counter,
            device_type: passkey.device_type,
            backed_up: passkey.backed_up,
            transports: passkey.transports,
            user_id: passkey.user_id,
        }
    }
}