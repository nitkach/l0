use anyhow::{anyhow, Result};
use log::warn;

mod app;
mod repository;

pub async fn run() -> Result<()> {
    if let Err(err) = dotenvy::dotenv() {
        warn!("Failed to load .env file: {err}");
    }

    app::run().await
}
