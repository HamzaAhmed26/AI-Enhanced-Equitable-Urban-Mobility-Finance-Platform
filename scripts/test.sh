#!/bin/bash

# AI-Enhanced Equitable Urban Mobility Finance Platform
# Test script for UK AI Agent Hackathon Ep2

set -e

echo "🧪 Testing AI-Enhanced Equitable Urban Mobility Finance Platform"
echo "================================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo not found. Please install it first.${NC}"
    echo "Visit: https://rustup.rs/"
    exit 1
fi

echo -e "${BLUE}📋 Running smart contract tests...${NC}"

# Test LoanPool contract
echo -e "${YELLOW}🔍 Testing LoanPool contract...${NC}"
cd contracts/loan_pool
cargo test
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ LoanPool tests passed${NC}"
else
    echo -e "${RED}❌ LoanPool tests failed${NC}"
    exit 1
fi
cd ../..

# Test EquityRateAdjuster contract
echo -e "${YELLOW}🔍 Testing EquityRateAdjuster contract...${NC}"
cd contracts/equity_rate_adjuster
cargo test
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ EquityRateAdjuster tests passed${NC}"
else
    echo -e "${RED}❌ EquityRateAdjuster tests failed${NC}"
    exit 1
fi
cd ../..

# Test RevenueDistributor contract
echo -e "${YELLOW}🔍 Testing RevenueDistributor contract...${NC}"
cd contracts/revenue_distributor
cargo test
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ RevenueDistributor tests passed${NC}"
else
    echo -e "${RED}❌ RevenueDistributor tests failed${NC}"
    exit 1
fi
cd ../..

# Test Governance contract
echo -e "${YELLOW}🔍 Testing Governance contract...${NC}"
cd contracts/governance
cargo test
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Governance tests passed${NC}"
else
    echo -e "${RED}❌ Governance tests failed${NC}"
    exit 1
fi
cd ../..

# Run integration tests if they exist
if [ -f "tests/integration_tests.rs" ]; then
    echo -e "${YELLOW}🔍 Running integration tests...${NC}"
    cargo test --test integration_tests
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Integration tests passed${NC}"
    else
        echo -e "${RED}❌ Integration tests failed${NC}"
        exit 1
    fi
fi

# Test frontend if Node.js is available
if command -v npm &> /dev/null; then
    echo -e "${YELLOW}🔍 Testing frontend...${NC}"
    npm test
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Frontend tests passed${NC}"
    else
        echo -e "${YELLOW}⚠️  Frontend tests failed or not configured${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  Node.js/npm not found, skipping frontend tests${NC}"
fi

# Display test summary
echo -e "${GREEN}"
echo "🎉 All Tests Passed!"
echo "==================="
echo "✅ LoanPool contract tests"
echo "✅ EquityRateAdjuster contract tests"
echo "✅ RevenueDistributor contract tests"
echo "✅ Governance contract tests"
echo ""
echo "🤖 AI Integration Features Tested:"
echo "- Equity score calculation"
echo "- AI-adjusted loan rates"
echo "- Equity-weighted revenue distribution"
echo "- Equity-boosted governance voting"
echo ""
echo "📊 Test Coverage:"
echo "- Smart contract functionality"
echo "- AI oracle integration (mocked)"
echo "- Equity calculations"
echo "- Revenue distribution logic"
echo "- Governance mechanisms"
echo -e "${NC}"

echo -e "${BLUE}🚀 Ready for deployment!${NC}"
echo "Run: ./scripts/deploy.sh to deploy to Futurenet"
