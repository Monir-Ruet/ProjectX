use crate::endpoints::AppState;
use anyhow::Result;
use tower_http::trace::TraceLayer;
use tracing::info;

mod config;
mod endpoints;
mod error;
mod middlewares;
mod models;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting ProjectX Web Server on port 3000");
    let app = endpoints::routes()
        .await
        .with_state(AppState::new().await)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
