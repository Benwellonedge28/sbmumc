//! # SBMUMC Module 1253: River Management
//!
//! Planning and control of river systems for multiple uses.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiverManagementObjective {
    FloodControl,
    WaterSupply,
    Navigation,
    Ecology,
    Hydropower,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiverManagementSystem {
    pub system_id: String,
    pub management_objective: RiverManagementObjective,
    pub objective_effectiveness: f64,
    pub environmental_flow: f64,
    pub stakeholder_balance: f64,
    pub longTermSustainability: f64,
}

impl RiverManagementSystem {
    pub fn new(management_objective: RiverManagementObjective) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            management_objective,
            objective_effectiveness: 0.0,
            environmental_flow: 0.0,
            stakeholder_balance: 0.0,
            long_term_sustainability: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.management_objective {
            RiverManagementObjective::FloodControl => {
                self.objective_effectiveness = 0.85 + rand_simple() * 0.14;
                self.environmental_flow = 0.50 + rand_simple() * 0.35;
            },
            RiverManagementObjective::WaterSupply => {
                self.objective_effectiveness = 0.80 + rand_simple() * 0.18;
                self.stakeholder_balance = 0.70 + rand_simple() * 0.25;
            },
            RiverManagementObjective::Navigation => {
                self.objective_effectiveness = 0.75 + rand_simple() * 0.22;
                self.environmental_flow = 0.45 + rand_simple() * 0.40;
            },
            RiverManagementObjective::Ecology => {
                self.environmental_flow = 0.85 + rand_simple() * 0.14;
                self.long_term_sustainability = 0.80 + rand_simple() * 0.18;
            },
            RiverManagementObjective::Hydropower => {
                self.objective_effectiveness = 0.80 + rand_simple() * 0.18;
                self.environmental_flow = 0.40 + rand_simple() * 0.40;
            },
        }

        if self.stakeholder_balance == 0.0 {
            self.stakeholder_balance = (self.objective_effectiveness + self.environmental_flow) / 2.0 * (0.6 + rand_simple() * 0.3);
        }
        if self.long_term_sustainability == 0.0 {
            self.long_term_sustainability = (self.environmental_flow + self.stakeholder_balance) / 2.0 * (0.7 + rand_simple() * 0.3);
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
    fn test_ecology_management() {
        let mut system = RiverManagementSystem::new(RiverManagementObjective::Ecology);
        system.analyze_system().unwrap();
        assert!(system.environmental_flow > 0.6);
    }
}