mod doc;
mod health;
pub mod metrics;
mod user;
pub mod passkey;

use crate::{
    endpoints::{
        doc::ApiDoc,
        health::{liveness, readiness},
    },
    state::AppState,
};
use axum::{routing::get, Router};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

pub async fn routes() -> Router<AppState> {
    metrics::init();

    Router::new()
        .route("/", get(|| async { "Welcome to ProjectX Web Service!" }))
        .merge(user::routes())
        .merge(metrics::metrics_router())
        .merge(passkey::routes())
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .into()
}
