//! Population Genetics Module (737)
//!
//! Genetic variation, population structure, and evolutionary genomics.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationStudy {
    pub study_id: String,
    pub population: String,
    pub sample_size: u32,
    pub snps_identified: u32,
    pub nucleotide_diversity_pi: f64,
    pub Tajimas_D: f64,
    pub Fst_neighbor_population: f64,
    pub selection_signatures: u32,
}

impl PopulationStudy {
    pub fn new(study_id: String, population: String) -> Self {
        Self {
            study_id,
            population,
            sample_size: 0,
            snps_identified: 0,
            nucleotide_diversity_pi: 0.0,
            Tajimas_D: 0.0,
            Fst_neighbor_population: 0.0,
            selection_signatures: 0,
        }
    }

    pub fn demographic_history(&self) -> String {
        if self.Tajimas_D > 2.0 {
            "Population bottleneck".into()
        } else if self.Tajimas_D < -2.0 {
            "Population expansion".into()
        } else {
            "Stable".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_population() {
        let study = PopulationStudy::new("POP-001".into(), "European".into());
        assert_eq!(study.population, "European");
    }
}
