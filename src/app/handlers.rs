use crate::{app::error::AppError, repository::Repository};
use anyhow::Result;
use axum::{
    debug_handler,
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use log::{error, info};

#[debug_handler]
pub(crate) async fn get_orders_list(
    State(pool): State<Repository>,
) -> Result<impl IntoResponse, AppError> {
    info!("Getting list of all orders");

    let list = pool.list().await?;

    info!("Found {} orders.", list.len());

    Ok(Json(list))
}

pub(crate) async fn get_order_by_id(
    State(pool): State<Repository>,
    Path(order_uid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    info!("Getting order with uid: {order_uid}");

    let Some(record) = pool.get(&order_uid).await? else {
        info!("Cannot find order with uid: {order_uid}");
        return Err(AppError::with_status_404(anyhow::anyhow!(
            "Cannot find order with uid: '{order_uid}'"
        )));
    };

    Ok(Json(record))
}

#[debug_handler]
pub(crate) async fn post_order(
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

pub(crate) async fn remove_order(
    State(pool): State<Repository>,
    Path(order_uid): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    pool.remove(&order_uid).await?;

    Ok(StatusCode::OK)
}
