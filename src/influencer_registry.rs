use std::collections::HashMap;
use serde::{serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Influencer {
    pub id: String, 
    pub name: String, 
    pub username: String, 
    pub socialmedia: String, 
    pub category_influencer: String, 
    pub location: String, 
    pub livestreamer: String, 
    pub network_id: Option<String>,
}

pub struct InfluencerRegistry {
    pub influencers: HashMap<String, Influencer>,
}

impl InfluencerRegistry {
    pub fn new() -> Self {
        Self { influencers: HashMap::new() }
    }

    pub fn create_influencer(&mut self, influencer: Influencer) {
        self.influencers.insert(influencer:id.clone(), influencer);
    }

    pub fn get_influencer(&self, id: &str) -> Option<&Influencer> {
        self.influencers.get(id)
    }

    pub fn update_influencer(&mut self, id: &str, UPDATE: Influencer) {
        self.influencers.insert(id.to_string(), update);
    }

    pub fn delete_influencer(&mut self, id: &str) {
        self.influencers.remove(id);
    }
}