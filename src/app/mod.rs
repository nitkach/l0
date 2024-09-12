use crate::repository::Repository;
use anyhow::Result;
use axum::{
    debug_handler,
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use error::AppError;
use http::StatusCode;
use log::{error, info};

mod error;

#[debug_handler]
async fn get_orders_list(State(pool): State<Repository>) -> Result<impl IntoResponse, AppError> {
    info!("Getting list of all orders");

    let list = pool.list().await?;

    Ok(Json(list))
}

async fn get_order_by_id(
    State(pool): State<Repository>,
    Path(order_uid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!("Getting order with uid: {order_uid}");

    let Some(record) = pool.get(&order_uid).await? else {
        info!("Cannot find order with uid: {order_uid}");
        return Err(AppError::with_status_404(anyhow::anyhow!(
            "Cannot find order with {order_uid} id."
        )));
    };

    Ok(Json(record))
}

#[debug_handler]
async fn post_order(
    State(pool): State<Repository>,
    body: String,
) -> Result<impl IntoResponse, AppError> {
    let order = match serde_json::from_str(&body) {
        Ok(string) => string,
        Err(err) => {
            error!("Failed to deserialize to JSON");
            return Err(err.into());
        }
    };

    pool.add(order).await?;

    Ok(StatusCode::CREATED)
}

async fn remove_order(
    State(pool): State<Repository>,
    Path(order_uid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    pool.remove(&order_uid).await?;

    Ok(StatusCode::OK)
}

pub async fn run() -> Result<()> {
    let shared_state = Repository::init().await?;

    let routes = Router::new()
        .route("/", get(get_orders_list))
        .route("/:id", get(get_order_by_id))
        .route("/", post(post_order))
        .route("/:id", delete(remove_order))
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
