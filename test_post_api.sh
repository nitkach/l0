#!/bin/bash

API_URL="http://localhost:3000"

orders=(
    '{"order_uid":"zxcv1234abcd","track_number":"TRACK123456","entry":"WBIL","delivery":{"name":"Jane Doe","phone":"+1 (111) 222-3333","zip":"54321","city":"San Francisco","address":"123 Main St","region":"CA","email":"jane.doe@example.com"},"payment":{"transaction":"zxcv1234abcd","request_id":"req_123456789","currency":"USD","provider":"paypal","amount":150,"payment_dt":1637907727,"bank":"Chase","delivery_cost":150,"goods_total":135,"custom_fee":0},"items":[{"chrt_id":9876543,"track_number":"TRACK123456","price":135,"rid":"abc123def456","name":"Gizmo","sale":0,"size":"M","total_price":135,"nm_id":4567890,"brand":"Acme","status":200}],"locale":"en","internal_signature":"","customer_id":"customer123","delivery_service":"USPS","shardkey":"13","sm_id":789,"date_created":"2022-10-01T09:00:00Z","oof_shard":"5"}'
    '{"order_uid":"qwer9876zyxw","track_number":"TRACK789012","entry":"WBIL","delivery":{"name":"John Smith","phone":"+1 (444) 555-6666","zip":"98765","city":"Seattle","address":"456 Oak St","region":"WA","email":"john.smith@example.com"},"payment":{"transaction":"qwer9876zyxw","request_id":"req_987654321","currency":"USD","provider":"visa","amount":80,"payment_dt":1637907727,"bank":"Bank of America","delivery_cost":8,"goods_total":72,"custom_fee":0},"items":[{"chrt_id":1234567,"track_number":"TRACK789012","price":72,"rid":"def456ghi789","name":"Gadget","sale":0,"size":"L","total_price":72,"nm_id":7654321,"brand":"Acme","status":200}],"locale":"en","internal_signature":"","customer_id":"customer456","delivery_service":"FedEx","shardkey":"14","sm_id":159,"date_created":"2022-11-15T12:30:00Z","oof_shard":"6"}'
    '{"order_uid":"asdf5678poiu","track_number":"TRACK246801","entry":"WBIL","delivery":{"name":"Sarah Lee","phone":"+1 (777) 888-9999","zip":"12345","city":"Chicago","address":"789 Oak Rd","region":"IL","email":"sarah.lee@example.com"},"payment":{"transaction":"asdf5678poiu","request_id":"req_246801357","currency":"USD","provider":"mastercard","amount":100,"payment_dt":1637907727,"bank":"Chase","delivery_cost":100,"goods_total":90,"custom_fee":0},"items":[{"chrt_id":3456789,"track_number":"TRACK246801","price":90,"rid":"ghi789jkl012","name":"Thingamajig","sale":0,"size":"XL","total_price":90,"nm_id":9012345,"brand":"Acme","status":200}],"locale":"en","internal_signature":"","customer_id":"customer789","delivery_service":"UPS","shardkey":"15","sm_id":246,"date_created":"2022-12-01T18:00:00Z","oof_shard":"7"}'
)

for order in "${orders[@]}"; do
    echo "Posting new order..."
    curl -X POST "$API_URL/" -H "Content-Type: application/json" -d "$order"
    echo
done
