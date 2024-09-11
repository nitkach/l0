#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct Delivery {
    pub(crate) name: String,
    pub(crate) phone: String,
    pub(crate) zip: String,
    pub(crate) city: String,
    pub(crate) address: String,
    pub(crate) region: String,
    pub(crate) email: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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

impl From<crate::app::model::Order> for OrderRecord {
    fn from(value: crate::app::model::Order) -> Self {
        Self {
            order_uid: value.order_uid,
            track_number: value.track_number,
            entry: value.entry,
            delivery: Delivery {
                name: value.delivery.name,
                phone: value.delivery.phone,
                zip: value.delivery.zip,
                city: value.delivery.city,
                address: value.delivery.address,
                region: value.delivery.region,
                email: value.delivery.email,
            },
            payment: Payment {
                transaction: value.payment.transaction,
                request_id: value.payment.request_id,
                currency: value.payment.currency,
                provider: value.payment.provider,
                amount: value.payment.amount,
                payment_dt: value.payment.payment_dt,
                bank: value.payment.bank,
                delivery_cost: value.payment.delivery_cost,
                goods_total: value.payment.goods_total,
                custom_fee: value.payment.custom_fee,
            },
            items: value
                .items
                .into_iter()
                .map(|item| Item {
                    chrt_id: item.chrt_id,
                    track_number: item.track_number,
                    price: item.price,
                    rid: item.rid,
                    name: item.name,
                    sale: item.sale,
                    size: item.size,
                    total_price: item.total_price,
                    nm_id: item.nm_id,
                    brand: item.brand,
                    status: item.status,
                })
                .collect(),
            locale: value.locale,
            internal_signature: value.internal_signature,
            customer_id: value.customer_id,
            delivery_service: value.delivery_service,
            shardkey: value.shardkey,
            sm_id: value.sm_id,
            date_created: value.date_created,
            oof_shard: value.oof_shard,
        }
    }
}
