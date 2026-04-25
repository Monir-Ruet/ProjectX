use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UserQuery {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponseDto {
    pub access_token: String,
    pub refresh_token: String,
    pub expire_in: i32,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequestDto {
    pub username: String,
    pub password: String,
}
