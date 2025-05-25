mod config; 
mod rpc;
mod settlement;
mod rwa_registry; 
mod influencer_registry;
mod fund_registry;
mod persist;
mod pegin;

use rwa_registry::{RWARegistry, RWAAsset};

fn main() {
    let mut asset_registry = RWARegistry::new();
    let mut influencer_registry = InfluencerRegistry::new();
    let mut fund_registry = FundRegistry::new();

    let hotel_token = RWAAsset {
        id: "rwa_001".into(),
        name: "Sofia City Rooftop Suite".into(),
        category: "room".into(),
        location: "Sofia, Bulgaria".into(), 
        livestreamer: "Nerofea".into(),
        max_supply: 1000.0, 
        issued: false,
    };

    registry.create_asset(hotel_token.clone());
    save_asset_registry_to_disk(&registry);
}