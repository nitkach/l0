use crate::model::{Item, OrderRecord};
use anyhow::{Context, Result};
use log::{error, info};
use lru::LruCache;
use std::sync::{Arc, Mutex};
use tokio_postgres::NoTls;

#[derive(Clone)]
pub(crate) struct Repository {
    client: Arc<tokio_postgres::Client>,
    cache: Arc<Mutex<LruCache<String, OrderRecord>>>,
}

impl Repository {
    pub(crate) async fn init() -> Result<Self> {
        let db_url = std::env::var("DATABASE_URL")
            .context("Could not find environment variable DATABASE_URL")?;
        let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        // It performs the actual IO with the server, and should generally
        // be spawned off onto an executor to run in the background.
        tokio::spawn(async move {
            if let Err(err) = connection.await {
                error!("connection error: {}", err);
            }
        });

        // Least Recently Used cache
        let cache = LruCache::new(std::num::NonZeroUsize::new(10).unwrap());

        // Arc for shared state - required by axum
        Ok(Self {
            client: Arc::new(client),
            cache: Arc::new(Mutex::new(cache)),
        })
    }

    pub(crate) async fn add(&self, order: OrderRecord) -> Result<()> {
        // Rust's std::sync::Mutex cannot be held across an await point,
        // so a new scope is created here. When exiting it, cache_guard is dropped.
        {
            let mut cache_guard = self.cache.lock().unwrap();

            // check if the order is in the cache
            if cache_guard
                .push(order.order_uid.clone(), order.clone())
                .is_some()
            {
                return Ok(());
            }
            info!("Added record to cache");
        }

        let statement = self
            .client
            .prepare(
                "
                insert into orders (
                    order_uid, track_number, entry, delivery_name, delivery_phone, delivery_zip,
                    delivery_city, delivery_address, delivery_region, delivery_email,
                    payment_transaction, payment_request_id, payment_currency, payment_provider,
                    payment_amount, payment_payment_dt, payment_bank, payment_delivery_cost,
                    payment_goods_total, payment_custom_fee, locale, internal_signature,
                    customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard
                ) values (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,
                    $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28
                );
                ",
            )
            .await?;

        // binding values
        self.client
            .execute(
                &statement,
                &[
                    &order.order_uid,
                    &order.track_number,
                    &order.entry,
                    &order.delivery.name,
                    &order.delivery.phone,
                    &order.delivery.zip,
                    &order.delivery.city,
                    &order.delivery.address,
                    &order.delivery.region,
                    &order.delivery.email,
                    &order.payment.transaction,
                    &order.payment.request_id,
                    &order.payment.currency,
                    &order.payment.provider,
                    &order.payment.amount,
                    &order.payment.payment_dt,
                    &order.payment.bank,
                    &order.payment.delivery_cost,
                    &order.payment.goods_total,
                    &order.payment.custom_fee,
                    &order.locale,
                    &order.internal_signature,
                    &order.customer_id,
                    &order.delivery_service,
                    &order.shardkey,
                    &order.sm_id,
                    &order.date_created,
                    &order.oof_shard,
                ],
            )
            .await?;

        for item in &order.items {
            let item_stmt = self
                .client
                .prepare(
                    "
                    insert into orders_items (
                        order_uid, chrt_id, track_number, price, rid, name, sale, size,
                        total_price, nm_id, brand, status
                    ) values (
                        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
                    );
                    ",
                )
                .await?;

            self.client
                .execute(
                    &item_stmt,
                    &[
                        &order.order_uid,
                        &item.chrt_id,
                        &item.track_number,
                        &item.price,
                        &item.rid,
                        &item.name,
                        &item.sale,
                        &item.size,
                        &item.total_price,
                        &item.nm_id,
                        &item.brand,
                        &item.status,
                    ],
                )
                .await?;
        }

        info!("Added record to database");

        Ok(())
    }

