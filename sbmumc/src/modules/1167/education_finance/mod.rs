//! # SBMUMC Module 1167: Education Finance
//!
//! Funding mechanisms and resource allocation in education.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FundingModel {
    PerPupil,
    BlockGrant,
    Categorical,
    Voucher,
    EquityBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationFinanceSystem {
    pub system_id: String,
    pub funding_model: FundingModel,
    pub funding_adequacy: f64,
    pub funding_equity: f64,
    pub efficiency_score: f64,
    pub sustainability_index: f64,
}

impl EducationFinanceSystem {
    pub fn new(funding_model: FundingModel) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            funding_model,
            funding_adequacy: 0.0,
            funding_equity: 0.0,
            efficiency_score: 0.0,
            sustainability_index: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.funding_model {
            FundingModel::PerPupil => {
                self.funding_adequacy = 0.70 + rand_simple() * 0.25;
                self.efficiency_score = 0.80 + rand_simple() * 0.18;
            },
            FundingModel::BlockGrant => {
                self.funding_adequacy = 0.65 + rand_simple() * 0.30;
                self.funding_equity = 0.70 + rand_simple() * 0.25;
                self.sustainability_index = 0.75 + rand_simple() * 0.22;
            },
            FundingModel::Categorical => {
                self.funding_adequacy = 0.75 + rand_simple() * 0.22;
                self.funding_equity = 0.80 + rand_simple() * 0.18;
                self.efficiency_score = 0.60 + rand_simple() * 0.35;
            },
            FundingModel::Voucher => {
                self.funding_adequacy = 0.55 + rand_simple() * 0.40;
                self.funding_equity = 0.40 + rand_simple() * 0.45;
                self.efficiency_score = 0.85 + rand_simple() * 0.14;
            },
            FundingModel::EquityBased => {
                self.funding_adequacy = 0.70 + rand_simple() * 0.25;
                self.funding_equity = 0.90 + rand_simple() * 0.10;
                self.sustainability_index = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.sustainability_index == 0.0 {
            self.sustainability_index = (self.funding_adequacy + self.efficiency_score) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_equity_based_funding() {
        let mut system = EducationFinanceSystem::new(FundingModel::EquityBased);
        system.analyze_system().unwrap();
        assert!(system.funding_equity > 0.7);
    }
}