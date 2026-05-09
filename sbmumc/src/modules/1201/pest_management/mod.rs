//! # SBMUMC Module 1201: Pest Management
//!
//! Strategies for controlling agricultural pests and diseases.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PestManagementStrategy {
    Chemical,
    Biological,
    Cultural,
    Integrated,
    Genetic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PestManagementSystem {
    pub system_id: String,
    pub management_strategy: PestManagementStrategy,
    pub control_efficacy: f64,
    pub resistance_development: f64,
    pub ecosystem_impact: f64,
    pub economic_cost: f64,
}

impl PestManagementSystem {
    pub fn new(management_strategy: PestManagementStrategy) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            management_strategy,
            control_efficacy: 0.0,
            resistance_development: 0.0,
            ecosystem_impact: 0.0,
            economic_cost: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.management_strategy {
            PestManagementStrategy::Chemical => {
                self.control_efficacy = 0.90 + rand_simple() * 0.10;
                self.resistance_development = 0.70 + rand_simple() * 0.25;
                self.ecosystem_impact = 0.25 + rand_simple() * 0.30;
            },
            PestManagementStrategy::Biological => {
                self.control_efficacy = 0.70 + rand_simple() * 0.25;
                self.ecosystem_impact = 0.85 + rand_simple() * 0.14;
                self.resistance_development = 0.20 + rand_simple() * 0.25;
            },
            PestManagementStrategy::Cultural => {
                self.control_efficacy = 0.60 + rand_simple() * 0.35;
                self.ecosystem_impact = 0.80 + rand_simple() * 0.18;
                self.economic_cost = 0.30 + rand_simple() * 0.30;
            },
            PestManagementStrategy::Integrated => {
                self.control_efficacy = 0.80 + rand_simple() * 0.18;
                self.ecosystem_impact = 0.70 + rand_simple() * 0.25;
                self.resistance_development = 0.40 + rand_simple() * 0.30;
            },
            PestManagementStrategy::Genetic => {
                self.control_efficacy = 0.85 + rand_simple() * 0.14;
                self.resistance_development = 0.30 + rand_simple() * 0.30;
                self.ecosystem_impact = 0.50 + rand_simple() * 0.35;
            },
        }

        if self.economic_cost == 0.0 {
            self.economic_cost = (1.0 - self.control_efficacy) * (0.5 + rand_simple() * 0.5);
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
    fn test_integrated_pest_mgmt() {
        let mut system = PestManagementSystem::new(PestManagementStrategy::Integrated);
        system.analyze_system().unwrap();
        assert!(system.control_efficacy > 0.6);
    }
}