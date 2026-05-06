//! Transplantation Module (721)
//!
//! Organ and tissue transplantation, HLA matching, and rejection prevention.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransplantType {
    Autograft,
    Allograft,
    Xenograft,
    Isograft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transplant {
    pub transplant_id: String,
    pub transplant_type: TransplantType,
    pub organ: String,
    pub donor_type: String,
    pub hla_mismatch: u8,
    pub rejection_risk: f64,
    pub immunosuppression_regimen: String,
    pub survival_rate_1yr: f64,
    pub survival_rate_5yr: f64,
}

impl Transplant {
    pub fn new(transplant_id: String, organ: String) -> Self {
        Self {
            transplant_id,
            transplant_type: TransplantType::Allograft,
            organ,
            donor_type: "Deceased".into(),
            hla_mismatch: 6,
            rejection_risk: 0.0,
            immunosuppression_regimen: "Tacrolimus + MMF".into(),
            survival_rate_1yr: 85.0,
            survival_rate_5yr: 70.0,
        }
    }

    pub fn calculate_match_score(&self) -> f64 {
        (6.0 - self.hla_mismatch as f64) / 6.0 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transplant() {
        let transplant = Transplant::new("TX-001".into(), "Kidney".into());
        assert_eq!(transplant.organ, "Kidney");
    }
}
