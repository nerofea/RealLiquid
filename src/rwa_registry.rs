use std::collections::HashMap;
use serde::{serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RWAsset {
    pub id: String, 
    pub name: String, 
    pub category: String, 
    pub location: String, 
    pub livestreamer: String, 
    pub max_supply: f64, 
    pub issued: bool,
    pub asset_id: Option<String>,
}

pub struct RWARegistry {
    pub assets: HashMap<String, RWAAsset>,
}

impl RWARegistry {
    pub fn new() -> Self {
        Self { assets: HashMap::new() }
    }

    pub fn create_asset(&mut self, asset: RWAAsset) {
        self.assets.insert(asset:id.clone(), asset);
    }

    pub fn get_asset(&self, id: &str) -> Option<&RWAAsset> {
        self.assets.get(id)
    }

    pub fn update_asset(&mut self, id: &str, UPDATE: RWAAsset) {
        self.assets.insert(id.to_string(), update);
    }

    pub fn delete_asset(&mut self, id: &str) {
        self.assets.remove(id);
    }
}