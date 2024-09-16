#!/bin/bash

API_URL="http://localhost:3000"

echo "Listing all orders..."
curl -X GET "$API_URL/"
echo
