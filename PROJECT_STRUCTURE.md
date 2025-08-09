# Project Structure Overview

## 🏗️ AI-Enhanced Equitable Urban Mobility Finance Platform

**UK AI Agent Hackathon Ep2 - Stellar Bounty Submission**

```
enhanced-equitable-urban-mobility-finance-platform/
├── 📁 contracts/                          # Soroban Smart Contracts (Rust)
│   ├── 📁 loan_pool/                      # Main crowdfunding contract
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                     # Main contract logic
│   │       └── test.rs                    # Comprehensive tests
│   ├── 📁 equity_rate_adjuster/           # AI-driven rate adjustments
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                     # Equity scoring & rate logic
│   │       └── test.rs                    # Rate adjustment tests
│   ├── 📁 revenue_distributor/            # Equity-weighted revenue distribution
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                     # Revenue distribution logic
│   │       └── test.rs                    # Distribution tests
│   └── 📁 governance/                     # DAO voting with equity boosts
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs                     # Governance & voting logic
│           └── test.rs                    # Governance tests
├── 📁 src/                                # React Frontend (TypeScript)
│   ├── 📁 components/                     # React components
│   │   ├── Dashboard.tsx                  # Main impact dashboard
│   │   └── Navigation.tsx                 # Navigation component
│   ├── 📁 contexts/                       # React contexts
│   │   └── SorobanContext.tsx             # Blockchain integration
│   ├── App.tsx                            # Main app component
│   ├── main.tsx                           # App entry point
│   └── index.css                          # Tailwind CSS styles
├── 📁 scripts/                            # Automation scripts
│   ├── deploy.sh                          # Contract deployment script
│   └── test.sh                            # Testing automation
├── 📄 README.md                           # Complete project documentation
├── 📄 SUBMISSION.md                       # Hackathon submission document
├── 📄 AI_INTEGRATION.md                   # AI features documentation
├── 📄 PROJECT_STRUCTURE.md                # This file
├── 📄 Cargo.toml                          # Workspace configuration
├── 📄 package.json                        # Frontend dependencies
├── 📄 vite.config.ts                      # Vite configuration
├── 📄 tailwind.config.js                  # Tailwind CSS configuration
└── 📄 index.html                          # HTML entry point
```

## 🎯 Key Components

### 🤖 AI-Enhanced Smart Contracts

#### 1. LoanPool Contract
**Purpose**: Core crowdfunding functionality with AI-driven equity features
**Key Features**:
- Asset creation and funding management
- AI-calculated equity scores for investments
- Investor equity bonus calculations
- Asset lifecycle management (funding → deployment → completion)

**AI Integration**:
- `calculate_equity_score()`: Analyzes location data for equity scoring
- `calculate_investor_equity_bonus()`: Determines bonus percentages
- Urban data integration for underserved area identification

#### 2. EquityRateAdjuster Contract
**Purpose**: AI-driven loan rate adjustments for equitable access
**Key Features**:
- Dynamic interest rate calculations based on equity scores
- Urban data analysis (income, pollution, transport access, density)
- Loan application management with AI-adjusted rates
- Oracle integration for real-time urban data

**AI Integration**:
- `calculate_equity_score()`: Multi-factor urban data analysis
- `calculate_adjusted_rate()`: Dynamic rate adjustments
- Environmental and social impact considerations

#### 3. RevenueDistributor Contract
**Purpose**: Equity-weighted revenue distribution with impact bonuses
**Key Features**:
- Transparent revenue distribution to investors
- Equity bonus pool allocation (20% of revenue)
- Impact multipliers for environmental benefits
- CO₂ savings and underserved ride tracking

**AI Integration**:
- `calculate_equity_bonus()`: Equity-based bonus calculations
- `calculate_impact_multiplier()`: Environmental impact rewards
- Underserved area focus bonuses

#### 4. Governance Contract
**Purpose**: Community governance with equity-weighted voting
**Key Features**:
- DAO proposal creation and voting
- Equity-boosted voting power (50% boost for high-equity voters)
- Community decision-making on platform policies
- Fair representation mechanisms

**AI Integration**:
- `calculate_equity_boost()`: Voting power enhancement
- Equity score thresholds for governance participation
- Fair community representation algorithms

### 🎨 Modern Frontend Dashboard

#### React Application Structure
- **Framework**: React 18 with TypeScript
- **Styling**: Tailwind CSS with custom design system
- **Charts**: Recharts for data visualization
- **Icons**: Lucide React for modern iconography

#### Key Components
- **Dashboard**: Real-time impact metrics and analytics
- **Navigation**: Modern navigation with network status
- **SorobanContext**: Blockchain integration and state management

#### Features
- Real-time impact visualization
- Equity score tracking
- Revenue distribution analytics
- Environmental impact metrics
- Community governance overview

## 🚀 Deployment & Testing

### Smart Contract Deployment
```bash
# Set environment variables
export SOURCE_ACCOUNT="your_stellar_account"
export SECRET_KEY="your_secret_key"

# Deploy all contracts
./scripts/deploy.sh
```

### Testing
```bash
# Run all smart contract tests
./scripts/test.sh

# Run frontend tests
npm test
```

### Frontend Development
```bash
# Install dependencies
npm install

# Start development server
npm run dev
```

## 📊 AI Integration Points

### 1. Equity Scoring Algorithm
**Location**: Multiple contracts
**Purpose**: Fair assessment of investment opportunities
**Factors**:
- Income levels (1-10 scale)
- Pollution levels (1-10 scale)
- Public transport access (1-10 scale)
- Population density (1-10 scale)

### 2. Rate Adjustment Logic
**Location**: `equity_rate_adjuster/src/lib.rs`
**Purpose**: Dynamic interest rate optimization
**Features**:
- Up to 15% rate reduction for underserved areas
- Environmental impact considerations
- Social justice prioritization

### 3. Revenue Distribution
**Location**: `revenue_distributor/src/lib.rs`
**Purpose**: Equitable profit sharing
**Features**:
- 20% equity bonus pool
- Impact multipliers for environmental benefits
- Underserved area focus rewards

### 4. Governance Enhancement
**Location**: `governance/src/lib.rs`
**Purpose**: Fair community representation
**Features**:
- 50% voting power boost for high-equity participants
- Equity score thresholds
- Fair decision-making mechanisms

## 🎯 Hackathon Requirements Met

### ✅ Stellar Soroban Integration
- 4 fully functional Soroban smart contracts
- Rust-based implementation with Soroban SDK
- Futurenet deployment ready

### ✅ AI Tool Usage
- OpenZeppelin AI Wizard integration
- Stella AI prompts for code generation
- Clear documentation of AI tool usage

### ✅ Equitable Finance Focus
- AI-driven equity scoring
- Dynamic rate adjustments for underserved areas
- Transparent revenue distribution with equity bonuses
- Community governance with fair representation

### ✅ Urban Mobility Solution
- Crowdfunding for shared mobility assets
- Environmental impact tracking
- Community-driven investment decisions
- Real-time impact visualization

## 🏆 Innovation Highlights

### First-of-its-Kind Features
1. **AI integration in Soroban contracts** for equitable finance
2. **Real-time equity scoring** for urban mobility investments
3. **Automated rate adjustments** based on social impact data
4. **Equity-weighted governance** for fair community representation

### Social Impact
1. **Reduced transportation inequities** in underserved communities
2. **Increased access** to sustainable mobility solutions
3. **Fairer financial inclusion** through AI-driven adjustments
4. **Environmental justice** through pollution-based prioritization

---

**Ready for UK AI Agent Hackathon Ep2 submission! 🚀**
