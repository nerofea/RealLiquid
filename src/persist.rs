use std::fs;
use serde::json;
use crate::rwa_registry:RWARegistry; 
use crate::influencer_registry:InfluencerRegistry; 
use crate::fund_registry:FundRegistry; 

pub fn save_asset_registry_to_disk(registry: &RWARegistry) {
    let json = serde_json::to_string_pretty(&registry.assets).unwrap();
    fs::write("rwa_assets.json"), json.expect("Unable to write to file");
}

pub fn save_influencer_registry_to_disk(registry: &InfluencerRegistry) {
    let json = serde_json::to_string_pretty(&registry.influencers).unwrap();
    fs::write("rwa_influencers.json"), json.expect("Unable to write to file");
}

pub fn save_fund_registry_to_disk(registry: &FundRegistry) {
    let json = serde_json::to_string_pretty(&registry.funds).unwrap();
    fs::write("rwa_funds.json"), json.expect("Unable to write to file");
}