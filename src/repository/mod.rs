use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use log::{error, info};
use model::{Item, OrderRecord};
use tokio_postgres::NoTls;

pub(crate) mod model;

#[derive(Clone)]
pub(crate) struct Repository {
    client: Arc<tokio_postgres::Client>,
    cache: Arc<Mutex<HashMap<String, OrderRecord>>>,
}

impl Repository {
    pub(crate) async fn init() -> Result<Self> {
        let db_url = std::env::var("DATABASE_URL")?;
        let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                error!("connection error: {}", err);
            }
        });

        Ok(Self {
            client: Arc::new(client),
            cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub(crate) async fn add(&self, order: OrderRecord) -> Result<()> {
        // Rust's std::sync::Mutex cannot be held across an await point,
        // so a new scope is created here. When exiting it, cache_guard is dropped.
        {
            let mut cache_guard = self.cache.lock().unwrap();

            // check if the order is in the cache
            match cache_guard.entry(order.order_uid.clone()) {
                std::collections::hash_map::Entry::Occupied(_) => {
                    return Ok(());
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(order.clone());
                    info!("Added record to cache");
                }
            };
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

        Ok(())
    }

    pub(crate) async fn get(&self, order_uid: &str) -> Result<Option<OrderRecord>> {
        {
            let mut cache_guard = self.cache.lock().unwrap();

            // check if the order is in the cache
            match cache_guard.entry(order_uid.to_owned()) {
                std::collections::hash_map::Entry::Occupied(occupied) => {
                    info!("Cache hit!");
                    return Ok(Some(occupied.get().clone()));
                }
                std::collections::hash_map::Entry::Vacant(_) => {
                    info!("Cache miss, getting record from database");
                }
            };
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

        let record = OrderRecord {
            order_uid: row.get("order_uid"),
            track_number: row.get("track_number"),
            entry: row.get("entry"),
            delivery: model::Delivery {
                name: row.get("delivery_name"),
                phone: row.get("delivery_phone"),
                zip: row.get("delivery_zip"),
                city: row.get("delivery_city"),
                address: row.get("delivery_address"),
                region: row.get("delivery_region"),
                email: row.get("delivery_email"),
            },
            payment: model::Payment {
                transaction: row.get("payment_transaction"),
                request_id: row.get("payment_request_id"),
                currency: row.get("payment_currency"),
                provider: row.get("payment_provider"),
                amount: row.get("payment_amount"),
                payment_dt: row.get("payment_payment_dt"),
                bank: row.get("payment_bank"),
                delivery_cost: row.get("payment_delivery_cost"),
                goods_total: row.get("payment_goods_total"),
                custom_fee: row.get("payment_custom_fee"),
            },
            items: Vec::new(),
            locale: row.get("locale"),
            internal_signature: row.get("internal_signature"),
            customer_id: row.get("customer_id"),
            delivery_service: row.get("delivery_service"),
            shardkey: row.get("shardkey"),
            sm_id: row.get("sm_id"),
            date_created: row.get("date_created"),
            oof_shard: row.get("oof_shard"),
        };

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
            .map(|item_row| Item {
                chrt_id: item_row.get("chrt_id"),
                track_number: item_row.get("track_number"),
                price: item_row.get("price"),
                rid: item_row.get("rid"),
                name: item_row.get("name"),
                sale: item_row.get("sale"),
                size: item_row.get("size"),
                total_price: item_row.get("total_price"),
                nm_id: item_row.get("nm_id"),
                brand: item_row.get("brand"),
                status: item_row.get("status"),
            })
            .collect::<Vec<_>>();

        Ok(Some(OrderRecord { items, ..record }))
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
            .map(|row| OrderRecord {
                order_uid: row.get("order_uid"),
                track_number: row.get("track_number"),
                entry: row.get("entry"),
                delivery: model::Delivery {
                    name: row.get("delivery_name"),
                    phone: row.get("delivery_phone"),
                    zip: row.get("delivery_zip"),
                    city: row.get("delivery_city"),
                    address: row.get("delivery_address"),
                    region: row.get("delivery_region"),
                    email: row.get("delivery_email"),
                },
                payment: model::Payment {
                    transaction: row.get("payment_transaction"),
                    request_id: row.get("payment_request_id"),
                    currency: row.get("payment_currency"),
                    provider: row.get("payment_provider"),
                    amount: row.get("payment_amount"),
                    payment_dt: row.get("payment_payment_dt"),
                    bank: row.get("payment_bank"),
                    delivery_cost: row.get("payment_delivery_cost"),
                    goods_total: row.get("payment_goods_total"),
                    custom_fee: row.get("payment_custom_fee"),
                },
                items: Vec::new(),
                locale: row.get("locale"),
                internal_signature: row.get("internal_signature"),
                customer_id: row.get("customer_id"),
                delivery_service: row.get("delivery_service"),
                shardkey: row.get("shardkey"),
                sm_id: row.get("sm_id"),
                date_created: row.get("date_created"),
                oof_shard: row.get("oof_shard"),
            })
            .collect::<Vec<OrderRecord>>();

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
                .map(|item_row| Item {
                    chrt_id: item_row.get("chrt_id"),
                    track_number: item_row.get("track_number"),
                    price: item_row.get("price"),
                    rid: item_row.get("rid"),
                    name: item_row.get("name"),
                    sale: item_row.get("sale"),
                    size: item_row.get("size"),
                    total_price: item_row.get("total_price"),
                    nm_id: item_row.get("nm_id"),
                    brand: item_row.get("brand"),
                    status: item_row.get("status"),
                })
                .collect::<Vec<_>>();

            list.push(OrderRecord { items, ..order });
        }

        Ok(list)
    }
}
