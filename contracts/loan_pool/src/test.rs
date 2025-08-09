#![cfg(test)]

use super::*;
use soroban_sdk::{
    symbol_short, vec, Address, Env, Symbol,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LoanPool);
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);

    LoanPool::initialize(&env, &admin, &oracle);

    // Verify initialization
    let data: DataKey = env.storage().instance().get(&DATA_KEY).unwrap();
    assert_eq!(data.admin, admin);
    assert_eq!(data.equity_oracle, oracle);
    assert_eq!(data.total_pool_balance, 0);
}

#[test]
fn test_create_asset() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LoanPool);
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);

    LoanPool::initialize(&env, &admin, &oracle);

    let asset_id = symbol_short!("ebike_001");
    let name = symbol_short!("Downtown E-Bikes");
    let asset_type = symbol_short!("e-bike");
    let target_amount = 10000;
    let location = symbol_short!("downtown_low_income");

    // Create asset
    LoanPool::create_asset(&env, &asset_id, &name, &asset_type, &target_amount, &location);

    // Verify asset creation
    let asset = LoanPool::get_asset(&env, &asset_id).unwrap();
    assert_eq!(asset.id, asset_id);
    assert_eq!(asset.name, name);
    assert_eq!(asset.asset_type, asset_type);
    assert_eq!(asset.target_amount, target_amount);
    assert_eq!(asset.funded_amount, 0);
    assert_eq!(asset.location, location);
    assert_eq!(asset.status, symbol_short!("funding"));
    assert!(asset.equity_score > 0); // Should have AI-calculated equity score
}

#[test]
fn test_invest_with_equity_bonus() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LoanPool);
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let investor = Address::generate(&env);

    LoanPool::initialize(&env, &admin, &oracle);

    // Create asset in underserved area
    let asset_id = symbol_short!("ebike_002");
    let location = symbol_short!("underserved_zone");
    
    LoanPool::create_asset(
        &env, 
        &asset_id, 
        &symbol_short!("Community E-Bikes"), 
        &symbol_short!("e-bike"), 
        &5000, 
        &location
    );

    // Invest in the asset
    let investment_amount = 1000;
    let equity_bonus = LoanPool::invest(&env, &investor, &asset_id, &investment_amount).unwrap();

    // Verify investment
    assert!(equity_bonus > 0); // Should have equity bonus for underserved area
    
    let asset = LoanPool::get_asset(&env, &asset_id).unwrap();
    assert_eq!(asset.funded_amount, investment_amount);
    assert_eq!(asset.investors.len(), 1);
    assert_eq!(asset.investors.get(0).unwrap(), investor);

    // Verify pool balance
    assert_eq!(LoanPool::get_pool_balance(&env), investment_amount);
}

#[test]
fn test_multiple_investments() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LoanPool);
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let investor1 = Address::generate(&env);
    let investor2 = Address::generate(&env);

    LoanPool::initialize(&env, &admin, &oracle);

    let asset_id = symbol_short!("shuttle_001");
    LoanPool::create_asset(
        &env, 
        &asset_id, 
        &symbol_short!("Community Shuttle"), 
        &symbol_short!("shuttle"), 
        &20000, 
        &symbol_short!("suburban_area")
    );

    // First investment
    LoanPool::invest(&env, &investor1, &asset_id, &8000).unwrap();
    
    // Second investment
    LoanPool::invest(&env, &investor2, &asset_id, &12000).unwrap();

    let asset = LoanPool::get_asset(&env, &asset_id).unwrap();
    assert_eq!(asset.funded_amount, 20000);
    assert_eq!(asset.status, symbol_short!("funded")); // Should be fully funded
    assert_eq!(asset.investors.len(), 2);

    // Verify investments
    let investments = LoanPool::get_asset_investments(&env, &asset_id);
    assert_eq!(investments.len(), 2);
}

#[test]
fn test_equity_score_calculation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LoanPool);
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);

    LoanPool::initialize(&env, &admin, &oracle);

    // Create asset in low-income area
    let low_income_asset = symbol_short!("low_income_asset");
    LoanPool::create_asset(
        &env, 
        &low_income_asset, 
        &symbol_short!("Low Income E-Bikes"), 
        &symbol_short!("e-bike"), 
        &5000, 
        &symbol_short!("low_income_zone")
    );

    // Create asset in high-income area
    let high_income_asset = symbol_short!("high_income_asset");
    LoanPool::create_asset(
        &env, 
        &high_income_asset, 
        &symbol_short!("High Income E-Bikes"), 
        &symbol_short!("e-bike"), 
        &5000, 
        &symbol_short!("high_income_zone")
    );

    let low_income_equity = LoanPool::get_asset(&env, &low_income_asset).unwrap().equity_score;
    let high_income_equity = LoanPool::get_asset(&env, &high_income_asset).unwrap().equity_score;

    // Low-income areas should generally have higher equity scores
    assert!(low_income_equity >= 0 && low_income_equity <= 100);
    assert!(high_income_equity >= 0 && high_income_equity <= 100);
}

