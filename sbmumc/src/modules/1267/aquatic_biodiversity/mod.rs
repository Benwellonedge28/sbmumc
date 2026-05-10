//! # SBMUMC Module 1267: Aquatic Biodiversity
//!
//! Conservation and management of aquatic species diversity.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AquaticHabitatType {
    Rivers,
    Lakes,
    Wetlands,
    Estuaries,
    CoralReefs,
    DeepSea,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquaticBiodiversitySystem {
    pub system_id: String,
    pub habitat_type: AquaticHabitatType,
    pub species_diversity: f64,
    pub habitat_health: f64,
    pub conservation_effectiveness: f64,
    pub ecosystem_resilience: f64,
}

impl AquaticBiodiversitySystem {
    pub fn new(habitat_type: AquaticHabitatType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            habitat_type,
            species_diversity: 0.0,
            habitat_health: 0.0,
            conservation_effectiveness: 0.0,
            ecosystem_resilience: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.habitat_type {
            AquaticHabitatType::Rivers => {
                self.species_diversity = 0.70 + rand_simple() * 0.25;
                self.habitat_health = 0.65 + rand_simple() * 0.30;
                self.ecosystem_resilience = 0.60 + rand_simple() * 0.35;
            },
            AquaticHabitatType::Lakes => {
                self.species_diversity = 0.60 + rand_simple() * 0.35;
                self.habitat_health = 0.70 + rand_simple() * 0.25;
                self.conservation_effectiveness = 0.75 + rand_simple() * 0.22;
            },
            AquaticHabitatType::Wetlands => {
                self.ecosystem_resilience = 0.85 + rand_simple() * 0.14;
                self.species_diversity = 0.80 + rand_simple() * 0.18;
                self.habitat_health = 0.75 + rand_simple() * 0.22;
            },
            AquaticHabitatType::Estuaries => {
                self.species_diversity = 0.85 + rand_simple() * 0.14;
                self.conservation_effectiveness = 0.70 + rand_simple() * 0.25;
                self.habitat_health = 0.65 + rand_simple() * 0.30;
            },
            AquaticHabitatType::CoralReefs => {
                self.species_diversity = 0.90 + rand_simple() * 0.10;
                self.habitat_health = 0.55 + rand_simple() * 0.40;
                self.ecosystem_resilience = 0.50 + rand_simple() * 0.40;
            },
            AquaticHabitatType::DeepSea => {
                self.habitat_health = 0.75 + rand_simple() * 0.22;
                self.ecosystem_resilience = 0.70 + rand_simple() * 0.25;
                self.species_diversity = 0.55 + rand_simple() * 0.40;
            },
        }

        if self.conservation_effectiveness == 0.0 {
            self.conservation_effectiveness = (self.species_diversity + self.habitat_health) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_wetlands() {
        let mut system = AquaticBiodiversitySystem::new(AquaticHabitatType::Wetlands);
        system.analyze_system().unwrap();
        assert!(system.ecosystem_resilience > 0.7);
    }
}
