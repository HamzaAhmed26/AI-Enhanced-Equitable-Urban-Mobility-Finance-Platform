#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, vec, Address, Env, Map, Symbol, Vec,
};

/// Represents a mobility asset that can be funded
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MobilityAsset {
    pub id: Symbol,
    pub name: Symbol,
    pub asset_type: Symbol, // "e-bike", "shuttle", "scooter"
    pub target_amount: i128,
    pub funded_amount: i128,
    pub location: Symbol, // City/zone identifier
    pub equity_score: i32, // AI-calculated equity score (0-100)
    pub status: Symbol, // "funding", "funded", "deployed", "completed"
    pub investors: Vec<Address>,
    pub created_at: u64,
}

/// Represents an investor's contribution
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Investment {
    pub investor: Address,
    pub asset_id: Symbol,
    pub amount: i128,
    pub equity_bonus: i32, // AI-calculated equity bonus percentage
    pub timestamp: u64,
}

/// Contract data structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataKey {
    pub admin: Address,
    pub assets: Map<Symbol, MobilityAsset>,
    pub investments: Vec<Investment>,
    pub total_pool_balance: i128,
    pub equity_oracle: Address, // AI oracle address for equity calculations
}

const DATA_KEY: Symbol = symbol_short!("DATA_KEY");

#[contract]
pub struct LoanPool;

#[contractimpl]
impl LoanPool {
    /// Initialize the contract with admin and AI oracle
    pub fn initialize(env: &Env, admin: Address, equity_oracle: Address) {
        let data = DataKey {
            admin,
            assets: Map::new(env),
            investments: vec![env],
            total_pool_balance: 0,
            equity_oracle,
        };
        env.storage().instance().set(&DATA_KEY, &data);
    }

    /// Create a new mobility asset for funding
    pub fn create_asset(
        env: &Env,
        asset_id: Symbol,
        name: Symbol,
        asset_type: Symbol,
        target_amount: i128,
        location: Symbol,
    ) -> Result<(), Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only admin can create assets
        if env.current_contract_address() != data.admin {
            return Err(symbol_short!("UNAUTHORIZED"));
        }

        // Check if asset already exists
        if data.assets.contains_key(&asset_id) {
            return Err(symbol_short!("ASSET_EXISTS"));
        }

        // Calculate equity score using AI oracle (mocked for demo)
        let equity_score = Self::calculate_equity_score(env, &location);

        let asset = MobilityAsset {
            id: asset_id.clone(),
            name,
            asset_type,
            target_amount,
            funded_amount: 0,
            location,
            equity_score,
            status: symbol_short!("funding"),
            investors: vec![env],
            created_at: env.ledger().timestamp(),
        };

