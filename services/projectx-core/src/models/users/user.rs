use chrono::{DateTime, Utc};
use domain::entities::users::user::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct UserUpdateRequest {
    pub name: String,
}

impl From<UserUpdateRequest> for User {
    fn from(user: UserUpdateRequest) -> User {
        User {
            name: user.name,
            ..User::default()
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub image: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.to_string(),
            name: user.name.to_string(),
            email: user.email.to_string(),
            role: user.role.to_string(),
            image: user.image,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
