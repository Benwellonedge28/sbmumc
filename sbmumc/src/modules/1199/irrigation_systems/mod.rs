//! # SBMUMC Module 1199: Irrigation Systems
//!
//! Water delivery systems for agricultural production.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IrrigationMethod {
    Drip,
    Sprinkler,
    Flood,
    CenterPivot,
    Subsurface,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrrigationSystem {
    pub system_id: String,
    pub irrigation_method: IrrigationMethod,
    pub water_efficiency: f64,
    pub crop_yield: f64,
    pub energy_consumption: f64,
    pub infrastructure_cost: f64,
}

impl IrrigationSystem {
    pub fn new(irrigation_method: IrrigationMethod) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            irrigation_method,
            water_efficiency: 0.0,
            crop_yield: 0.0,
            energy_consumption: 0.0,
            infrastructure_cost: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.irrigation_method {
            IrrigationMethod::Drip => {
                self.water_efficiency = 0.90 + rand_simple() * 0.10;
                self.crop_yield = 0.85 + rand_simple() * 0.14;
                self.infrastructure_cost = 0.70 + rand_simple() * 0.25;
            },
            IrrigationMethod::Sprinkler => {
                self.water_efficiency = 0.75 + rand_simple() * 0.22;
                self.crop_yield = 0.80 + rand_simple() * 0.18;
                self.energy_consumption = 0.40 + rand_simple() * 0.30;
            },
            IrrigationMethod::Flood => {
                self.water_efficiency = 0.40 + rand_simple() * 0.30;
                self.crop_yield = 0.65 + rand_simple() * 0.30;
                self.infrastructure_cost = 0.20 + rand_simple() * 0.25;
            },
            IrrigationMethod::CenterPivot => {
                self.water_efficiency = 0.70 + rand_simple() * 0.25;
                self.crop_yield = 0.80 + rand_simple() * 0.18;
                self.energy_consumption = 0.50 + rand_simple() * 0.30;
            },
            IrrigationMethod::Subsurface => {
                self.water_efficiency = 0.95 + rand_simple() * 0.05;
                self.crop_yield = 0.85 + rand_simple() * 0.14;
                self.infrastructure_cost = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.energy_consumption == 0.0 {
            self.energy_consumption = 0.30 + rand_simple() * 0.35;
        }
        if self.infrastructure_cost == 0.0 {
            self.infrastructure_cost = 0.40 + rand_simple() * 0.40;
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
        let mut system = IrrigationSystem::new(IrrigationMethod::Drip);
        system.analyze_system().unwrap();
        assert!(system.water_efficiency > 0.7);
    }
}