#[test]
fn test_asset_lifecycle() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LoanPool);
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let investor = Address::generate(&env);

    LoanPool::initialize(&env, &admin, &oracle);

    let asset_id = symbol_short!("lifecycle_test");
    LoanPool::create_asset(
        &env, 
        &asset_id, 
        &symbol_short!("Lifecycle Test"), 
        &symbol_short!("e-bike"), 
        &1000, 
        &symbol_short!("test_zone")
    );

    // Fund the asset
    LoanPool::invest(&env, &investor, &asset_id, &1000).unwrap();
    
    let asset = LoanPool::get_asset(&env, &asset_id).unwrap();
    assert_eq!(asset.status, symbol_short!("funded"));

    // Deploy the asset
    LoanPool::deploy_asset(&env, &asset_id).unwrap();
    
    let asset = LoanPool::get_asset(&env, &asset_id).unwrap();
    assert_eq!(asset.status, symbol_short!("deployed"));

    // Complete the asset
    LoanPool::complete_asset(&env, &asset_id).unwrap();
    
    let asset = LoanPool::get_asset(&env, &asset_id).unwrap();
    assert_eq!(asset.status, symbol_short!("completed"));
}

#[test]
fn test_get_all_assets() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LoanPool);
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);

    LoanPool::initialize(&env, &admin, &oracle);

    // Create multiple assets
    LoanPool::create_asset(
        &env, 
        &symbol_short!("asset1"), 
        &symbol_short!("Asset 1"), 
        &symbol_short!("e-bike"), 
        &1000, 
        &symbol_short!("zone1")
    );

    LoanPool::create_asset(
        &env, 
        &symbol_short!("asset2"), 
        &symbol_short!("Asset 2"), 
        &symbol_short!("shuttle"), 
        &2000, 
        &symbol_short!("zone2")
    );

    let all_assets = LoanPool::get_all_assets(&env);
    assert_eq!(all_assets.len(), 2);
}

#[test]
#[should_panic(expected = "ASSET_NOT_FOUND")]
fn test_get_nonexistent_asset() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LoanPool);
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);

    LoanPool::initialize(&env, &admin, &oracle);
    
    // Try to get non-existent asset
    LoanPool::get_asset(&env, &symbol_short!("nonexistent")).unwrap();
}

#[test]
#[should_panic(expected = "INVALID_AMOUNT")]
fn test_invest_invalid_amount() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LoanPool);
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let investor = Address::generate(&env);

    LoanPool::initialize(&env, &admin, &oracle);

    let asset_id = symbol_short!("test_asset");
    LoanPool::create_asset(
        &env, 
        &asset_id, 
        &symbol_short!("Test Asset"), 
        &symbol_short!("e-bike"), 
        &1000, 
        &symbol_short!("test_zone")
    );

    // Try to invest with invalid amount
    LoanPool::invest(&env, &investor, &asset_id, &0).unwrap();
}

#[test]
fn test_equity_bonus_calculation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LoanPool);
    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let investor = Address::generate(&env);

    LoanPool::initialize(&env, &admin, &oracle);

    // Test investment in underserved area (should get higher bonus)
    let underserved_asset = symbol_short!("underserved_asset");
    LoanPool::create_asset(
        &env, 
        &underserved_asset, 
        &symbol_short!("Underserved Asset"), 
        &symbol_short!("e-bike"), 
        &1000, 
        &symbol_short!("underserved_zone")
    );

    let underserved_bonus = LoanPool::invest(&env, &investor, &underserved_asset, &500).unwrap();

    // Test investment in regular area (should get lower bonus)
    let regular_asset = symbol_short!("regular_asset");
    LoanPool::create_asset(
        &env, 
        &regular_asset, 
        &symbol_short!("Regular Asset"), 
        &symbol_short!("e-bike"), 
        &1000, 
        &symbol_short!("regular_zone")
    );

    let regular_bonus = LoanPool::invest(&env, &investor, &regular_asset, &500).unwrap();

    // Underserved areas should generally get higher equity bonuses
    assert!(underserved_bonus >= regular_bonus);
    assert!(underserved_bonus <= 25); // Max bonus cap
    assert!(regular_bonus <= 25); // Max bonus cap
}
