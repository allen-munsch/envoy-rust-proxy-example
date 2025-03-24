#!/bin/bash
set -e

# Colors for better output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${GREEN}Testing the Envoy WASM filter...${NC}"

# Test 1: Basic request
echo -e "\n${BLUE}Test 1: Basic Request${NC}"
echo "Sending request to http://localhost:10000/get"
curl -s http://localhost:10000/get | jq .

# Test 2: Check for custom headers
echo -e "\n${BLUE}Test 2: Check for custom headers${NC}"
echo "Sending request and inspecting headers:"
curl -s -v http://localhost:10000/headers 2>&1 | grep -E "x-wasm-|x-response-metadata|x-envoy-upstream-rq-timeout-ms"

# Test 3: Response metadata
echo -e "\n${BLUE}Test 3: View full response with headers${NC}"
echo "Detailed response info:"
curl -s -D - http://localhost:10000/anything | grep -v "date:"

# Test 4: Check Envoy stats for our filter
echo -e "\n${BLUE}Test 4: Checking Envoy stats${NC}"
curl -s http://localhost:9901/stats | grep wasm

echo -e "\n${GREEN}Tests completed!${NC}"
