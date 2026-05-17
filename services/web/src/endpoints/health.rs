use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    checks: Vec<CheckResult>,
}

#[derive(Serialize)]
pub struct CheckResult {
    name: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[utoipa::path(
    get,
    path = "/health/live",
    tag = "health",
    responses(
        (status = 200, description = "Service is alive")
    )
)]
pub async fn liveness() -> &'static str {
    "Healthy"
}

#[utoipa::path(
    get,
    path = "/health/ready",
    tag = "health",
    responses(
        (status = 200, description = "Service is ready")
    )
)]
pub async fn readiness() -> (StatusCode, Json<HealthResponse>) {
    let checks = Vec::new();
    let all_healthy = true;

    let status = if all_healthy { "healthy" } else { "unhealthy" };
    let code = if all_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        code,
        Json(HealthResponse {
            status: status.to_string(),
            checks,
        }),
    )
}
