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

pub async fn liveness() -> &'static str {
    "Healthy"
}

// Readiness probe - can the service handle requests?
// Checks dependencies like database connections
pub async fn readiness() -> (StatusCode, Json<HealthResponse>) {
    let mut checks = Vec::new();
    let mut all_healthy = true;

    // match state.db.ping().await {
    //     Ok(_) => {
    //         checks.push(CheckResult {
    //             name: "database".to_string(),
    //             status: "healthy".to_string(),
    //             message: None,
    //         });
    //     }
    //     Err(e) => {
    //         all_healthy = false;
    //         checks.push(CheckResult {
    //             name: "database".to_string(),
    //             status: "unhealthy".to_string(),
    //             message: Some(e.to_string()),
    //         });
    //     }
    // }

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