        data.assets.set(&asset_id, &asset);
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(())
    }

    /// Invest in a mobility asset with AI-adjusted equity bonuses
    pub fn invest(
        env: &Env,
        investor: Address,
        asset_id: Symbol,
        amount: i128,
    ) -> Result<i32, Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Validate amount
        if amount <= 0 {
            return Err(symbol_short!("INVALID_AMOUNT"));
        }

        // Get asset
        let mut asset = data.assets.get(&asset_id).ok_or(symbol_short!("ASSET_NOT_FOUND"))?;
        
        // Check if asset is still funding
        if asset.status != symbol_short!("funding") {
            return Err(symbol_short!("ASSET_NOT_FUNDING"));
        }

        // Calculate equity bonus based on investor and location
        let equity_bonus = Self::calculate_investor_equity_bonus(env, &investor, &asset.location);

        // Create investment record
        let investment = Investment {
            investor: investor.clone(),
            asset_id: asset_id.clone(),
            amount,
            equity_bonus,
            timestamp: env.ledger().timestamp(),
        };

        // Update asset
        asset.funded_amount += amount;
        asset.investors.push_back(&investor);

        // Check if funding target reached
        if asset.funded_amount >= asset.target_amount {
            asset.status = symbol_short!("funded");
        }

        // Update data
        data.assets.set(&asset_id, &asset);
        data.investments.push_back(&investment);
        data.total_pool_balance += amount;
        
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(equity_bonus)
    }

    /// Get asset details
    pub fn get_asset(env: &Env, asset_id: Symbol) -> Result<MobilityAsset, Symbol> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        data.assets.get(&asset_id).ok_or(symbol_short!("ASSET_NOT_FOUND"))
    }

    /// Get all assets
    pub fn get_all_assets(env: &Env) -> Vec<MobilityAsset> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        let mut assets = vec![env];
        
        for (_, asset) in data.assets.iter() {
            assets.push_back(&asset);
        }
        
        assets
    }

    /// Get investments for an asset
    pub fn get_asset_investments(env: &Env, asset_id: Symbol) -> Vec<Investment> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        let mut asset_investments = vec![env];
        
        for investment in data.investments.iter() {
            if investment.asset_id == asset_id {
                asset_investments.push_back(&investment);
            }
        }
        
        asset_investments
    }

    /// Get total pool balance
    pub fn get_pool_balance(env: &Env) -> i128 {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        data.total_pool_balance
    }

    /// AI-driven equity score calculation (mocked for demo)
    fn calculate_equity_score(env: &Env, location: &Symbol) -> i32 {
        // In a real implementation, this would call the AI oracle
        // For demo purposes, we'll use a simple algorithm based on location hash
        let location_str = location.to_string();
        let hash = env.crypto().sha256(&location_str.as_bytes());
        
        // Convert first byte to equity score (0-100)
        let score = (hash[0] as i32) % 101;
        
        // Ensure underserved areas get higher equity scores
        if location_str.contains("low_income") || location_str.contains("underserved") {
            score + 20
        } else {
            score
        }
    }

    /// Calculate investor equity bonus based on location and investment history
    fn calculate_investor_equity_bonus(env: &Env, investor: &Address, location: &Symbol) -> i32 {
        // In a real implementation, this would analyze:
        // - Investor's location (lower income areas get higher bonuses)
        // - Investment history (first-time investors get bonuses)
        // - Community impact metrics
        
        let investor_str = investor.to_string();
        let location_str = location.to_string();
        
        let mut bonus = 0;
        
        // Bonus for underserved area investments
        if location_str.contains("low_income") || location_str.contains("underserved") {
            bonus += 15;
        }
        
        // Bonus for first-time investors (simplified check)
        if investor_str.len() < 10 {
            bonus += 10;
        }
        
        // Cap bonus at 25%
        if bonus > 25 {
            bonus = 25;
        }
        
        bonus
    }

    /// Deploy a funded asset (admin only)
    pub fn deploy_asset(env: &Env, asset_id: Symbol) -> Result<(), Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only admin can deploy assets
        if env.current_contract_address() != data.admin {
            return Err(symbol_short!("UNAUTHORIZED"));
        }

        let mut asset = data.assets.get(&asset_id).ok_or(symbol_short!("ASSET_NOT_FOUND"))?;
        
        if asset.status != symbol_short!("funded") {
            return Err(symbol_short!("ASSET_NOT_FUNDED"));
        }

        asset.status = symbol_short!("deployed");
        data.assets.set(&asset_id, &asset);
        
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(())
    }

    /// Complete an asset (admin only) - triggers revenue distribution
    pub fn complete_asset(env: &Env, asset_id: Symbol) -> Result<(), Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only admin can complete assets
        if env.current_contract_address() != data.admin {
            return Err(symbol_short!("UNAUTHORIZED"));
        }

        let mut asset = data.assets.get(&asset_id).ok_or(symbol_short!("ASSET_NOT_FOUND"))?;
        
        if asset.status != symbol_short!("deployed") {
            return Err(symbol_short!("ASSET_NOT_DEPLOYED"));
        }

        asset.status = symbol_short!("completed");
        data.assets.set(&asset_id, &asset);
        
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(())
    }
}

#[cfg(test)]
mod test;
