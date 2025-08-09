#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, vec, Address, Env, Map, Symbol, Vec,
};

/// Represents urban data used for AI-driven rate adjustments
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UrbanData {
    pub location: Symbol,
    pub income_level: i32, // 1-10 scale (1 = lowest income)
    pub pollution_level: i32, // 1-10 scale (1 = lowest pollution)
    pub public_transport_score: i32, // 1-10 scale (1 = poorest access)
    pub population_density: i32, // 1-10 scale (1 = lowest density)
    pub timestamp: u64,
}

/// Represents a loan application with AI-adjusted rates
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoanApplication {
    pub id: Symbol,
    pub borrower: Address,
    pub asset_id: Symbol,
    pub requested_amount: i128,
    pub base_rate: i32, // Base interest rate (percentage)
    pub adjusted_rate: i32, // AI-adjusted rate (percentage)
    pub equity_score: i32, // AI-calculated equity score (0-100)
    pub urban_data: UrbanData,
    pub status: Symbol, // "pending", "approved", "rejected", "active", "completed"
    pub created_at: u64,
}

/// Contract data structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataKey {
    pub admin: Address,
    pub oracle: Address, // AI oracle address
    pub applications: Map<Symbol, LoanApplication>,
    pub urban_data_cache: Map<Symbol, UrbanData>,
    pub base_rate: i32, // Default base rate (percentage)
    pub max_rate_adjustment: i32, // Maximum rate adjustment (percentage)
}

const DATA_KEY: Symbol = symbol_short!("DATA_KEY");

#[contract]
pub struct EquityRateAdjuster;

#[contractimpl]
impl EquityRateAdjuster {
    /// Initialize the contract with admin and AI oracle
    pub fn initialize(env: &Env, admin: Address, oracle: Address, base_rate: i32) {
        let data = DataKey {
            admin,
            oracle,
            applications: Map::new(env),
            urban_data_cache: Map::new(env),
            base_rate,
            max_rate_adjustment: 15, // 15% maximum adjustment
        };
        env.storage().instance().set(&DATA_KEY, &data);
    }

    /// Submit a loan application with AI-driven rate adjustment
    pub fn submit_application(
        env: &Env,
        borrower: Address,
        asset_id: Symbol,
        requested_amount: i128,
        location: Symbol,
    ) -> Result<Symbol, Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Validate amount
        if requested_amount <= 0 {
            return Err(symbol_short!("INVALID_AMOUNT"));
        }

        // Generate application ID
        let application_id = Self::generate_application_id(env, &borrower, &asset_id);

        // Get or fetch urban data for the location
        let urban_data = Self::get_urban_data(env, &location);

        // Calculate equity score using AI oracle
        let equity_score = Self::calculate_equity_score(env, &urban_data);

        // Calculate AI-adjusted interest rate
        let adjusted_rate = Self::calculate_adjusted_rate(env, &data.base_rate, &equity_score, &urban_data);

        let application = LoanApplication {
            id: application_id.clone(),
            borrower,
            asset_id,
            requested_amount,
            base_rate: data.base_rate,
            adjusted_rate,
            equity_score,
            urban_data: urban_data.clone(),
            status: symbol_short!("pending"),
            created_at: env.ledger().timestamp(),
        };

        // Store application
        data.applications.set(&application_id, &application);
        
