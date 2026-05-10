//! # SBMUMC Module 1230: Soil Conservation
//!
//! Practices to prevent soil degradation and erosion.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConservationTechnique {
    Terracing,
    ContourFarming,
    StripCropping,
    Windbreaks,
    ErosionControl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilConservationSystem {
    pub system_id: String,
    pub conservation_technique: ConservationTechnique,
    pub erosion_control: f64,
    pub soil_loss_prevention: f64,
    pub water_retention: f64,
    pub implementation_cost: f64,
}

impl SoilConservationSystem {
    pub fn new(conservation_technique: ConservationTechnique) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            conservation_technique,
            erosion_control: 0.0,
            soil_loss_prevention: 0.0,
            water_retention: 0.0,
            implementation_cost: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.conservation_technique {
            ConservationTechnique::Terracing => {
                self.erosion_control = 0.90 + rand_simple() * 0.10;
                self.soil_loss_prevention = 0.85 + rand_simple() * 0.14;
                self.implementation_cost = 0.70 + rand_simple() * 0.25;
            },
            ConservationTechnique::ContourFarming => {
                self.erosion_control = 0.75 + rand_simple() * 0.22;
                self.soil_loss_prevention = 0.70 + rand_simple() * 0.25;
                self.implementation_cost = 0.20 + rand_simple() * 0.25;
            },
            ConservationTechnique::StripCropping => {
                self.erosion_control = 0.80 + rand_simple() * 0.18;
                self.water_retention = 0.75 + rand_simple() * 0.22;
                self.implementation_cost = 0.30 + rand_simple() * 0.30;
            },
            ConservationTechnique::Windbreaks => {
                self.erosion_control = 0.85 + rand_simple() * 0.14;
                self.water_retention = 0.70 + rand_simple() * 0.25;
                self.implementation_cost = 0.40 + rand_simple() * 0.35;
            },
            ConservationTechnique::ErosionControl => {
                self.soil_loss_prevention = 0.85 + rand_simple() * 0.14;
                self.erosion_control = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.water_retention == 0.0 {
            self.water_retention = self.erosion_control * (0.6 + rand_simple() * 0.3);
        }
        if self.implementation_cost == 0.0 {
            self.implementation_cost = 0.35 + rand_simple() * 0.40;
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
    fn test_terracing() {
        let mut system = SoilConservationSystem::new(ConservationTechnique::Terracing);
        system.analyze_system().unwrap();
        assert!(system.erosion_control > 0.7);
    }
}