create table if not exists orders (
                order_uid varchar(255) primary key,
             track_number varchar(255) not null,
                    entry varchar(50)  not null,
            delivery_name varchar(255) not null,
           delivery_phone varchar(20)  not null,
             delivery_zip varchar(20)  not null,
            delivery_city varchar(100) not null,
         delivery_address varchar(255) not null,
          delivery_region varchar(100) not null,
           delivery_email varchar(255) not null,
      payment_transaction varchar(255) not null,
       payment_request_id varchar(255) not null,
         payment_currency varchar(10)  not null,
         payment_provider varchar(50)  not null,
           payment_amount int          not null,
       payment_payment_dt bigint       not null,
             payment_bank varchar(50)  not null,
    payment_delivery_cost int          not null,
      payment_goods_total int          not null,
       payment_custom_fee int          not null,
                   locale varchar(10)  not null,
       internal_signature varchar(255) not null,
              customer_id varchar(255) not null,
         delivery_service varchar(100) not null,
                 shardkey varchar(10)  not null,
                    sm_id int          not null,
             date_created varchar(50)  not null,
                oof_shard varchar(10)  not null
);

create table if not exists orders_items (
       order_uid varchar(255) references orders(order_uid) on delete cascade not null,
         chrt_id int          not null,
    track_number varchar(255) not null,
           price int          not null,
             rid varchar(255) not null,
            name varchar(255) not null,
            sale int          not null,
            size varchar(10)  not null,
     total_price int          not null,
           nm_id int          not null,
           brand varchar(255) not null,
          status int          not null
);
