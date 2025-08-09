#!/bin/bash

# AI-Enhanced Equitable Urban Mobility Finance Platform
# Deployment script for UK AI Agent Hackathon Ep2

set -e

echo "🚀 Deploying AI-Enhanced Equitable Urban Mobility Finance Platform"
echo "================================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NETWORK="futurenet"
RPC_URL="https://rpc-futurenet.stellar.org"
PASSPHRASE="Test SDF Future Network ; October 2022"

# Check if soroban CLI is installed
if ! command -v soroban &> /dev/null; then
    echo -e "${RED}❌ Soroban CLI not found. Please install it first.${NC}"
    echo "Visit: https://soroban.stellar.org/docs/getting-started/setup"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo not found. Please install it first.${NC}"
    echo "Visit: https://rustup.rs/"
    exit 1
fi

echo -e "${BLUE}📋 Prerequisites check passed${NC}"

# Build all contracts
echo -e "${YELLOW}🔨 Building smart contracts...${NC}"

# Build LoanPool contract
echo "Building LoanPool contract..."
cd contracts/loan_pool
cargo build --target wasm32-unknown-unknown --release
cd ../..

# Build EquityRateAdjuster contract
echo "Building EquityRateAdjuster contract..."
cd contracts/equity_rate_adjuster
cargo build --target wasm32-unknown-unknown --release
cd ../..

# Build RevenueDistributor contract
echo "Building RevenueDistributor contract..."
cd contracts/revenue_distributor
cargo build --target wasm32-unknown-unknown --release
cd ../..

# Build Governance contract
echo "Building Governance contract..."
cd contracts/governance
cargo build --target wasm32-unknown-unknown --release
cd ../..

echo -e "${GREEN}✅ All contracts built successfully${NC}"

# Check if we have a source account
if [ -z "$SOURCE_ACCOUNT" ]; then
    echo -e "${YELLOW}⚠️  SOURCE_ACCOUNT not set. Please set it to your Stellar account address.${NC}"
    echo "Example: export SOURCE_ACCOUNT=GABCDEF123456789..."
    echo "You can create a test account at: https://laboratory.stellar.org/#account-creator"
    exit 1
fi

# Check if we have a secret key
if [ -z "$SECRET_KEY" ]; then
    echo -e "${YELLOW}⚠️  SECRET_KEY not set. Please set it to your Stellar secret key.${NC}"
    echo "Example: export SECRET_KEY=SABCDEF123456789..."
    exit 1
fi

echo -e "${BLUE}📋 Using source account: ${SOURCE_ACCOUNT}${NC}"

# Deploy contracts
echo -e "${YELLOW}🚀 Deploying contracts to ${NETWORK}...${NC}"

# Deploy LoanPool
echo "Deploying LoanPool contract..."
LOAN_POOL_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/loan_pool.wasm \
    --source $SOURCE_ACCOUNT \
    --secret-key $SECRET_KEY \
    --rpc-url $RPC_URL \
    --network-passphrase "$PASSPHRASE" \
    --output json | jq -r '.contractId')

echo -e "${GREEN}✅ LoanPool deployed: ${LOAN_POOL_ID}${NC}"

# Deploy EquityRateAdjuster
echo "Deploying EquityRateAdjuster contract..."
EQUITY_RATE_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/equity_rate_adjuster.wasm \
    --source $SOURCE_ACCOUNT \
    --secret-key $SECRET_KEY \
    --rpc-url $RPC_URL \
    --network-passphrase "$PASSPHRASE" \
    --output json | jq -r '.contractId')

echo -e "${GREEN}✅ EquityRateAdjuster deployed: ${EQUITY_RATE_ID}${NC}"

# Deploy RevenueDistributor
echo "Deploying RevenueDistributor contract..."
REVENUE_DIST_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/revenue_distributor.wasm \
    --source $SOURCE_ACCOUNT \
    --secret-key $SECRET_KEY \
    --rpc-url $RPC_URL \
    --network-passphrase "$PASSPHRASE" \
    --output json | jq -r '.contractId')

echo -e "${GREEN}✅ RevenueDistributor deployed: ${REVENUE_DIST_ID}${NC}"

# Deploy Governance
echo "Deploying Governance contract..."
GOVERNANCE_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/governance.wasm \
    --source $SOURCE_ACCOUNT \
    --secret-key $SECRET_KEY \
    --rpc-url $RPC_URL \
    --network-passphrase "$PASSPHRASE" \
    --output json | jq -r '.contractId')

