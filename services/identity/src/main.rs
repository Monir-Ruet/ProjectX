use crate::{services::init_services, state::app_state::AppState};
use anyhow::Result;
use dotenv::dotenv;

mod repositories;
mod services;
mod state;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::PgPool::connect(&database_url).await?;

    let state = AppState::new(pool);
    init_services(state).await?;

    Ok(())
}
