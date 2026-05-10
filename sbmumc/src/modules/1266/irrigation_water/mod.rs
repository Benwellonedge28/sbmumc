//! # SBMUMC Module 1266: Irrigation Water
//!
//! Systems for managing irrigation water resources in agriculture.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrrigationMethod {
    DripIrrigation,
    SprinklerIrrigation,
    FloodIrrigation,
    FurrowIrrigation,
    SubsurfaceIrrigation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrrigationWaterSystem {
    pub system_id: String,
    pub irrigation_method: IrrigationMethod,
    pub water_efficiency: f64,
    pub crop_yield_impact: f64,
    pub soil_health: f64,
    pub energy_consumption: f64,
}

impl IrrigationWaterSystem {
    pub fn new(irrigation_method: IrrigationMethod) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            irrigation_method,
            water_efficiency: 0.0,
            crop_yield_impact: 0.0,
            soil_health: 0.0,
            energy_consumption: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.irrigation_method {
            IrrigationMethod::DripIrrigation => {
                self.water_efficiency = 0.95 + rand_simple() * 0.05;
                self.soil_health = 0.85 + rand_simple() * 0.14;
                self.energy_consumption = 0.40 + rand_simple() * 0.35;
            },
            IrrigationMethod::SprinklerIrrigation => {
                self.water_efficiency = 0.75 + rand_simple() * 0.22;
                self.crop_yield_impact = 0.80 + rand_simple() * 0.18;
                self.energy_consumption = 0.60 + rand_simple() * 0.35;
            },
            IrrigationMethod::FloodIrrigation => {
                self.crop_yield_impact = 0.70 + rand_simple() * 0.25;
                self.energy_consumption = 0.30 + rand_simple() * 0.35;
                self.water_efficiency = 0.45 + rand_simple() * 0.40;
            },
            IrrigationMethod::FurrowIrrigation => {
                self.water_efficiency = 0.55 + rand_simple() * 0.40;
                self.crop_yield_impact = 0.65 + rand_simple() * 0.30;
                self.soil_health = 0.60 + rand_simple() * 0.35;
            },
            IrrigationMethod::SubsurfaceIrrigation => {
                self.water_efficiency = 0.90 + rand_simple() * 0.10;
                self.soil_health = 0.90 + rand_simple() * 0.10;
                self.energy_consumption = 0.50 + rand_simple() * 0.35;
            },
        }

        if self.energy_consumption == 0.0 {
            self.energy_consumption = (1.0 - self.water_efficiency) * (0.5 + rand_simple() * 0.5);
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
    fn test_drip_irrigation() {
        let mut system = IrrigationWaterSystem::new(IrrigationMethod::DripIrrigation);
        system.analyze_system().unwrap();
        assert!(system.water_efficiency > 0.8);
    }
}
