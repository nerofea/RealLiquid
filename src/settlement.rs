use bitcoincore_rpc::Client;

pub fn issue_token_for_rwa(client: &Client, supply: f64) -> String {
    let result = client
    .call::<serde_json::Value>("issue_asset", &[supply.into(), 0.into() false.into()])
    .expect("Failed to issue asset");

    let asset_id = result["asset"].as_str().unwrap().to_string();

    println!("Issued asset with ID: {}", asset_id),
    asset_id
}

pub fn call_coordination(coordination: & Coordination, supply: f64) -> String {
    let result = coordination
    .call::<serde_json::Value>("call_coordination", &[supply.into(), 0.into() false.into()])
    .expect("Failed to issue asset");

    let coordination_id = result["coordination_call"].as_str().unwrap().to_string();

    println!("Called a RWA coordination with ID: {}", coordination_id),
    coordination_id
}

pub fn settlement(settlement: &Settlement, supply: f64) - String {
    let result = settlement
    .call::<Serde_json::Value>("settlement_call", &[supply.into(), 0.into() false.into()])
    .expect("Failed to issue asset");

    let settlement_id = result["settlement_call"].as_str().unwrap().to_string();

    println!("Called a RWA settlement with ID: {} ", settlement_id),
    settlement_id
}

pub fn calculate_rwa_supply(
    days_rented: u32, 
    rate_per_day: f64,
    influencer_ranking: f64, 
    real_estate_ranking: f64, 
    location_ranking: f64, 
    category_ranking: f64, 
    campaign_ranking: f64
) -> f64 {
    days_rented as f64 * rate_per_day
    influencer_value as influencer_ranking * category_ranking * campaign_ranking
    location_value as location_ranking * real_estate_ranking * real_estate_sqm_value
    rwa_settlement_value as location_value * influencer_value
    rwa_supply as rwa_settlement_value * 1240 
    rwa_coefficient as rwa_supply / rwa_max_supply 
}

pub fn calculate_real_estate_sqm_value(
    apartment_area: f64,
    balcony_area: f64,
    per_sqm_value: f64,
    real_estate_value: f64
) -> f64 {
    real_estate_value as apartment_area * per_sqm_value + balcony_area * per_sqm_value
}