//! # SBMUMC Module 1270: Ocean Acidification
//!
//! Systems for monitoring and addressing ocean acidification impacts.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AcidificationImpact {
    CoralCalcification,
    ShellfishProduction,
    MarineFoodWeb,
    CarbonCycle,
    Biodiversity,
    EcosystemStructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OceanAcidificationSystem {
    pub system_id: String,
    pub acidification_impact: AcidificationImpact,
    pub ph_decline: f64,
    pub species_sensitivity: f64,
    pub adaptive_capacity: f64,
    pub ecosystem_vulnerability: f64,
}

impl OceanAcidificationSystem {
    pub fn new(acidification_impact: AcidificationImpact) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            acidification_impact,
            ph_decline: 0.0,
            species_sensitivity: 0.0,
            adaptive_capacity: 0.0,
            ecosystem_vulnerability: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.acidification_impact {
            AcidificationImpact::CoralCalcification => {
                self.species_sensitivity = 0.90 + rand_simple() * 0.10;
                self.ecosystem_vulnerability = 0.85 + rand_simple() * 0.14;
            },
            AcidificationImpact::ShellfishProduction => {
                self.species_sensitivity = 0.85 + rand_simple() * 0.14;
                self.adaptive_capacity = 0.40 + rand_simple() * 0.35;
            },
            AcidificationImpact::MarineFoodWeb => {
                self.ecosystem_vulnerability = 0.80 + rand_simple() * 0.18;
                self.species_sensitivity = 0.70 + rand_simple() * 0.25;
            },
            AcidificationImpact::CarbonCycle => {
                self.adaptive_capacity = 0.60 + rand_simple() * 0.35;
                self.ecosystem_vulnerability = 0.55 + rand_simple() * 0.40;
            },
            AcidificationImpact::Biodiversity => {
                self.species_sensitivity = 0.75 + rand_simple() * 0.22;
                self.ecosystem_vulnerability = 0.70 + rand_simple() * 0.25;
            },
            AcidificationImpact::EcosystemStructure => {
                self.ecosystem_vulnerability = 0.75 + rand_simple() * 0.22;
                self.adaptive_capacity = 0.50 + rand_simple() * 0.40;
                self.species_sensitivity = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.ph_decline == 0.0 {
            self.ph_decline = 0.8 + rand_simple() * 0.2;
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
    fn test_coral_calcification() {
        let mut system = OceanAcidificationSystem::new(AcidificationImpact::CoralCalcification);
        system.analyze_system().unwrap();
        assert!(system.species_sensitivity > 0.7);
    }
}
