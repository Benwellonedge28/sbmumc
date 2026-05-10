//! # SBMUMC Module 1258: Flood Management
//!
//! Strategies for reducing flood risks and impacts.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FloodManagementStrategy {
    Structural,
    Natural,
    EarlyWarning,
    LandUse,
    Insurance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloodManagementSystem {
    pub system_id: String,
    pub management_strategy: FloodManagementStrategy,
    pub risk_reduction: f64,
    pub response_time: f64,
    pub community_resilience: f64,
    pub cost_effectiveness: f64,
}

impl FloodManagementSystem {
    pub fn new(management_strategy: FloodManagementStrategy) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            management_strategy,
            risk_reduction: 0.0,
            response_time: 0.0,
            community_resilience: 0.0,
            cost_effectiveness: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.management_strategy {
            FloodManagementStrategy::Structural => {
                self.risk_reduction = 0.85 + rand_simple() * 0.14;
                self.cost_effectiveness = 0.60 + rand_simple() * 0.35;
            },
            FloodManagementStrategy::Natural => {
                self.risk_reduction = 0.70 + rand_simple() * 0.25;
                self.community_resilience = 0.80 + rand_simple() * 0.18;
                self.cost_effectiveness = 0.75 + rand_simple() * 0.22;
            },
            FloodManagementStrategy::EarlyWarning => {
                self.response_time = 0.90 + rand_simple() * 0.10;
                self.community_resilience = 0.85 + rand_simple() * 0.14;
            },
            FloodManagementStrategy::LandUse => {
                self.risk_reduction = 0.80 + rand_simple() * 0.18;
                self.cost_effectiveness = 0.70 + rand_simple() * 0.25;
            },
            FloodManagementStrategy::Insurance => {
                self.community_resilience = 0.75 + rand_simple() * 0.22;
                self.cost_effectiveness = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.response_time == 0.0 {
            self.response_time = (self.risk_reduction + self.community_resilience) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_early_warning() {
        let mut system = FloodManagementSystem::new(FloodManagementStrategy::EarlyWarning);
        system.analyze_system().unwrap();
        assert!(system.response_time > 0.7);
    }
}