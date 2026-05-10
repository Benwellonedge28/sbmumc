//! # SBMUMC Module 1211: Farm Management
//!
//! Planning, organization, and control of farm operations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManagementApproach {
    Traditional,
    Commercial,
    Diversified,
    ContractFarming,
    Cooperative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmManagementSystem {
    pub system_id: String,
    pub management_approach: ManagementApproach,
    pub operational_efficiency: f64,
    pub risk_management: f64,
    pub profitability: f64,
    pub sustainability_integration: f64,
}

impl FarmManagementSystem {
    pub fn new(management_approach: ManagementApproach) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            management_approach,
            operational_efficiency: 0.0,
            risk_management: 0.0,
            profitability: 0.0,
            sustainability_integration: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.management_approach {
            ManagementApproach::Traditional => {
                self.operational_efficiency = 0.60 + rand_simple() * 0.30;
                self.risk_management = 0.50 + rand_simple() * 0.40;
            },
            ManagementApproach::Commercial => {
                self.operational_efficiency = 0.85 + rand_simple() * 0.14;
                self.profitability = 0.80 + rand_simple() * 0.18;
                self.risk_management = 0.65 + rand_simple() * 0.30;
            },
            ManagementApproach::Diversified => {
                self.risk_management = 0.85 + rand_simple() * 0.14;
                self.sustainability_integration = 0.75 + rand_simple() * 0.22;
                self.operational_efficiency = 0.60 + rand_simple() * 0.35;
            },
            ManagementApproach::ContractFarming => {
                self.profitability = 0.85 + rand_simple() * 0.14;
                self.operational_efficiency = 0.80 + rand_simple() * 0.18;
            },
            ManagementApproach::Cooperative => {
                self.risk_management = 0.80 + rand_simple() * 0.18;
                self.operational_efficiency = 0.70 + rand_simple() * 0.25;
                self.sustainability_integration = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.profitability == 0.0 {
            self.profitability = (self.operational_efficiency + self.risk_management) / 2.0 * (0.7 + rand_simple() * 0.3);
        }
        if self.sustainability_integration == 0.0 {
            self.sustainability_integration = 0.50 + rand_simple() * 0.40;
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
    fn test_commercial_management() {
        let mut system = FarmManagementSystem::new(ManagementApproach::Commercial);
        system.analyze_system().unwrap();
        assert!(system.operational_efficiency > 0.6);
    }
}