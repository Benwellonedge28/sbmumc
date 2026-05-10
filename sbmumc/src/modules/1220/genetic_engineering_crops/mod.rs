//! # SBMUMC Module 1220: Genetic Engineering Crops
//!
//! Development and application of genetically modified crops.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GMOCharacterType {
    PestResistance,
    HerbicideTolerance,
    DroughtTolerance,
    NutritionalEnhancement,
    YieldEnhancement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticEngineeringCropsSystem {
    pub system_id: String,
    pub character_type: GMOCharacterType,
    pub trait_efficacy: f64,
    pub yield_impact: f64,
    pub environmental_safety: f64,
    pub regulatory_acceptance: f64,
}

impl GeneticEngineeringCropsSystem {
    pub fn new(character_type: GMOCharacterType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            character_type,
            trait_efficacy: 0.0,
            yield_impact: 0.0,
            environmental_safety: 0.0,
            regulatory_acceptance: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.character_type {
            GMOCharacterType::PestResistance => {
                self.trait_efficacy = 0.90 + rand_simple() * 0.10;
                self.yield_impact = 0.20 + rand_simple() * 0.20;
                self.environmental_safety = 0.70 + rand_simple() * 0.25;
            },
            GMOCharacterType::HerbicideTolerance => {
                self.trait_efficacy = 0.85 + rand_simple() * 0.14;
                self.yield_impact = 0.15 + rand_simple() * 0.20;
                self.environmental_safety = 0.60 + rand_simple() * 0.35;
            },
            GMOCharacterType::DroughtTolerance => {
                self.trait_efficacy = 0.75 + rand_simple() * 0.22;
                self.yield_impact = 0.10 + rand_simple() * 0.15;
                self.environmental_safety = 0.80 + rand_simple() * 0.18;
            },
            GMOCharacterType::NutritionalEnhancement => {
                self.trait_efficacy = 0.80 + rand_simple() * 0.18;
                self.environmental_safety = 0.85 + rand_simple() * 0.14;
            },
            GMOCharacterType::YieldEnhancement => {
                self.trait_efficacy = 0.75 + rand_simple() * 0.22;
                self.yield_impact = 0.25 + rand_simple() * 0.20;
            },
        }

        if self.regulatory_acceptance == 0.0 {
            self.regulatory_acceptance = (self.trait_efficacy + self.environmental_safety) / 2.0 * (0.5 + rand_simple() * 0.4);
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
    fn test_pest_resistance() {
        let mut system = GeneticEngineeringCropsSystem::new(GMOCharacterType::PestResistance);
        system.analyze_system().unwrap();
        assert!(system.trait_efficacy > 0.7);
    }
}