        // Cache urban data
        data.urban_data_cache.set(&location, &urban_data);
        
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(application_id)
    }

    /// Approve a loan application (admin only)
    pub fn approve_application(env: &Env, application_id: Symbol) -> Result<(), Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only admin can approve applications
        if env.current_contract_address() != data.admin {
            return Err(symbol_short!("UNAUTHORIZED"));
        }

        let mut application = data.applications.get(&application_id).ok_or(symbol_short!("APPLICATION_NOT_FOUND"))?;
        
        if application.status != symbol_short!("pending") {
            return Err(symbol_short!("INVALID_STATUS"));
        }

        application.status = symbol_short!("approved");
        data.applications.set(&application_id, &application);
        
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(())
    }

    /// Reject a loan application (admin only)
    pub fn reject_application(env: &Env, application_id: Symbol) -> Result<(), Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only admin can reject applications
        if env.current_contract_address() != data.admin {
            return Err(symbol_short!("UNAUTHORIZED"));
        }

        let mut application = data.applications.get(&application_id).ok_or(symbol_short!("APPLICATION_NOT_FOUND"))?;
        
        if application.status != symbol_short!("pending") {
            return Err(symbol_short!("INVALID_STATUS"));
        }

        application.status = symbol_short!("rejected");
        data.applications.set(&application_id, &application);
        
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(())
    }

    /// Get application details
    pub fn get_application(env: &Env, application_id: Symbol) -> Result<LoanApplication, Symbol> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        data.applications.get(&application_id).ok_or(symbol_short!("APPLICATION_NOT_FOUND"))
    }

    /// Get all applications for a borrower
    pub fn get_borrower_applications(env: &Env, borrower: Address) -> Vec<LoanApplication> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        let mut applications = vec![env];
        
        for (_, application) in data.applications.iter() {
            if application.borrower == borrower {
                applications.push_back(&application);
            }
        }
        
        applications
    }

    /// Update urban data (oracle only)
    pub fn update_urban_data(
        env: &Env,
        location: Symbol,
        income_level: i32,
        pollution_level: i32,
        public_transport_score: i32,
        population_density: i32,
    ) -> Result<(), Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only oracle can update urban data
        if env.current_contract_address() != data.oracle {
            return Err(symbol_short!("UNAUTHORIZED"));
        }

        let urban_data = UrbanData {
            location: location.clone(),
            income_level,
            pollution_level,
            public_transport_score,
            population_density,
            timestamp: env.ledger().timestamp(),
        };

        data.urban_data_cache.set(&location, &urban_data);
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(())
    }

    /// Get urban data for a location
    pub fn get_urban_data_for_location(env: &Env, location: Symbol) -> Result<UrbanData, Symbol> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        data.urban_data_cache.get(&location).ok_or(symbol_short!("DATA_NOT_FOUND"))
    }

    /// Calculate rate adjustment based on equity factors
    pub fn calculate_rate_adjustment(
        env: &Env,
        location: Symbol,
    ) -> Result<i32, Symbol> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        let urban_data = Self::get_urban_data(env, &location);
        let equity_score = Self::calculate_equity_score(env, &urban_data);
        let adjusted_rate = Self::calculate_adjusted_rate(env, &data.base_rate, &equity_score, &urban_data);
        
        Ok(adjusted_rate - data.base_rate)
    }

    /// Generate unique application ID
    fn generate_application_id(env: &Env, borrower: &Address, asset_id: &Symbol) -> Symbol {
        let borrower_str = borrower.to_string();
        let asset_str = asset_id.to_string();
        let timestamp = env.ledger().timestamp();
        
        let combined = format!("{}_{}_{}", borrower_str, asset_str, timestamp);
        let hash = env.crypto().sha256(&combined.as_bytes());
        
        // Convert first 8 bytes to symbol
        let mut id_bytes = [0u8; 8];
        id_bytes.copy_from_slice(&hash[0..8]);
        
        Symbol::from_bytes(&id_bytes)
    }

    /// Get urban data (fetch from oracle or use cached)
    fn get_urban_data(env: &Env, location: &Symbol) -> UrbanData {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Try to get cached data first
        if let Some(cached_data) = data.urban_data_cache.get(location) {
            return cached_data;
        }

        // If not cached, generate mock data based on location
        // In a real implementation, this would call the AI oracle
        Self::generate_mock_urban_data(env, location)
    }

    /// Generate mock urban data for demo purposes
    fn generate_mock_urban_data(env: &Env, location: &Symbol) -> UrbanData {
        let location_str = location.to_string();
        let hash = env.crypto().sha256(&location_str.as_bytes());
        
        // Generate deterministic but varied data based on location
        let income_level = ((hash[0] as i32) % 10) + 1;
        let pollution_level = ((hash[1] as i32) % 10) + 1;
        let public_transport_score = ((hash[2] as i32) % 10) + 1;
        let population_density = ((hash[3] as i32) % 10) + 1;

        UrbanData {
            location: location.clone(),
            income_level,
            pollution_level,
            public_transport_score,
            population_density,
            timestamp: env.ledger().timestamp(),
        }
    }

    /// Calculate equity score using AI oracle (mocked for demo)
    fn calculate_equity_score(env: &Env, urban_data: &UrbanData) -> i32 {
        // In a real implementation, this would call the AI oracle
        // For demo purposes, we'll use a weighted algorithm
        
        let mut score = 0;
        
        // Lower income areas get higher equity scores
        score += (11 - urban_data.income_level) * 10;
        
        // Higher pollution areas get higher equity scores (more need for clean transport)
        score += urban_data.pollution_level * 5;
        
        // Lower public transport access gets higher equity scores
        score += (11 - urban_data.public_transport_score) * 8;
        
        // Higher population density gets moderate equity boost
        score += urban_data.population_density * 3;
        
        // Normalize to 0-100 range
        score = score / 4;
        if score > 100 {
            score = 100;
        }
        
        score
    }

    /// Calculate AI-adjusted interest rate
    fn calculate_adjusted_rate(env: &Env, base_rate: &i32, equity_score: &i32, urban_data: &UrbanData) -> i32 {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Higher equity scores get lower rates
        let equity_adjustment = (100 - equity_score) * data.max_rate_adjustment / 100;
        
        // Additional adjustments based on urban factors
        let mut additional_adjustment = 0;
        
        // Lower income areas get additional rate reduction
        if urban_data.income_level <= 3 {
            additional_adjustment -= 5;
        }
        
        // Higher pollution areas get additional rate reduction
        if urban_data.pollution_level >= 8 {
            additional_adjustment -= 3;
        }
        
        // Poor public transport access gets additional rate reduction
        if urban_data.public_transport_score <= 3 {
            additional_adjustment -= 4;
        }
        
        let total_adjustment = equity_adjustment + additional_adjustment;
        let adjusted_rate = base_rate - total_adjustment;
        
        // Ensure rate doesn't go below 1%
        if adjusted_rate < 1 {
            1
        } else {
            adjusted_rate
        }
    }

    /// Get contract statistics
    pub fn get_stats(env: &Env) -> (i32, i32, i32) {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        let mut pending = 0;
        let mut approved = 0;
        let mut rejected = 0;
        
        for (_, application) in data.applications.iter() {
            match application.status.as_str() {
                "pending" => pending += 1,
                "approved" => approved += 1,
                "rejected" => rejected += 1,
                _ => {}
            }
        }
        
        (pending, approved, rejected)
    }
}

#[cfg(test)]
mod test;
