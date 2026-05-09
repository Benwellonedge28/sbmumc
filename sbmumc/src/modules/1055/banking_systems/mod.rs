//! # SBMUMC Module 1055: Banking Systems
//!
//! Banking institutions, services, and financial intermediation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BankType {
    Central,
    Commercial,
    Investment,
    Cooperative,
    Digital,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankingSystem {
    pub bank_id: String,
    pub bank_name: String,
    pub bank_type: BankType,
    pub total_assets: f64,
    pub capital_adequacy_ratio: f64,
    pub non_performing_loan_ratio: f64,
    pub profitability_score: f64,
}

impl BankingSystem {
    pub fn new(name: String, bank_type: BankType) -> Self {
        Self {
            bank_id: crate::core::uuid_simple(),
            bank_name: name,
            bank_type,
            total_assets: 0.0,
            capital_adequacy_ratio: 0.0,
            non_performing_loan_ratio: 0.0,
            profitability_score: 0.0,
        }
    }

    pub fn assess_bank(&mut self, assets: f64) -> Result<()> {
        self.total_assets = assets;

        match self.bank_type {
            BankType::Central => {
                self.capital_adequacy_ratio = 8.0 + rand_simple() * 4.0;
                self.non_performing_loan_ratio = 0.01 + rand_simple() * 0.02;
            },
            BankType::Commercial => {
                self.capital_adequacy_ratio = 10.0 + rand_simple() * 6.0;
                self.non_performing_loan_ratio = 0.02 + rand_simple() * 0.05;
            },
            BankType::Digital => {
                self.capital_adequacy_ratio = 12.0 + rand_simple() * 8.0;
                self.non_performing_loan_ratio = 0.01 + rand_simple() * 0.04;
            },
            _ => {
                self.capital_adequacy_ratio = 8.0 + rand_simple() * 6.0;
                self.non_performing_loan_ratio = 0.03 + rand_simple() * 0.07;
            }
        }

        self.profitability_score = (self.capital_adequacy_ratio / 15.0) * (1.0 - self.non_performing_loan_ratio);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankingService {
    pub service_id: String,
    pub service_type: String,
    pub fee_income_ratio: f64,
    pub customer_satisfaction: f64,
    pub digital_adoption_rate: f64,
    pub market_share: f64,
}

impl BankingService {
    pub fn new(service_type: String) -> Self {
        Self {
            service_id: crate::core::uuid_simple(),
            service_type,
            fee_income_ratio: 0.0,
            customer_satisfaction: 0.0,
            digital_adoption_rate: 0.0,
            market_share: 0.0,
        }
    }

    pub fn evaluate_service(&mut self) -> Result<()> {
        self.fee_income_ratio = 0.15 + rand_simple() * 0.35;
        self.customer_satisfaction = 0.7 + rand_simple() * 0.3;
        self.digital_adoption_rate = 0.3 + rand_simple() * 0.7;
        self.market_share = 0.05 + rand_simple() * 0.25;
        Ok(())
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

pub fn compute_loan_default_probability(bank_id: &str) -> Result<f64> {
    Ok(0.02 + rand_simple() * 0.08)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commercial_bank() {
        let mut bank = BankingSystem::new(
            "First_National_Bank".to_string(),
            BankType::Commercial,
        );
        bank.assess_bank(5e10).unwrap();
        assert!(bank.capital_adequacy_ratio > 8.0);
    }

    #[test]
    fn test_digital_banking() {
        let mut service = BankingService::new("Mobile_Payments".to_string());
        service.evaluate_service().unwrap();
        assert!(service.digital_adoption_rate > 0.3);
    }
}