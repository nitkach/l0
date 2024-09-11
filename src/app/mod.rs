use crate::repository::Repository;
use anyhow::Result;
use axum::{
    debug_handler,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use error::AppError;
use http::StatusCode;
use log::info;
use model::Order;

mod error;
pub(crate) mod model;

#[debug_handler]
async fn get_orders_list(State(pool): State<Repository>) -> Result<impl IntoResponse, AppError> {
    let list = pool.list().await?;

    Ok(Json(list))
}

async fn get_order_by_id(
    State(pool): State<Repository>,
    Path(order_uid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let Some(record) = pool.get(&order_uid).await? else {
        return Err(AppError::with_status_404(anyhow::anyhow!(
            "Cannot find order with {order_uid} id."
        )));
    };

    Ok(Json(record))
}

#[debug_handler]
async fn post_order(
    State(pool): State<Repository>,
    Json(order): Json<Order>,
) -> Result<impl IntoResponse, AppError> {
    let order = crate::repository::model::OrderRecord::from(order);

    pool.add(order).await?;

    Ok(StatusCode::CREATED)
}

pub async fn run() -> Result<()> {
    let shared_state = Repository::init().await?;

    let routes = Router::new()
        .route("/", get(get_orders_list))
        .route("/:id", get(get_order_by_id))
        .route("/", post(post_order))
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
