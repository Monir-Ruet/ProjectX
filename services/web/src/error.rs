use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tonic::{Code, Status};

#[derive(Debug)]
pub enum Error {
    Grpc(Status),
    GrpcConnection(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    message: String,
}

impl From<Status> for Error {
    fn from(status: Status) -> Self {
        Error::Grpc(status)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::Grpc(status) => map_grpc_status(&status),
            Error::GrpcConnection(msg) => (StatusCode::BAD_GATEWAY, msg),
        };

        let body = Json(ErrorResponse {
            success: false,
            message,
        });

        (status, body).into_response()
    }
}

fn map_grpc_status(status: &Status) -> (StatusCode, String) {
    match status.code() {
        Code::AlreadyExists => (StatusCode::CONFLICT, "Already exists".to_string()),
        Code::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
        Code::InvalidArgument => (StatusCode::BAD_REQUEST, status.message().to_string()),
        Code::Unauthenticated => (
            StatusCode::UNAUTHORIZED,
            "Authentication required".to_string(),
        ),
        Code::PermissionDenied => (StatusCode::FORBIDDEN, "Permission denied".to_string()),
        Code::Unavailable => (
            StatusCode::SERVICE_UNAVAILABLE,
            "Service unavailable".to_string(),
        ),
        Code::DeadlineExceeded => (StatusCode::GATEWAY_TIMEOUT, "Request timeout".to_string()),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        ),
    }
}
