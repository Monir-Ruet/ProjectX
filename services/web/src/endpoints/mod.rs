mod doc;
mod health;
pub mod metrics;
mod user;

use crate::{
    endpoints::{
        doc::ApiDoc,
        health::{liveness, readiness},
    },
    state::AppState,
};
use axum::{Router, routing::get};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

pub async fn routes() -> Router<AppState> {
    metrics::init();

    Router::new()
        .route("/", get(|| async { "Welcome to ProjectX Web Service!" }))
        .merge(user::routes())
        .merge(metrics::metrics_router())
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .into()
}
