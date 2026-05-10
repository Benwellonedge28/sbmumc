//! # SBMUMC Module 1277: Deep Sea Exploration
//!
//! Systems for exploring deep ocean environments.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExplorationTechnology {
    MannedSubmersibles,
    AutonomousUnderwaterVehicles,
    RemotelyOperatedVehicles,
    DeepSeaObservatories,
    SamplingSystems,
    MappingSonars,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeaExplorationSystem {
    pub system_id: String,
    pub exploration_technology: ExplorationTechnology,
    pub depth_capability: f64,
    pub data_collection: f64,
    pub operational_range: f64,
    pub mission_duration: f64,
}

impl DeepSeaExplorationSystem {
    pub fn new(exploration_technology: ExplorationTechnology) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            exploration_technology,
            depth_capability: 0.0,
            data_collection: 0.0,
            operational_range: 0.0,
            mission_duration: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.exploration_technology {
            ExplorationTechnology::MannedSubmersibles => {
                self.depth_capability = 0.85 + rand_simple() * 0.14;
                self.data_collection = 0.90 + rand_simple() * 0.10;
                self.mission_duration = 0.60 + rand_simple() * 0.35;
            },
            ExplorationTechnology::AutonomousUnderwaterVehicles => {
                self.operational_range = 0.90 + rand_simple() * 0.10;
                self.mission_duration = 0.85 + rand_simple() * 0.14;
                self.data_collection = 0.75 + rand_simple() * 0.22;
            },
            ExplorationTechnology::RemotelyOperatedVehicles => {
                self.depth_capability = 0.95 + rand_simple() * 0.05;
                self.data_collection = 0.85 + rand_simple() * 0.14;
                self.operational_range = 0.70 + rand_simple() * 0.25;
            },
            ExplorationTechnology::DeepSeaObservatories => {
                self.mission_duration = 0.95 + rand_simple() * 0.05;
                self.data_collection = 0.90 + rand_simple() * 0.10;
                self.operational_range = 0.80 + rand_simple() * 0.18;
            },
            ExplorationTechnology::SamplingSystems => {
                self.data_collection = 0.80 + rand_simple() * 0.18;
                self.depth_capability = 0.75 + rand_simple() * 0.22;
                self.operational_range = 0.65 + rand_simple() * 0.30;
            },
            ExplorationTechnology::MappingSonars => {
                self.operational_range = 0.85 + rand_simple() * 0.14;
                self.data_collection = 0.90 + rand_simple() * 0.10;
                self.depth_capability = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.operational_range == 0.0 {
            self.operational_range = (self.depth_capability + self.data_collection) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_rov_systems() {
        let mut system = DeepSeaExplorationSystem::new(ExplorationTechnology::RemotelyOperatedVehicles);
        system.analyze_system().unwrap();
        assert!(system.depth_capability > 0.8);
    }
}
