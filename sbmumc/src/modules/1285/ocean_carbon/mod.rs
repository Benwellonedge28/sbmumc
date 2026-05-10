//! # SBMUMC Module 1285: Ocean Carbon
//!
//! Systems for understanding ocean carbon cycling.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CarbonProcess {
    BiologicalPump,
    SolubilityPump,
    CoastalBlueCarbon,
    DeepOceanStorage,
    AirSeaExchange,
    CarbonateSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OceanCarbonSystem {
    pub system_id: String,
    pub carbon_process: CarbonProcess,
    pub carbon_flux: f64,
    pub sequestration_efficiency: f64,
    pub climate_feedback: f64,
    pub measurement_accuracy: f64,
}

impl OceanCarbonSystem {
    pub fn new(carbon_process: CarbonProcess) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            carbon_process,
            carbon_flux: 0.0,
            sequestration_efficiency: 0.0,
            climate_feedback: 0.0,
            measurement_accuracy: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.carbon_process {
            CarbonProcess::BiologicalPump => {
                self.carbon_flux = 0.85 + rand_simple() * 0.14;
                self.sequestration_efficiency = 0.80 + rand_simple() * 0.18;
                self.measurement_accuracy = 0.75 + rand_simple() * 0.22;
            },
            CarbonProcess::SolubilityPump => {
                self.carbon_flux = 0.80 + rand_simple() * 0.18;
                self.sequestration_efficiency = 0.85 + rand_simple() * 0.14;
                self.climate_feedback = 0.80 + rand_simple() * 0.18;
            },
            CarbonProcess::CoastalBlueCarbon => {
                self.sequestration_efficiency = 0.90 + rand_simple() * 0.10;
                self.carbon_flux = 0.70 + rand_simple() * 0.25;
                self.climate_feedback = 0.75 + rand_simple() * 0.22;
            },
            CarbonProcess::DeepOceanStorage => {
                self.sequestration_efficiency = 0.95 + rand_simple() * 0.05;
                self.carbon_flux = 0.75 + rand_simple() * 0.22;
                self.measurement_accuracy = 0.65 + rand_simple() * 0.30;
            },
            CarbonProcess::AirSeaExchange => {
                self.carbon_flux = 0.80 + rand_simple() * 0.18;
                self.climate_feedback = 0.85 + rand_simple() * 0.14;
                self.measurement_accuracy = 0.80 + rand_simple() * 0.18;
            },
            CarbonProcess::CarbonateSystem => {
                self.measurement_accuracy = 0.90 + rand_simple() * 0.10;
                self.carbon_flux = 0.75 + rand_simple() * 0.22;
                self.climate_feedback = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.climate_feedback == 0.0 {
            self.climate_feedback = (self.carbon_flux + self.sequestration_efficiency) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_coastal_blue_carbon() {
        let mut system = OceanCarbonSystem::new(CarbonProcess::CoastalBlueCarbon);
        system.analyze_system().unwrap();
        assert!(system.sequestration_efficiency > 0.7);
    }
}
