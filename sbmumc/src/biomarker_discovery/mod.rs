//! Biomarker Discovery Module (715)
//!
//! Biomarker identification, validation, and clinical utility assessment.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiomarkerType {
    Diagnostic,
    Prognostic,
    Predictive,
    Pharmacodynamic,
    Safety,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biomarker {
    pub biomarker_id: String,
    pub biomarker_type: BiomarkerType,
    pub target_disease: String,
    pub assay_method: String,
    pub sensitivity: f64,
    pub specificity: f64,
    pub cutoff_value: f64,
    pub sample_type: String,
    pub validation_status: String,
    pub clinical_utility: String,
}

impl Biomarker {
    pub fn new(biomarker_id: String, target_disease: String) -> Self {
        Self {
            biomarker_id,
            biomarker_type: BiomarkerType::Diagnostic,
            target_disease,
            assay_method: "ELISA".into(),
            sensitivity: 0.0,
            specificity: 0.0,
            cutoff_value: 0.0,
            sample_type: "Blood".into(),
            validation_status: "Discovery".into(),
            clinical_utility: "Unknown".into(),
        }
    }

    pub fn diagnostic_accuracy(&self) -> f64 {
        (self.sensitivity + self.specificity) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_biomarker() {
        let biomarker = Biomarker::new("BM-001".into(), "Cancer".into());
        assert_eq!(biomarker.target_disease, "Cancer");
    }
}
