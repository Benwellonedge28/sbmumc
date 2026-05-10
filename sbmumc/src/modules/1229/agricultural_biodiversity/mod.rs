//! # SBMUMC Module 1229: Agricultural Biodiversity
//!
//! Diversity of species and varieties in agricultural systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiodiversityComponent {
    CropDiversity,
    LivestockDiversity,
    PollinatorHealth,
    SoilMicrobiome,
    WildRelatives,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalBiodiversitySystem {
    pub system_id: String,
    pub biodiversity_component: BiodiversityComponent,
    pub diversity_index: f64,
    pub ecosystem_service: f64,
    pub resilience_enhancement: f64,
    pub cultural_value: f64,
}

impl AgriculturalBiodiversitySystem {
    pub fn new(biodiversity_component: BiodiversityComponent) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            biodiversity_component,
            diversity_index: 0.0,
            ecosystem_service: 0.0,
            resilience_enhancement: 0.0,
            cultural_value: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.biodiversity_component {
            BiodiversityComponent::CropDiversity => {
                self.diversity_index = 0.85 + rand_simple() * 0.14;
                self.resilience_enhancement = 0.80 + rand_simple() * 0.18;
            },
            BiodiversityComponent::LivestockDiversity => {
                self.diversity_index = 0.75 + rand_simple() * 0.22;
                self.cultural_value = 0.80 + rand_simple() * 0.18;
            },
            BiodiversityComponent::PollinatorHealth => {
                self.ecosystem_service = 0.90 + rand_simple() * 0.10;
                self.resilience_enhancement = 0.85 + rand_simple() * 0.14;
            },
            BiodiversityComponent::SoilMicrobiome => {
                self.ecosystem_service = 0.85 + rand_simple() * 0.14;
                self.diversity_index = 0.80 + rand_simple() * 0.18;
            },
            BiodiversityComponent::WildRelatives => {
                self.diversity_index = 0.80 + rand_simple() * 0.18;
                self.cultural_value = 0.70 + rand_simple() * 0.25;
                self.resilience_enhancement = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.cultural_value == 0.0 {
            self.cultural_value = (self.diversity_index + self.ecosystem_service) / 2.0 * (0.5 + rand_simple() * 0.4);
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
    fn test_pollinator_health() {
        let mut system = AgriculturalBiodiversitySystem::new(BiodiversityComponent::PollinatorHealth);
        system.analyze_system().unwrap();
        assert!(system.ecosystem_service > 0.7);
    }
}