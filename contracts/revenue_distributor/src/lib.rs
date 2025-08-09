#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, vec, Address, Env, Map, Symbol, Vec,
};

/// Represents a revenue distribution event
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevenueDistribution {
    pub id: Symbol,
    pub asset_id: Symbol,
    pub total_revenue: i128,
    pub distribution_amount: i128,
    pub equity_bonus_pool: i128,
    pub timestamp: u64,
    pub distributions: Vec<InvestorDistribution>,
}

/// Represents an investor's revenue distribution
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvestorDistribution {
    pub investor: Address,
    pub base_amount: i128,
    pub equity_bonus: i128,
    pub total_amount: i128,
    pub equity_score: i32,
    pub impact_multiplier: i32, // Multiplier based on social impact
}

/// Represents ride revenue data from oracle
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RideRevenue {
    pub asset_id: Symbol,
    pub revenue_amount: i128,
    pub ride_count: i32,
    pub co2_saved: i32, // CO2 saved in kg
    pub underserved_rides: i32, // Rides in underserved areas
    pub timestamp: u64,
}

/// Contract data structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataKey {
    pub admin: Address,
    pub oracle: Address, // Revenue oracle address
    pub loan_pool: Address, // Loan pool contract address
    pub distributions: Map<Symbol, RevenueDistribution>,
    pub ride_revenues: Map<Symbol, RideRevenue>,
    pub equity_bonus_rate: i32, // Percentage of revenue for equity bonuses
    pub impact_bonus_rate: i32, // Additional bonus for high-impact zones
}

const DATA_KEY: Symbol = symbol_short!("DATA_KEY");

#[contract]
pub struct RevenueDistributor;

#[contractimpl]
impl RevenueDistributor {
    /// Initialize the contract
    pub fn initialize(
        env: &Env,
        admin: Address,
        oracle: Address,
        loan_pool: Address,
        equity_bonus_rate: i32,
    ) {
        let data = DataKey {
            admin,
            oracle,
            loan_pool,
            distributions: Map::new(env),
            ride_revenues: Map::new(env),
            equity_bonus_rate,
            impact_bonus_rate: 10, // 10% additional bonus for high-impact zones
        };
        env.storage().instance().set(&DATA_KEY, &data);
    }

    /// Record ride revenue from oracle
    pub fn record_revenue(
        env: &Env,
        asset_id: Symbol,
        revenue_amount: i128,
        ride_count: i32,
        co2_saved: i32,
        underserved_rides: i32,
    ) -> Result<(), Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only oracle can record revenue
        if env.current_contract_address() != data.oracle {
            return Err(symbol_short!("UNAUTHORIZED"));
        }

        let revenue = RideRevenue {
            asset_id: asset_id.clone(),
            revenue_amount,
            ride_count,
            co2_saved,
            underserved_rides,
            timestamp: env.ledger().timestamp(),
        };

