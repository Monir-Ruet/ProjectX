use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::Value;

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
