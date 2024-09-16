#!/bin/bash

API_URL="http://localhost:3000"

orders_uids=(
  'b563feb7b2b84b6test'
  'zxcv1234abcd'
  'qwer9876zyxw'
  'asdf5678poiu'
)

for order_uid in "${orders_uids[@]}"; do
    echo "Deleting order with uid: $order_uid..."
    curl -X DELETE "$API_URL/$order_uid"
    echo
done
