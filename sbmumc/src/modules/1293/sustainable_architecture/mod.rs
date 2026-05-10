//! # SBMUMC Module 1293: Sustainable Architecture
//!
//! Systems for environmentally sustainable building design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SustainabilityFeature {
    PassiveSolar,
    NaturalVentilation,
    RainwaterHarvesting,
    GreenRoofs,
    RecycledMaterials,
    NetZeroDesign,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SustainableArchitectureSystem {
    pub system_id: String,
    pub sustainability_feature: SustainabilityFeature,
    pub energy_efficiency: f64,
    pub carbon_footprint: f64,
    pub cost_effectiveness: f64,
    pub occupant_comfort: f64,
}

impl SustainableArchitectureSystem {
    pub fn new(sustainability_feature: SustainabilityFeature) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            sustainability_feature,
            energy_efficiency: 0.0,
            carbon_footprint: 0.0,
            cost_effectiveness: 0.0,
            occupant_comfort: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.sustainability_feature {
            SustainabilityFeature::PassiveSolar => {
                self.energy_efficiency = 0.85 + rand_simple() * 0.14;
                self.occupant_comfort = 0.90 + rand_simple() * 0.10;
                self.carbon_footprint = 0.75 + rand_simple() * 0.22;
            },
            SustainabilityFeature::NaturalVentilation => {
                self.energy_efficiency = 0.80 + rand_simple() * 0.18;
                self.occupant_comfort = 0.85 + rand_simple() * 0.14;
                self.cost_effectiveness = 0.75 + rand_simple() * 0.22;
            },
            SustainabilityFeature::RainwaterHarvesting => {
                self.carbon_footprint = 0.70 + rand_simple() * 0.25;
                self.cost_effectiveness = 0.80 + rand_simple() * 0.18;
                self.energy_efficiency = 0.65 + rand_simple() * 0.30;
            },
            SustainabilityFeature::GreenRoofs => {
                self.occupant_comfort = 0.80 + rand_simple() * 0.18;
                self.carbon_footprint = 0.75 + rand_simple() * 0.22;
                self.energy_efficiency = 0.70 + rand_simple() * 0.25;
            },
            SustainabilityFeature::RecycledMaterials => {
                self.carbon_footprint = 0.85 + rand_simple() * 0.14;
                self.cost_effectiveness = 0.70 + rand_simple() * 0.25;
                self.occupant_comfort = 0.60 + rand_simple() * 0.35;
            },
            SustainabilityFeature::NetZeroDesign => {
                self.carbon_footprint = 0.95 + rand_simple() * 0.05;
                self.energy_efficiency = 0.90 + rand_simple() * 0.10;
                self.cost_effectiveness = 0.55 + rand_simple() * 0.40;
            },
        }

        if self.cost_effectiveness == 0.0 {
            self.cost_effectiveness = (self.energy_efficiency + self.occupant_comfort) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_passive_solar() {
        let mut system = SustainableArchitectureSystem::new(SustainabilityFeature::PassiveSolar);
        system.analyze_system().unwrap();
        assert!(system.energy_efficiency > 0.7);
    }
}
