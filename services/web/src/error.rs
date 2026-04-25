use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    GrpcConnection(String),
    GrpcCall(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::GrpcConnection(e) => (StatusCode::SERVICE_UNAVAILABLE, e),
            AppError::GrpcCall(e) => (StatusCode::BAD_GATEWAY, e),
        };

        let body = Json(ErrorResponse {
            error: error_message,
        });

        (status, body).into_response()
    }
}
