use std::net::SocketAddr;

use anyhow::Result;
use tower_http::trace::TraceLayer;
use tracing::info;

mod config;
mod endpoints;
mod error;
mod extractor;
mod middlewares;
mod models;
mod repositories;
mod services;
mod state;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    info!("Starting ProjectX Web Server on port 3000");

    let state = state::AppState::new().await?;

    let app = endpoints::routes()
        .await
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
