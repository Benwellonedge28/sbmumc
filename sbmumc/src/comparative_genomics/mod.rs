//! Comparative Genomics Module (736)
//!
//! Cross-species genome comparison, evolutionary analysis, and orthology.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparativeAnalysis {
    pub analysis_id: String,
    pub species1: String,
    pub species2: String,
    pub synteny_blocks: u32,
    pub ortholog_pairs: u32,
    pub divergence_time_mya: f64,
    pub conserved_regions_percent: f64,
    pub selection_pressure_dn_ds: f64,
}

impl ComparativeAnalysis {
    pub fn new(analysis_id: String, species1: String, species2: String) -> Self {
        Self {
            analysis_id,
            species1,
            species2,
            synteny_blocks: 0,
            ortholog_pairs: 0,
            divergence_time_mya: 0.0,
            conserved_regions_percent: 0.0,
            selection_pressure_dn_ds: 0.0,
        }
    }

    pub fn is_conserved(&self) -> bool {
        self.conserved_regions_percent > 70.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_comparative() {
        let analysis = ComparativeAnalysis::new("CG-001".into(), "Human".into(), "Mouse".into());
        assert_eq!(analysis.species1, "Human");
    }
}