        data.ride_revenues.set(&asset_id, &revenue);
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(())
    }

    /// Distribute revenue to investors with equity bonuses
    pub fn distribute_revenue(
        env: &Env,
        asset_id: Symbol,
        investors: Vec<Address>,
        investment_amounts: Vec<i128>,
        equity_scores: Vec<i32>,
    ) -> Result<Symbol, Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only admin can trigger distribution
        if env.current_contract_address() != data.admin {
            return Err(symbol_short!("UNAUTHORIZED"));
        }

        // Get revenue data
        let revenue = data.ride_revenues.get(&asset_id).ok_or(symbol_short!("REVENUE_NOT_FOUND"))?;
        
        // Validate input arrays
        if investors.len() != investment_amounts.len() || investors.len() != equity_scores.len() {
            return Err(symbol_short!("INVALID_INPUT"));
        }

        let total_investment = investment_amounts.iter().sum();
        let equity_bonus_pool = revenue.revenue_amount * data.equity_bonus_rate / 100;
        let distribution_amount = revenue.revenue_amount - equity_bonus_pool;

        let mut distributions = vec![env];
        let mut total_distributed = 0;

        // Calculate distributions for each investor
        for i in 0..investors.len() {
            let investor = &investors.get(i).unwrap();
            let investment_amount = investment_amounts.get(i).unwrap();
            let equity_score = equity_scores.get(i).unwrap();

            // Calculate base distribution proportional to investment
            let base_amount = if total_investment > 0 {
                distribution_amount * investment_amount / total_investment
            } else {
                0
            };

            // Calculate equity bonus
            let equity_bonus = Self::calculate_equity_bonus(
                env,
                &equity_bonus_pool,
                &equity_score,
                &revenue.underserved_rides,
                &revenue.ride_count,
            );

            // Calculate impact multiplier for high-impact zones
            let impact_multiplier = Self::calculate_impact_multiplier(
                env,
                &revenue.co2_saved,
                &revenue.underserved_rides,
                &revenue.ride_count,
            );

            let total_amount = base_amount + equity_bonus;
            total_distributed += total_amount;

            let distribution = InvestorDistribution {
                investor: investor.clone(),
                base_amount,
                equity_bonus,
                total_amount,
                equity_score: equity_score.clone(),
                impact_multiplier,
            };

            distributions.push_back(&distribution);
        }

        // Create distribution record
        let distribution_id = Self::generate_distribution_id(env, &asset_id);
        let distribution = RevenueDistribution {
            id: distribution_id.clone(),
            asset_id,
            total_revenue: revenue.revenue_amount,
            distribution_amount,
            equity_bonus_pool,
            timestamp: env.ledger().timestamp(),
            distributions,
        };

        data.distributions.set(&distribution_id, &distribution);
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(distribution_id)
    }

    /// Get distribution details
    pub fn get_distribution(env: &Env, distribution_id: Symbol) -> Result<RevenueDistribution, Symbol> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        data.distributions.get(&distribution_id).ok_or(symbol_short!("DISTRIBUTION_NOT_FOUND"))
    }

    /// Get revenue data for an asset
    pub fn get_revenue(env: &Env, asset_id: Symbol) -> Result<RideRevenue, Symbol> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        data.ride_revenues.get(&asset_id).ok_or(symbol_short!("REVENUE_NOT_FOUND"))
    }

    /// Get all distributions for an asset
    pub fn get_asset_distributions(env: &Env, asset_id: Symbol) -> Vec<RevenueDistribution> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        let mut asset_distributions = vec![env];
        
        for (_, distribution) in data.distributions.iter() {
            if distribution.asset_id == asset_id {
                asset_distributions.push_back(&distribution);
            }
        }
        
        asset_distributions
    }

    /// Calculate total impact metrics
    pub fn get_impact_metrics(env: &Env) -> (i32, i32, i32) {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        let mut total_co2_saved = 0;
        let mut total_rides = 0;
        let mut total_underserved_rides = 0;
        
        for (_, revenue) in data.ride_revenues.iter() {
            total_co2_saved += revenue.co2_saved;
            total_rides += revenue.ride_count;
            total_underserved_rides += revenue.underserved_rides;
        }
        
        (total_co2_saved, total_rides, total_underserved_rides)
    }

    /// Update equity bonus rate (admin only)
    pub fn update_equity_bonus_rate(env: &Env, new_rate: i32) -> Result<(), Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only admin can update rates
        if env.current_contract_address() != data.admin {
            return Err(symbol_short!("UNAUTHORIZED"));
        }

        // Validate rate (0-50%)
        if new_rate < 0 || new_rate > 50 {
            return Err(symbol_short!("INVALID_RATE"));
        }

        data.equity_bonus_rate = new_rate;
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(())
    }

    /// Update impact bonus rate (admin only)
    pub fn update_impact_bonus_rate(env: &Env, new_rate: i32) -> Result<(), Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only admin can update rates
        if env.current_contract_address() != data.admin {
            return Err(symbol_short!("UNAUTHORIZED"));
        }

        // Validate rate (0-25%)
        if new_rate < 0 || new_rate > 25 {
            return Err(symbol_short!("INVALID_RATE"));
        }

        data.impact_bonus_rate = new_rate;
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(())
    }

    /// Generate unique distribution ID
    fn generate_distribution_id(env: &Env, asset_id: &Symbol) -> Symbol {
        let asset_str = asset_id.to_string();
        let timestamp = env.ledger().timestamp();
        
        let combined = format!("dist_{}_{}", asset_str, timestamp);
        let hash = env.crypto().sha256(&combined.as_bytes());
        
        // Convert first 8 bytes to symbol
        let mut id_bytes = [0u8; 8];
        id_bytes.copy_from_slice(&hash[0..8]);
        
        Symbol::from_bytes(&id_bytes)
    }

    /// Calculate equity bonus for an investor
    fn calculate_equity_bonus(
        env: &Env,
        equity_bonus_pool: &i128,
        equity_score: &i32,
        underserved_rides: &i32,
        total_rides: &i32,
    ) -> i128 {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Base equity bonus based on equity score
        let base_bonus = equity_bonus_pool * equity_score / 100;
        
        // Additional bonus for underserved area focus
        let underserved_bonus = if total_rides > 0 {
            let underserved_ratio = underserved_rides * 100 / total_rides;
            base_bonus * underserved_ratio / 100
        } else {
            0
        };
        
        base_bonus + underserved_bonus
    }

    /// Calculate impact multiplier for high-impact zones
    fn calculate_impact_multiplier(
        env: &Env,
        co2_saved: &i32,
        underserved_rides: &i32,
        total_rides: &i32,
    ) -> i32 {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        let mut multiplier = 100; // Base 100%
        
        // Bonus for CO2 savings
        if co2_saved > 1000 {
            multiplier += 10; // 10% bonus for significant CO2 savings
        }
        
        // Bonus for underserved area focus
        if total_rides > 0 {
            let underserved_ratio = underserved_rides * 100 / total_rides;
            if underserved_ratio > 50 {
                multiplier += data.impact_bonus_rate; // Additional bonus for high underserved ratio
            }
        }
        
        multiplier
    }

    /// Get distribution statistics
    pub fn get_stats(env: &Env) -> (i32, i128, i32) {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        let mut total_distributions = 0;
        let mut total_revenue_distributed = 0;
        let mut total_assets = 0;
        
        for (_, distribution) in data.distributions.iter() {
            total_distributions += 1;
            total_revenue_distributed += distribution.total_revenue;
        }
        
        for (_, _) in data.ride_revenues.iter() {
            total_assets += 1;
        }
        
        (total_distributions, total_revenue_distributed, total_assets)
    }
}

#[cfg(test)]
mod test;
