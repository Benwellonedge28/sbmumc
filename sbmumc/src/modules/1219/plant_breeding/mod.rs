//! # SBMUMC Module 1219: Plant Breeding
//!
//! Genetic improvement of crop plants for desired traits.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreedingMethod {
    Selection,
    Hybridization,
    MutationBreeding,
    GenomicSelection,
    GeneticEngineering,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlantBreedingSystem {
    pub system_id: String,
    pub breeding_method: BreedingMethod,
    pub genetic_gain: f64,
    pub trait_improvement: f64,
    pub development_time: f64,
    pub stability_score: f64,
}

impl PlantBreedingSystem {
    pub fn new(breeding_method: BreedingMethod) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            breeding_method,
            genetic_gain: 0.0,
            trait_improvement: 0.0,
            development_time: 0.0,
            stability_score: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.breeding_method {
            BreedingMethod::Selection => {
                self.genetic_gain = 0.60 + rand_simple() * 0.30;
                self.development_time = 0.80 + rand_simple() * 0.18;
            },
            BreedingMethod::Hybridization => {
                self.genetic_gain = 0.80 + rand_simple() * 0.18;
                self.trait_improvement = 0.75 + rand_simple() * 0.22;
                self.stability_score = 0.70 + rand_simple() * 0.25;
            },
            BreedingMethod::MutationBreeding => {
                self.genetic_gain = 0.50 + rand_simple() * 0.35;
                self.trait_improvement = 0.65 + rand_simple() * 0.30;
            },
            BreedingMethod::GenomicSelection => {
                self.genetic_gain = 0.85 + rand_simple() * 0.14;
                self.development_time = 0.85 + rand_simple() * 0.14;
                self.stability_score = 0.75 + rand_simple() * 0.22;
            },
            BreedingMethod::GeneticEngineering => {
                self.trait_improvement = 0.90 + rand_simple() * 0.10;
                self.development_time = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.stability_score == 0.0 {
            self.stability_score = (self.genetic_gain + self.trait_improvement) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_genomic_selection() {
        let mut system = PlantBreedingSystem::new(BreedingMethod::GenomicSelection);
        system.analyze_system().unwrap();
        assert!(system.genetic_gain > 0.6);
    }
}