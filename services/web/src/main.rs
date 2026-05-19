use std::net::SocketAddr;

use anyhow::Result;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

mod config;
mod endpoints;
mod error;
mod extractor;
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
        .layer(ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::new())
            .layer(CompressionLayer::new().br(true).gzip(true))
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    ).await?;

    Ok(())
}
