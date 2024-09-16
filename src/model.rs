use anyhow::Error;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct OrderRecord {
    pub(crate) order_uid: String,
    pub(crate) track_number: String,
    pub(crate) entry: String,
    pub(crate) delivery: Delivery,
    pub(crate) payment: Payment,
    pub(crate) items: Vec<Item>,
    pub(crate) locale: String,
    pub(crate) internal_signature: String,
    pub(crate) customer_id: String,
    pub(crate) delivery_service: String,
    pub(crate) shardkey: String,
    pub(crate) sm_id: i32,
    pub(crate) date_created: String,
    pub(crate) oof_shard: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Delivery {
    pub(crate) name: String,
    pub(crate) phone: String,
    pub(crate) zip: String,
    pub(crate) city: String,
    pub(crate) address: String,
    pub(crate) region: String,
    pub(crate) email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Payment {
    pub(crate) transaction: String,
    pub(crate) request_id: String,
    pub(crate) currency: String,
    pub(crate) provider: String,
    pub(crate) amount: i32,
    pub(crate) payment_dt: i64,
    pub(crate) bank: String,
    pub(crate) delivery_cost: i32,
    pub(crate) goods_total: i32,
    pub(crate) custom_fee: i32,
}

impl TryFrom<Row> for OrderRecord {
    type Error = anyhow::Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        let order = OrderRecord {
            order_uid: row.try_get("order_uid")?,
            track_number: row.try_get("track_number")?,
            entry: row.try_get("entry")?,
            delivery: Delivery {
                name: row.try_get("delivery_name")?,
                phone: row.try_get("delivery_phone")?,
                zip: row.try_get("delivery_zip")?,
                city: row.try_get("delivery_city")?,
                address: row.try_get("delivery_address")?,
                region: row.try_get("delivery_region")?,
                email: row.try_get("delivery_email")?,
            },
            payment: Payment {
                transaction: row.try_get("payment_transaction")?,
                request_id: row.try_get("payment_request_id")?,
                currency: row.try_get("payment_currency")?,
                provider: row.try_get("payment_provider")?,
                amount: row.try_get("payment_amount")?,
                payment_dt: row.try_get("payment_payment_dt")?,
                bank: row.try_get("payment_bank")?,
                delivery_cost: row.try_get("payment_delivery_cost")?,
                goods_total: row.try_get("payment_goods_total")?,
                custom_fee: row.try_get("payment_custom_fee")?,
            },
            items: Vec::new(),
            locale: row.try_get("locale")?,
            internal_signature: row.try_get("internal_signature")?,
            customer_id: row.try_get("customer_id")?,
            delivery_service: row.try_get("delivery_service")?,
            shardkey: row.try_get("shardkey")?,
            sm_id: row.try_get("sm_id")?,
            date_created: row.try_get("date_created")?,
            oof_shard: row.try_get("oof_shard")?,
        };

        Ok(order)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Item {
    pub(crate) chrt_id: i32,
    pub(crate) track_number: String,
    pub(crate) price: i32,
    pub(crate) rid: String,
    pub(crate) name: String,
    pub(crate) sale: i32,
    pub(crate) size: String,
    pub(crate) total_price: i32,
    pub(crate) nm_id: i32,
    pub(crate) brand: String,
    pub(crate) status: i32,
}

impl TryFrom<Row> for Item {
    type Error = Error;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        let item = Item {
            chrt_id: row.try_get("chrt_id")?,
            track_number: row.try_get("track_number")?,
            price: row.try_get("price")?,
            rid: row.try_get("rid")?,
            name: row.try_get("name")?,
            sale: row.try_get("sale")?,
            size: row.try_get("size")?,
            total_price: row.try_get("total_price")?,
            nm_id: row.try_get("nm_id")?,
            brand: row.try_get("brand")?,
            status: row.try_get("status")?,
        };

        Ok(item)
    }
}
