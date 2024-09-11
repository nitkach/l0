use anyhow::{anyhow, Result};

mod app;
mod repository;

pub async fn run() -> Result<()> {
    if let Err(err) = dotenvy::dotenv() {
        return Err(anyhow!(err));
    }

    app::run().await
}
