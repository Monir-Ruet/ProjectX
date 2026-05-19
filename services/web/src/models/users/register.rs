use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;
use domain::entities::users::user::User;

#[derive(Deserialize, ToSchema, Validate)]
pub struct RegisterRequest {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Name must be between 3 and 50 characters"
    ))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, max = 100, message = "Password must be at least 6 characters"))]
    pub password: String,
}

impl From<RegisterRequest> for User {
    fn from(req: RegisterRequest) -> User {
        User{
            name: req.name.into(),
            email: req.email.into(),
            password: req.password.into(),
            role: "user".into(),
            ..User::default()
        }
    }
}