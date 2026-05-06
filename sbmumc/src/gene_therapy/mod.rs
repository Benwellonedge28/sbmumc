//! Gene Therapy Module (698)
//!
//! Therapeutic gene delivery, viral vectors, and genetic treatment protocols.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VectorType {
    AAV,
    Lentivirus,
    Adenovirus,
    Retrovirus,
    NonViral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneTherapy {
    pub therapy_id: String,
    pub target_disease: String,
    pub vector_type: VectorType,
    pub therapeutic_gene: String,
    pub delivery_route: String,
    pub expression_duration: String,
    pub clinical_phase: u8,
    pub efficacy_rate: f64,
    pub safety_score: f64,
}

impl GeneTherapy {
    pub fn new(therapy_id: String, target_disease: String) -> Self {
        Self {
            therapy_id,
            target_disease,
            vector_type: VectorType::AAV,
            therapeutic_gene: String::new(),
            delivery_route: "IV".into(),
            expression_duration: "Transient".into(),
            clinical_phase: 0,
            efficacy_rate: 0.0,
            safety_score: 0.0,
        }
    }

    pub fn approval_likelihood(&self) -> f64 {
        (self.efficacy_rate + self.safety_score) / 2.0 * (self.clinical_phase as f64 / 3.0)
    }

    pub fn is_ready_for_clinic(&self) -> bool {
        self.clinical_phase >= 2 && self.safety_score > 80.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gene_therapy() {
        let therapy = GeneTherapy::new("GT-001".into(), "SMA".into());
        assert_eq!(therapy.target_disease, "SMA");
    }
}
