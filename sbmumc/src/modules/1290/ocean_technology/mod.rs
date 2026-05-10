//! # SBMUMC Module 1290: Ocean Technology
//!
//! Systems for developing ocean observation and exploration technologies.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OceanTechnologyType {
    SensorTechnology,
    UnderwaterRobotics,
    CommunicationSystems,
    DataAnalytics,
    AutonomousPlatforms,
    EnergySystems,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OceanTechnologySystem {
    pub system_id: String,
    pub technology_type: OceanTechnologyType,
    pub technology_maturity: f64,
    pub operational_reliability: f64,
    pub innovation_index: f64,
    pub cost_effectiveness: f64,
}

impl OceanTechnologySystem {
    pub fn new(technology_type: OceanTechnologyType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            technology_type,
            technology_maturity: 0.0,
            operational_reliability: 0.0,
            innovation_index: 0.0,
            cost_effectiveness: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.technology_type {
            OceanTechnologyType::SensorTechnology => {
                self.technology_maturity = 0.85 + rand_simple() * 0.14;
                self.operational_reliability = 0.90 + rand_simple() * 0.10;
                self.innovation_index = 0.75 + rand_simple() * 0.22;
            },
            OceanTechnologyType::UnderwaterRobotics => {
                self.operational_reliability = 0.80 + rand_simple() * 0.18;
                self.innovation_index = 0.90 + rand_simple() * 0.10;
                self.technology_maturity = 0.75 + rand_simple() * 0.22;
            },
            OceanTechnologyType::CommunicationSystems => {
                self.technology_maturity = 0.80 + rand_simple() * 0.18;
                self.operational_reliability = 0.85 + rand_simple() * 0.14;
                self.innovation_index = 0.70 + rand_simple() * 0.25;
            },
            OceanTechnologyType::DataAnalytics => {
                self.innovation_index = 0.95 + rand_simple() * 0.05;
                self.technology_maturity = 0.85 + rand_simple() * 0.14;
                self.cost_effectiveness = 0.80 + rand_simple() * 0.18;
            },
            OceanTechnologyType::AutonomousPlatforms => {
                self.operational_reliability = 0.75 + rand_simple() * 0.22;
                self.innovation_index = 0.85 + rand_simple() * 0.14;
                self.technology_maturity = 0.70 + rand_simple() * 0.25;
            },
            OceanTechnologyType::EnergySystems => {
                self.technology_maturity = 0.70 + rand_simple() * 0.25;
                self.cost_effectiveness = 0.75 + rand_simple() * 0.22;
                self.innovation_index = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.cost_effectiveness == 0.0 {
            self.cost_effectiveness = (self.technology_maturity + self.operational_reliability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_underwater_robotics() {
        let mut system = OceanTechnologySystem::new(OceanTechnologyType::UnderwaterRobotics);
        system.analyze_system().unwrap();
        assert!(system.innovation_index > 0.7);
    }
}
