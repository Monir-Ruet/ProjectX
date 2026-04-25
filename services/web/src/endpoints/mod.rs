mod auth;
mod user;

use axum::{Router, routing::get};

use crate::health::{liveness, readiness};

pub fn routes() -> Router {
    Router::new()
        // .merge(user::routes())
        .merge(auth::routes())
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
}