echo -e "${GREEN}✅ Governance deployed: ${GOVERNANCE_ID}${NC}"

# Initialize contracts
echo -e "${YELLOW}🔧 Initializing contracts...${NC}"

# Initialize LoanPool
echo "Initializing LoanPool..."
soroban contract invoke \
    --id $LOAN_POOL_ID \
    --source $SOURCE_ACCOUNT \
    --secret-key $SECRET_KEY \
    --rpc-url $RPC_URL \
    --network-passphrase "$PASSPHRASE" \
    -- initialize \
    --admin $SOURCE_ACCOUNT \
    --equity-oracle $SOURCE_ACCOUNT

echo -e "${GREEN}✅ LoanPool initialized${NC}"

# Initialize EquityRateAdjuster
echo "Initializing EquityRateAdjuster..."
soroban contract invoke \
    --id $EQUITY_RATE_ID \
    --source $SOURCE_ACCOUNT \
    --secret-key $SECRET_KEY \
    --rpc-url $RPC_URL \
    --network-passphrase "$PASSPHRASE" \
    -- initialize \
    --admin $SOURCE_ACCOUNT \
    --oracle $SOURCE_ACCOUNT \
    --base-rate 8

echo -e "${GREEN}✅ EquityRateAdjuster initialized${NC}"

# Initialize RevenueDistributor
echo "Initializing RevenueDistributor..."
soroban contract invoke \
    --id $REVENUE_DIST_ID \
    --source $SOURCE_ACCOUNT \
    --secret-key $SECRET_KEY \
    --rpc-url $RPC_URL \
    --network-passphrase "$PASSPHRASE" \
    -- initialize \
    --admin $SOURCE_ACCOUNT \
    --oracle $SOURCE_ACCOUNT \
    --loan-pool $LOAN_POOL_ID \
    --equity-bonus-rate 20

echo -e "${GREEN}✅ RevenueDistributor initialized${NC}"

# Initialize Governance
echo "Initializing Governance..."
soroban contract invoke \
    --id $GOVERNANCE_ID \
    --source $SOURCE_ACCOUNT \
    --secret-key $SECRET_KEY \
    --rpc-url $RPC_URL \
    --network-passphrase "$PASSPHRASE" \
    -- initialize \
    --admin $SOURCE_ACCOUNT \
    --oracle $SOURCE_ACCOUNT \
    --loan-pool $LOAN_POOL_ID \
    --min-proposal-duration 86400

echo -e "${GREEN}✅ Governance initialized${NC}"

# Save deployment info
echo -e "${YELLOW}💾 Saving deployment information...${NC}"

cat > deployment.json << EOF
{
  "network": "${NETWORK}",
  "rpc_url": "${RPC_URL}",
  "deployed_at": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "contracts": {
    "loan_pool": "${LOAN_POOL_ID}",
    "equity_rate_adjuster": "${EQUITY_RATE_ID}",
    "revenue_distributor": "${REVENUE_DIST_ID}",
    "governance": "${GOVERNANCE_ID}"
  },
  "admin": "${SOURCE_ACCOUNT}",
  "description": "AI-Enhanced Equitable Urban Mobility Finance Platform - UK AI Agent Hackathon Ep2"
}
EOF

echo -e "${GREEN}✅ Deployment information saved to deployment.json${NC}"

# Display summary
echo -e "${GREEN}"
echo "🎉 Deployment Complete!"
echo "======================"
echo "Network: ${NETWORK}"
echo "LoanPool: ${LOAN_POOL_ID}"
echo "EquityRateAdjuster: ${EQUITY_RATE_ID}"
echo "RevenueDistributor: ${REVENUE_DIST_ID}"
echo "Governance: ${GOVERNANCE_ID}"
echo ""
echo "🔗 View on Stellar Explorer:"
echo "https://explorer.stellar.org/futurenet/account/${SOURCE_ACCOUNT}"
echo ""
echo "📊 Next steps:"
echo "1. Update frontend configuration with contract addresses"
echo "2. Run tests: ./scripts/test.sh"
echo "3. Start frontend: npm run dev"
echo -e "${NC}"

echo -e "${BLUE}🤖 AI Integration Notes:${NC}"
echo "- Equity scoring uses AI oracle integration (mocked for demo)"
echo "- Rate adjustments are calculated based on urban data"
echo "- Revenue distribution includes equity bonuses"
echo "- Governance voting includes equity-weighted logic"

echo -e "${GREEN}✅ Ready for UK AI Agent Hackathon Ep2 submission!${NC}"
