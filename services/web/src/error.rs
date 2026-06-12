use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug)]
pub enum AppError {
    NotFound(Value),
    BadRequest(Value),
    Internal(Value),
    Conflict(Value),
    Unauthorized(Value),
    Forbidden(Value),
}

#[derive(Serialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub problem_type: String,
    pub title: String,
    pub status: u16,
    pub detail: Option<Value>,
    pub instance: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, title, detail) = match self {
            AppError::NotFound(detail) => (StatusCode::NOT_FOUND, "Not Found", Some(detail)),

            AppError::BadRequest(detail) => (StatusCode::BAD_REQUEST, "Bad Request", Some(detail)),

            AppError::Internal(detail) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                Some(detail),
            ),

            AppError::Conflict(detail) => (StatusCode::CONFLICT, "Conflict", Some(detail)),

            AppError::Unauthorized(detail) => {
                (StatusCode::UNAUTHORIZED, "Unauthorized", Some(detail))
            }

            AppError::Forbidden(detail) => (StatusCode::FORBIDDEN, "Forbidden", Some(detail)),
        };

        let body = ProblemDetails {
            problem_type: "https://tools.ietf.org/html/rfc9110#section-15.5.5".to_string(),
            title: title.to_string(),
            status: status.as_u16(),
            detail,
            instance: None,
        };

        (status, Json(body)).into_response()
    }
}


#[derive(Error, Debug)]
pub enum WebauthnError {
    #[error("unknown webauthn error")]
    Unknown,
    #[error("Corrupt Session")]
    CorruptSession,
    #[error("User Not Found")]
    UserNotFound,
    #[error("User Has No Credentials")]
    UserHasNoCredentials,
    #[error("Deserialising Session failed: {0}")]
    InvalidSessionState(#[from] tower_sessions::session::Error),
}
impl IntoResponse for WebauthnError {
    fn into_response(self) -> Response {
        let body = match self {
            WebauthnError::CorruptSession => "Corrupt Session",
            WebauthnError::UserNotFound => "User Not Found",
            WebauthnError::Unknown => "Unknown Error",
            WebauthnError::UserHasNoCredentials => "User Has No Credentials",
            WebauthnError::InvalidSessionState(_) => "Deserialising Session failed",
        };

        // its often easiest to implement `IntoResponse` by calling other implementations
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
