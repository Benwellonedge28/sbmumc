//! # SBMUMC Module 1279: Climate Ocean
//!
//! Systems for understanding ocean-climate interactions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OceanClimateProcess {
    ThermohalineCirculation,
    ElNino,
    CarbonSequestration,
    HeatAbsorption,
    SeaIceFormation,
    Upwelling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateOceanSystem {
    pub system_id: String,
    pub climate_process: OceanClimateProcess,
    pub climate_influence: f64,
    pub change_rate: f64,
    pub feedback_mechanism: f64,
    pub prediction_accuracy: f64,
}

impl ClimateOceanSystem {
    pub fn new(climate_process: OceanClimateProcess) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            climate_process,
            climate_influence: 0.0,
            change_rate: 0.0,
            feedback_mechanism: 0.0,
            prediction_accuracy: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.climate_process {
            OceanClimateProcess::ThermohalineCirculation => {
                self.climate_influence = 0.90 + rand_simple() * 0.10;
                self.feedback_mechanism = 0.85 + rand_simple() * 0.14;
                self.prediction_accuracy = 0.70 + rand_simple() * 0.25;
            },
            OceanClimateProcess::ElNino => {
                self.climate_influence = 0.85 + rand_simple() * 0.14;
                self.prediction_accuracy = 0.80 + rand_simple() * 0.18;
                self.change_rate = 0.60 + rand_simple() * 0.35;
            },
            OceanClimateProcess::CarbonSequestration => {
                self.climate_influence = 0.80 + rand_simple() * 0.18;
                self.feedback_mechanism = 0.90 + rand_simple() * 0.10;
                self.change_rate = 0.55 + rand_simple() * 0.40;
            },
            OceanClimateProcess::HeatAbsorption => {
                self.climate_influence = 0.95 + rand_simple() * 0.05;
                self.change_rate = 0.70 + rand_simple() * 0.25;
                self.feedback_mechanism = 0.80 + rand_simple() * 0.18;
            },
            OceanClimateProcess::SeaIceFormation => {
                self.climate_influence = 0.85 + rand_simple() * 0.14;
                self.prediction_accuracy = 0.75 + rand_simple() * 0.22;
                self.feedback_mechanism = 0.90 + rand_simple() * 0.10;
            },
            OceanClimateProcess::Upwelling => {
                self.climate_influence = 0.75 + rand_simple() * 0.22;
                self.feedback_mechanism = 0.70 + rand_simple() * 0.25;
                self.prediction_accuracy = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.prediction_accuracy == 0.0 {
            self.prediction_accuracy = (self.climate_influence + self.feedback_mechanism) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_heat_absorption() {
        let mut system = ClimateOceanSystem::new(OceanClimateProcess::HeatAbsorption);
        system.analyze_system().unwrap();
        assert!(system.climate_influence > 0.8);
    }
}
