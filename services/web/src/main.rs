use anyhow::Result;
use dapr::serde_json::json;
use scalar_api_reference::axum::router;
use tracing::info;

mod config;
mod endpoints;
mod error;
mod health;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting ProjectX Web Server on port 3000");

    let configuration = json!({
        "url": "/openapi.json",
        "theme": "mars"
    });
    let app = endpoints::routes().merge(router("/scalar", &configuration));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
