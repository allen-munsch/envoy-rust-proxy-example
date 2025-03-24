#!/bin/bash
set -e

# Colors for better output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting Envoy WASM filter deployment...${NC}"

# Step 1: Ensure wasm32 target is installed
echo -e "${GREEN}Installing wasm32-unknown-unknown target...${NC}"
rustup target add wasm32-unknown-unknown

# Step 2: Build the WASM filter
echo -e "${GREEN}Building Rust WASM filter...${NC}"
cargo build --target wasm32-unknown-unknown --release

# Step 3: Create directories
echo -e "${GREEN}Setting up directories...${NC}"
mkdir -p config target/wasm

# Step 4: Copy WASM file to the target directory
echo -e "${GREEN}Copying WASM filter to target directory...${NC}"
cp target/wasm32-unknown-unknown/release/envoy_wasm_filter.wasm target/wasm/

# Step 5: Start containers
echo -e "${GREEN}Starting Docker containers...${NC}"
docker compose up -d

# Step 6: Check if services are running
echo -e "${GREEN}Waiting for services to start...${NC}"
sleep 5
if docker compose ps | grep -q "Up"; then
  echo -e "${GREEN}Services are running!${NC}"
else
  echo -e "${YELLOW}Warning: Services might not be running correctly. Check 'docker compose logs'.${NC}"
fi

# Step 7: Test the filter
echo -e "${GREEN}Testing the filter...${NC}"
echo "Sending request to http://localhost:10000/anything"
curl -v http://localhost:10000/anything

echo -e "\n${GREEN}Deployment complete. Access your service at http://localhost:10000${NC}"
echo -e "${GREEN}Access Envoy admin interface at http://localhost:9901${NC}"
echo -e "${YELLOW}To stop the service, run: docker compose down${NC}"
