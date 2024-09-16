use crate::repository::Repository;
use anyhow::Result;
use axum::{
    routing::{delete, get, post},
    Router,
};
use log::info;

mod error;
mod handlers;

pub async fn run() -> Result<()> {
    let shared_state = Repository::init().await?;

    let routes = Router::new()
        .route("/", get(handlers::get_orders_list))
        .route("/:id", get(handlers::get_order_by_id))
        .route("/", post(handlers::post_order))
        .route("/:id", delete(handlers::remove_order))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let (ip, port) = {
        let info = listener.local_addr().unwrap();
        (info.ip(), info.port())
    };

    info!("ip = {ip}, port = {port}. Bound IP address and port");

    axum::serve(listener, routes.into_make_service()).await?;

    Ok(())
}