    pub(crate) async fn get(&self, order_uid: &str) -> Result<Option<OrderRecord>> {
        {
            let mut cache_guard = self.cache.lock().unwrap();

            // check if the order is in the cache
            match cache_guard.get(order_uid) {
                Some(order) => {
                    info!("Cache hit!");
                    return Ok(Some(order.clone()));
                }
                None => {
                    info!("Cache miss, getting record from database");
                }
            }
        }

        let statement = self
            .client
            .prepare(
                "
                select
                    order_uid, track_number, entry,
                    delivery_name, delivery_phone, delivery_zip,
                    delivery_city, delivery_address, delivery_region, delivery_email,
                    payment_transaction, payment_request_id, payment_currency,
                    payment_provider, payment_amount, payment_payment_dt, payment_bank,
                    payment_delivery_cost, payment_goods_total, payment_custom_fee,
                    locale, internal_signature, customer_id, delivery_service,
                    shardkey, sm_id, date_created, oof_shard
                from orders
                where order_uid = $1;
                ",
            )
            .await?;

        let Some(row) = self.client.query_opt(&statement, &[&order_uid]).await? else {
            return Ok(None);
        };

        let record: OrderRecord = row.try_into()?;

        let statement = self
            .client
            .prepare(
                "
                select
                    chrt_id, track_number, price, rid, name, sale, size,
                    total_price, nm_id, brand, status
                from orders_items
                where order_uid = $1;
                ",
            )
            .await?;

        let items_rows = self.client.query(&statement, &[&order_uid]).await?;

        let items = items_rows
            .into_iter()
            .map(Item::try_from)
            .collect::<Result<Vec<_>>>()?;

        let order_record = OrderRecord { items, ..record };

        // put order in cache
        {
            let mut cache_guard = self.cache.lock().unwrap();

            cache_guard.put(order_uid.to_owned(), order_record.clone());
        }

        Ok(Some(order_record))
    }

    pub(crate) async fn list(&self) -> Result<Vec<OrderRecord>> {
        let statement = self
            .client
            .prepare(
                "
                select
                    order_uid, track_number, entry,
                    delivery_name, delivery_phone, delivery_zip,
                    delivery_city, delivery_address, delivery_region, delivery_email,
                    payment_transaction, payment_request_id, payment_currency,
                    payment_provider, payment_amount, payment_payment_dt, payment_bank,
                    payment_delivery_cost, payment_goods_total, payment_custom_fee,
                    locale, internal_signature, customer_id, delivery_service,
                    shardkey, sm_id, date_created, oof_shard
                from orders;
                ",
            )
            .await?;

        let orders_rows = self.client.query(&statement, &[]).await?;

        let orders = orders_rows
            .into_iter()
            .map(OrderRecord::try_from)
            .collect::<Result<Vec<_>>>()?;

        let mut list = Vec::new();
        for order in orders {
            let statement = self
                .client
                .prepare(
                    "
                    select
                        chrt_id, track_number, price, rid, name, sale, size,
                        total_price, nm_id, brand, status
                    from orders_items
                    where order_uid = $1;
                    ",
                )
                .await?;

            let items_rows = self.client.query(&statement, &[&order.order_uid]).await?;
            let items = items_rows
                .into_iter()
                .map(Item::try_from)
                .collect::<Result<Vec<_>>>()?;

            list.push(OrderRecord { items, ..order });
        }

        Ok(list)
    }

    pub(crate) async fn remove(&self, order_uid: &str) -> Result<()> {
        {
            let mut cache_guard = self.cache.lock().unwrap();

            // check if the order is in the cache
            match cache_guard.pop(order_uid) {
                Some(_) => {
                    info!("Order removed from cache");
                }
                None => {
                    info!("Order for removing not found in cache");
                }
            }
        }

        let statement = self
            .client
            .prepare(
                "
                delete from orders where order_uid = $1;
                ",
            )
            .await?;

        let rows_affected = self.client.execute(&statement, &[&order_uid]).await?;

        info!("Removed {rows_affected} rows from database");

        Ok(())
    }
}
