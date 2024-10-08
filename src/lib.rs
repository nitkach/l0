use anyhow::Result;
use log::warn;

mod app;
mod model;
mod repository;

pub async fn run() -> Result<()> {
    if let Err(err) = dotenvy::dotenv() {
        warn!("Failed to load .env file: {err}");
    }

    app::run().await
}
