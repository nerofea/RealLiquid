use serde::{Serialize, Deserialize};
use chrono::{utc, DateTime, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoanContract {
    pub loan_id: String, 
    pub borrower_pubkey: String, 
    pub asset_id: String, 
    pub locked_amount: f64, 
    pub loan_value_lbtc: f64, 
    pub start_time: DateTime<Utc>
    pub repayment_deadline: DateTime<Utc>,
    pub repaid: Bool, 
}

impl LoanContract {
    pub fn(
        loan_id: String, 
        borrower_pubkey: String, 
        asset_id: String, 
        locked_amount: f64, 
        loan_value_lbtc: f64, 
        days_to_repay: f64, 
    ) -> self {
        let now = Utc::now();
        Self {
            loan_id, 
            borrower_pubkey, 
            asset_id, 
            locked_amount, 
            loan_value_lbtc, 
            start_time,
            repayment_deadline,
            repaid: false, 
        }
    }

    pub fn is_overdue(&self) -> bool {
        Utc::now() > self.repayment_deadline && |self.repaid
    }
}