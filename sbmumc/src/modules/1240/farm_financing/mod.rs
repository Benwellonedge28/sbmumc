//! # SBMUMC Module 1240: Farm Financing
//!
//! Financial services and capital access for agricultural operations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinancingModel {
    Traditional,
    Microfinance,
    ValueChain,
    Crowdfunding,
    AgriculturalBonds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmFinancingSystem {
    pub system_id: String,
    pub financing_model: FinancingModel,
    pub capital_access: f64,
    pub interest_affordability: f64,
    pub risk_sharing: f64,
    pub financial_inclusion: f64,
}

impl FarmFinancingSystem {
    pub fn new(financing_model: FinancingModel) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            financing_model,
            capital_access: 0.0,
            interest_affordability: 0.0,
            risk_sharing: 0.0,
            financial_inclusion: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.financing_model {
            FinancingModel::Traditional => {
                self.capital_access = 0.70 + rand_simple() * 0.25;
                self.interest_affordability = 0.60 + rand_simple() * 0.35;
            },
            FinancingModel::Microfinance => {
                self.financial_inclusion = 0.90 + rand_simple() * 0.10;
                self.capital_access = 0.85 + rand_simple() * 0.14;
                self.interest_affordability = 0.65 + rand_simple() * 0.30;
            },
            FinancingModel::ValueChain => {
                self.capital_access = 0.85 + rand_simple() * 0.14;
                self.risk_sharing = 0.80 + rand_simple() * 0.18;
                self.interest_affordability = 0.75 + rand_simple() * 0.22;
            },
            FinancingModel::Crowdfunding => {
                self.capital_access = 0.75 + rand_simple() * 0.22;
                self.financial_inclusion = 0.80 + rand_simple() * 0.18;
            },
            FinancingModel::AgriculturalBonds => {
                self.capital_access = 0.80 + rand_simple() * 0.18;
                self.interest_affordability = 0.70 + rand_simple() * 0.25;
                self.risk_sharing = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.risk_sharing == 0.0 {
            self.risk_sharing = (self.capital_access + self.financial_inclusion) / 2.0 * (0.6 + rand_simple() * 0.3);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microfinance() {
        let mut system = FarmFinancingSystem::new(FinancingModel::Microfinance);
        system.analyze_system().unwrap();
        assert!(system.financial_inclusion > 0.7);
    }
}