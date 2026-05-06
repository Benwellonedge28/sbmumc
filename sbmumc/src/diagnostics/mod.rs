//! Diagnostics Module (713)
//!
//! Medical diagnostics, disease detection, point-of-care testing, and diagnostic algorithms.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticType {
    Molecular,
    Serological,
    Imaging,
    Histopathological,
    Physiological,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticTest {
    pub test_id: String,
    pub diagnostic_type: DiagnosticType,
    pub target_condition: String,
    pub sensitivity: f64,
    pub specificity: f64,
    pub ppv: f64,
    pub npv: f64,
    pub turnaround_time_hours: f64,
    pub cost_usd: f64,
    pub clinical_validity: String,
}

impl DiagnosticTest {
    pub fn new(test_id: String, target_condition: String) -> Self {
        Self {
            test_id,
            diagnostic_type: DiagnosticType::Molecular,
            target_condition,
            sensitivity: 0.0,
            specificity: 0.0,
            ppv: 0.0,
            npv: 0.0,
            turnaround_time_hours: 24.0,
            cost_usd: 0.0,
            clinical_validity: "Validated".into(),
        }
    }

    pub fn likelihood_ratio(&self) -> f64 {
        if self.specificity == 0.0 || self.specificity >= 100.0 { return 0.0; }
        self.sensitivity / (100.0 - self.specificity)
    }

    pub fn diagnostic_odds_ratio(&self) -> f64 {
        let lr_pos = self.likelihood_ratio();
        let lr_neg = if self.sensitivity >= 100.0 || self.sensitivity == 0.0 { 1.0 } else {
            (100.0 - self.sensitivity) / self.specificity
        };
        if lr_neg == 0.0 { return f64::MAX; }
        lr_pos / lr_neg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_diagnostic() {
        let test = DiagnosticTest::new("DT-001".into(), "COVID-19".into());
        assert_eq!(test.target_condition, "COVID-19");
    }
}
