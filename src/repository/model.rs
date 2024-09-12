use serde::{Deserialize, Serialize};

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
