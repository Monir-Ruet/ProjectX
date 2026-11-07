use anyhow::Result;
use axum::routing::get;
use std::net::SocketAddr;
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
pub mod socket;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    info!("Starting ProjectX Web Server on port 8080");

    let state = state::AppState::new().await?;

    let app = endpoints::routes()
        .await
        .route("/ws", get(socket::ws_handler))
        .with_state(state).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::new())
            .layer(CompressionLayer::new().br(true).gzip(true)),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .await?;

    Ok(())
}
