#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, vec, Address, Env, Map, Symbol, Vec,
};

/// Represents a governance proposal
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub id: Symbol,
    pub title: Symbol,
    pub description: Symbol,
    pub proposer: Address,
    pub proposal_type: Symbol, // "asset_funding", "rate_adjustment", "policy_change"
    pub target_asset: Option<Symbol>, // For asset-specific proposals
    pub amount: Option<i128>, // For funding proposals
    pub start_time: u64,
    pub end_time: u64,
    pub status: Symbol, // "active", "passed", "failed", "executed"
    pub yes_votes: i128,
    pub no_votes: i128,
    pub total_votes: i128,
    pub equity_boost_threshold: i32, // Minimum equity score for boost
}

/// Represents a voter's participation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vote {
    pub voter: Address,
    pub proposal_id: Symbol,
    pub vote: Symbol, // "yes", "no", "abstain"
    pub voting_power: i128,
    pub equity_boost: i128,
    pub total_power: i128,
    pub equity_score: i32,
    pub timestamp: u64,
}

/// Represents a voter's stake and equity data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoterData {
    pub address: Address,
    pub stake_amount: i128,
    pub equity_score: i32,
    pub voting_power: i128,
    pub last_vote_time: u64,
    pub total_votes_cast: i32,
}

/// Contract data structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataKey {
    pub admin: Address,
    pub oracle: Address, // Equity oracle address
    pub loan_pool: Address, // Loan pool contract address
    pub proposals: Map<Symbol, Proposal>,
    pub votes: Map<Symbol, Vec<Vote>>, // proposal_id -> votes
    pub voters: Map<Address, VoterData>,
    pub min_proposal_duration: u64, // Minimum proposal duration in seconds
    pub quorum_threshold: i32, // Minimum participation percentage
    pub equity_boost_multiplier: i32, // Multiplier for equity-boosted votes
}

const DATA_KEY: Symbol = symbol_short!("DATA_KEY");

#[contract]
pub struct Governance;

#[contractimpl]
impl Governance {
    /// Initialize the contract
    pub fn initialize(
        env: &Env,
        admin: Address,
        oracle: Address,
        loan_pool: Address,
        min_proposal_duration: u64,
    ) {
        let data = DataKey {
            admin,
            oracle,
            loan_pool,
            proposals: Map::new(env),
            votes: Map::new(env),
            voters: Map::new(env),
            min_proposal_duration,
            quorum_threshold: 10, // 10% minimum participation
            equity_boost_multiplier: 150, // 50% boost for high-equity voters
        };
        env.storage().instance().set(&DATA_KEY, &data);
    }

    /// Create a new governance proposal
    pub fn create_proposal(
        env: &Env,
        proposer: Address,
        title: Symbol,
        description: Symbol,
        proposal_type: Symbol,
        target_asset: Option<Symbol>,
        amount: Option<i128>,
        duration: u64,
    ) -> Result<Symbol, Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Validate duration
        if duration < data.min_proposal_duration {
            return Err(symbol_short!("DURATION_TOO_SHORT"));
        }

        // Generate proposal ID
        let proposal_id = Self::generate_proposal_id(env, &proposer, &title);

        // Check if proposal already exists
        if data.proposals.contains_key(&proposal_id) {
            return Err(symbol_short!("PROPOSAL_EXISTS"));
        }

        let current_time = env.ledger().timestamp();
        let end_time = current_time + duration;

        let proposal = Proposal {
            id: proposal_id.clone(),
            title,
            description,
            proposer,
            proposal_type,
            target_asset,
            amount,
            start_time: current_time,
            end_time,
            status: symbol_short!("active"),
            yes_votes: 0,
            no_votes: 0,
            total_votes: 0,
            equity_boost_threshold: 70, // 70% equity score for boost
        };

