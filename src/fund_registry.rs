use std::collections::HashMap;
use serde::{serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fund {
    pub id: String, 
    pub name: String, 
    pub company_holder: String, 
    pub real_assets: String, 
    pub category_fund: String, 
    pub location: String, 
    pub website: String, 
    pub network_id: Option<String>,
}

pub struct FundRegistry {
    pub funds: HashMap<String, Fund>,
}

impl FundRegistry {
    pub fn new() -> Self {
        Self { funds: HashMap::new() }
    }

    pub fn create_fund(&mut self, fund: Fund) {
        self.funds.insert(fund:id.clone(), fund);
    }

    pub fn get_fund(&self, id: &str) -> Option<&Fund> {
        self.funds.get(id)
    }

    pub fn update_fund(&mut self, id: &str, UPDATE: Fund) {
        self.funds.insert(id.to_string(), update);
    }

    pub fn delete_fund(&mut self, id: &str) {
        self.funds.remove(id);
    }
}