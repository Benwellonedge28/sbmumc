//! # SBMUMC Module 1319: Climate Responsive
//!
//! Systems for climate-responsive building design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClimateStrategy {
    PassiveCooling,
    PassiveHeating,
    NaturalVentilation,
    ThermalMass,
    Shading,
    WindSheltering,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateResponsiveSystem {
    pub system_id: String,
    pub climate_strategy: ClimateStrategy,
    pub climate_adaptation: f64,
    pub energy_reduction: f64,
    pub occupant_comfort: f64,
    pub cost_effectiveness: f64,
}

impl ClimateResponsiveSystem {
    pub fn new(climate_strategy: ClimateStrategy) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            climate_strategy,
            climate_adaptation: 0.0,
            energy_reduction: 0.0,
            occupant_comfort: 0.0,
            cost_effectiveness: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.climate_strategy {
            ClimateStrategy::PassiveCooling => {
                self.energy_reduction = 0.90 + rand_simple() * 0.10;
                self.occupant_comfort = 0.85 + rand_simple() * 0.14;
                self.climate_adaptation = 0.85 + rand_simple() * 0.14;
            },
            ClimateStrategy::PassiveHeating => {
                self.energy_reduction = 0.85 + rand_simple() * 0.14;
                self.climate_adaptation = 0.90 + rand_simple() * 0.10;
                self.cost_effectiveness = 0.80 + rand_simple() * 0.18;
            },
            ClimateStrategy::NaturalVentilation => {
                self.occupant_comfort = 0.90 + rand_simple() * 0.10;
                self.energy_reduction = 0.85 + rand_simple() * 0.14;
                self.cost_effectiveness = 0.85 + rand_simple() * 0.14;
            },
            ClimateStrategy::ThermalMass => {
                self.climate_adaptation = 0.90 + rand_simple() * 0.10;
                self.energy_reduction = 0.85 + rand_simple() * 0.14;
                self.occupant_comfort = 0.80 + rand_simple() * 0.18;
            },
            ClimateStrategy::Shading => {
                self.energy_reduction = 0.85 + rand_simple() * 0.14;
                self.occupant_comfort = 0.90 + rand_simple() * 0.10;
                self.climate_adaptation = 0.85 + rand_simple() * 0.14;
            },
            ClimateStrategy::WindSheltering => {
                self.climate_adaptation = 0.85 + rand_simple() * 0.14;
                self.occupant_comfort = 0.80 + rand_simple() * 0.18;
                self.cost_effectiveness = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.cost_effectiveness == 0.0 {
            self.cost_effectiveness = (self.energy_reduction + self.occupant_comfort) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_natural_ventilation() {
        let mut system = ClimateResponsiveSystem::new(ClimateStrategy::NaturalVentilation);
        system.analyze_system().unwrap();
        assert!(system.occupant_comfort > 0.7);
    }
}