        data.proposals.set(&proposal_id, &proposal);
        data.votes.set(&proposal_id, vec![env]);
        
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(proposal_id)
    }

    /// Vote on a proposal with equity-weighted voting power
    pub fn vote(
        env: &Env,
        voter: Address,
        proposal_id: Symbol,
        vote_choice: Symbol,
    ) -> Result<i128, Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Get proposal
        let mut proposal = data.proposals.get(&proposal_id).ok_or(symbol_short!("PROPOSAL_NOT_FOUND"))?;
        
        // Check if proposal is still active
        if proposal.status != symbol_short!("active") {
            return Err(symbol_short!("PROPOSAL_NOT_ACTIVE"));
        }

        let current_time = env.ledger().timestamp();
        if current_time > proposal.end_time {
            return Err(symbol_short!("VOTING_ENDED"));
        }

        // Get or create voter data
        let mut voter_data = data.voters.get(&voter).unwrap_or(VoterData {
            address: voter.clone(),
            stake_amount: 0,
            equity_score: 0,
            voting_power: 0,
            last_vote_time: 0,
            total_votes_cast: 0,
        });

        // Calculate voting power based on stake and equity
        let voting_power = Self::calculate_voting_power(env, &voter_data);
        let equity_boost = Self::calculate_equity_boost(env, &voter_data, &proposal);
        let total_power = voting_power + equity_boost;

        // Create vote record
        let vote = Vote {
            voter: voter.clone(),
            proposal_id: proposal_id.clone(),
            vote: vote_choice.clone(),
            voting_power,
            equity_boost,
            total_power,
            equity_score: voter_data.equity_score,
            timestamp: current_time,
        };

        // Update proposal votes
        let mut votes = data.votes.get(&proposal_id).unwrap_or(vec![env]);
        
        // Check if voter already voted
        for existing_vote in votes.iter() {
            if existing_vote.voter == voter {
                return Err(symbol_short!("ALREADY_VOTED"));
            }
        }

        votes.push_back(&vote);
        data.votes.set(&proposal_id, &votes);

        // Update proposal totals
        if vote_choice == symbol_short!("yes") {
            proposal.yes_votes += total_power;
        } else if vote_choice == symbol_short!("no") {
            proposal.no_votes += total_power;
        }
        // Abstain votes don't count toward totals

        proposal.total_votes += total_power;
        data.proposals.set(&proposal_id, &proposal);

        // Update voter data
        voter_data.last_vote_time = current_time;
        voter_data.total_votes_cast += 1;
        data.voters.set(&voter, &voter_data);
        
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(total_power)
    }

    /// Execute a passed proposal
    pub fn execute_proposal(env: &Env, proposal_id: Symbol) -> Result<(), Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only admin can execute proposals
        if env.current_contract_address() != data.admin {
            return Err(symbol_short!("UNAUTHORIZED"));
        }

        let mut proposal = data.proposals.get(&proposal_id).ok_or(symbol_short!("PROPOSAL_NOT_FOUND"))?;
        
        if proposal.status != symbol_short!("passed") {
            return Err(symbol_short!("PROPOSAL_NOT_PASSED"));
        }

        // Execute based on proposal type
        match proposal.proposal_type.as_str() {
            "asset_funding" => {
                // In a real implementation, this would trigger funding
                // For demo purposes, we'll just mark as executed
            },
            "rate_adjustment" => {
                // In a real implementation, this would adjust rates
            },
            "policy_change" => {
                // In a real implementation, this would update policies
            },
            _ => return Err(symbol_short!("UNKNOWN_PROPOSAL_TYPE")),
        }

        proposal.status = symbol_short!("executed");
        data.proposals.set(&proposal_id, &proposal);
        
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(())
    }

    /// Finalize voting and determine proposal outcome
    pub fn finalize_proposal(env: &Env, proposal_id: Symbol) -> Result<Symbol, Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        let mut proposal = data.proposals.get(&proposal_id).ok_or(symbol_short!("PROPOSAL_NOT_FOUND"))?;
        
        if proposal.status != symbol_short!("active") {
            return Err(symbol_short!("PROPOSAL_NOT_ACTIVE"));
        }

        let current_time = env.ledger().timestamp();
        if current_time <= proposal.end_time {
            return Err(symbol_short!("VOTING_NOT_ENDED"));
        }

        // Calculate total possible votes (all stakeholders)
        let total_possible_votes = Self::calculate_total_possible_votes(env);
        let participation_rate = if total_possible_votes > 0 {
            proposal.total_votes * 100 / total_possible_votes
        } else {
            0
        };

        // Check quorum
        if participation_rate < data.quorum_threshold {
            proposal.status = symbol_short!("failed");
            data.proposals.set(&proposal_id, &proposal);
            env.storage().instance().set(&DATA_KEY, &data);
            return Ok(symbol_short!("failed"));
        }

        // Determine outcome
        let outcome = if proposal.yes_votes > proposal.no_votes {
            proposal.status = symbol_short!("passed");
            symbol_short!("passed")
        } else {
            proposal.status = symbol_short!("failed");
            symbol_short!("failed")
        };

        data.proposals.set(&proposal_id, &proposal);
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(outcome)
    }

    /// Update voter's stake and equity data
    pub fn update_voter_data(
        env: &Env,
        voter: Address,
        stake_amount: i128,
        equity_score: i32,
    ) -> Result<(), Symbol> {
        let mut data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only oracle can update voter data
        if env.current_contract_address() != data.oracle {
            return Err(symbol_short!("UNAUTHORIZED"));
        }

        let mut voter_data = data.voters.get(&voter).unwrap_or(VoterData {
            address: voter.clone(),
            stake_amount: 0,
            equity_score: 0,
            voting_power: 0,
            last_vote_time: 0,
            total_votes_cast: 0,
        });

        voter_data.stake_amount = stake_amount;
        voter_data.equity_score = equity_score;
        voter_data.voting_power = Self::calculate_voting_power(env, &voter_data);

        data.voters.set(&voter, &voter_data);
        env.storage().instance().set(&DATA_KEY, &data);
        
        Ok(())
    }

    /// Get proposal details
    pub fn get_proposal(env: &Env, proposal_id: Symbol) -> Result<Proposal, Symbol> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        data.proposals.get(&proposal_id).ok_or(symbol_short!("PROPOSAL_NOT_FOUND"))
    }

    /// Get votes for a proposal
    pub fn get_proposal_votes(env: &Env, proposal_id: Symbol) -> Vec<Vote> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        data.votes.get(&proposal_id).unwrap_or(vec![env])
    }

    /// Get voter data
    pub fn get_voter_data(env: &Env, voter: Address) -> Result<VoterData, Symbol> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        data.voters.get(&voter).ok_or(symbol_short!("VOTER_NOT_FOUND"))
    }

    /// Get all active proposals
    pub fn get_active_proposals(env: &Env) -> Vec<Proposal> {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        let mut active_proposals = vec![env];
        
        for (_, proposal) in data.proposals.iter() {
            if proposal.status == symbol_short!("active") {
                active_proposals.push_back(&proposal);
            }
        }
        
        active_proposals
    }

    /// Generate unique proposal ID
    fn generate_proposal_id(env: &Env, proposer: &Address, title: &Symbol) -> Symbol {
        let proposer_str = proposer.to_string();
        let title_str = title.to_string();
        let timestamp = env.ledger().timestamp();
        
        let combined = format!("prop_{}_{}_{}", proposer_str, title_str, timestamp);
        let hash = env.crypto().sha256(&combined.as_bytes());
        
        // Convert first 8 bytes to symbol
        let mut id_bytes = [0u8; 8];
        id_bytes.copy_from_slice(&hash[0..8]);
        
        Symbol::from_bytes(&id_bytes)
    }

    /// Calculate voting power based on stake
    fn calculate_voting_power(env: &Env, voter_data: &VoterData) -> i128 {
        // Base voting power is 1:1 with stake amount
        voter_data.stake_amount
    }

    /// Calculate equity boost for voting power
    fn calculate_equity_boost(env: &Env, voter_data: &VoterData, proposal: &Proposal) -> i128 {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        // Only give equity boost if voter's equity score meets threshold
        if voter_data.equity_score >= proposal.equity_boost_threshold {
            // Calculate boost as percentage of base voting power
            let boost_percentage = data.equity_boost_multiplier - 100; // 50% boost
            voter_data.voting_power * boost_percentage / 100
        } else {
            0
        }
    }

    /// Calculate total possible votes from all stakeholders
    fn calculate_total_possible_votes(env: &Env) -> i128 {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        let mut total = 0;
        for (_, voter_data) in data.voters.iter() {
            total += voter_data.voting_power;
        }
        
        total
    }

    /// Get governance statistics
    pub fn get_stats(env: &Env) -> (i32, i32, i32, i32) {
        let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
        
        let mut total_proposals = 0;
        let mut active_proposals = 0;
        let mut passed_proposals = 0;
        let mut total_voters = 0;
        
        for (_, proposal) in data.proposals.iter() {
            total_proposals += 1;
            match proposal.status.as_str() {
                "active" => active_proposals += 1,
                "passed" => passed_proposals += 1,
                _ => {}
            }
        }
        
        for (_, _) in data.voters.iter() {
            total_voters += 1;
        }
        
        (total_proposals, active_proposals, passed_proposals, total_voters)
    }
}

#[cfg(test)]
mod test;
