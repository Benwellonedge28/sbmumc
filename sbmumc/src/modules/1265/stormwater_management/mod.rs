//! # SBMUMC Module 1265: Stormwater Management
//!
//! Systems for managing stormwater runoff and drainage.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StormwaterManagementStrategy {
    RetentionPonds,
    PermeablePavements,
    GreenInfrastructures,
    DetentionSystems,
    WaterHarvesting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StormwaterManagementSystem {
    pub system_id: String,
    pub management_strategy: StormwaterManagementStrategy,
    pub runoff_reduction: f64,
    pub water_quality_treatment: f64,
    pub flood_mitigation: f64,
    pub cost_effectiveness: f64,
}

impl StormwaterManagementSystem {
    pub fn new(management_strategy: StormwaterManagementStrategy) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            management_strategy,
            runoff_reduction: 0.0,
            water_quality_treatment: 0.0,
            flood_mitigation: 0.0,
            cost_effectiveness: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.management_strategy {
            StormwaterManagementStrategy::RetentionPonds => {
                self.runoff_reduction = 0.80 + rand_simple() * 0.18;
                self.water_quality_treatment = 0.75 + rand_simple() * 0.22;
                self.flood_mitigation = 0.85 + rand_simple() * 0.14;
            },
            StormwaterManagementStrategy::PermeablePavements => {
                self.runoff_reduction = 0.70 + rand_simple() * 0.25;
                self.cost_effectiveness = 0.75 + rand_simple() * 0.22;
                self.water_quality_treatment = 0.60 + rand_simple() * 0.35;
            },
            StormwaterManagementStrategy::GreenInfrastructures => {
                self.water_quality_treatment = 0.85 + rand_simple() * 0.14;
                self.runoff_reduction = 0.70 + rand_simple() * 0.25;
                self.cost_effectiveness = 0.65 + rand_simple() * 0.30;
            },
            StormwaterManagementStrategy::DetentionSystems => {
                self.flood_mitigation = 0.90 + rand_simple() * 0.10;
                self.runoff_reduction = 0.75 + rand_simple() * 0.22;
                self.cost_effectiveness = 0.70 + rand_simple() * 0.25;
            },
            StormwaterManagementStrategy::WaterHarvesting => {
                self.cost_effectiveness = 0.80 + rand_simple() * 0.18;
                self.runoff_reduction = 0.65 + rand_simple() * 0.30;
                self.water_quality_treatment = 0.55 + rand_simple() * 0.40;
            },
        }

        if self.cost_effectiveness == 0.0 {
            self.cost_effectiveness = (self.runoff_reduction + self.water_quality_treatment) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_detention_systems() {
        let mut system = StormwaterManagementSystem::new(StormwaterManagementStrategy::DetentionSystems);
        system.analyze_system().unwrap();
        assert!(system.flood_mitigation > 0.7);
    }
}
