//! # SBMUMC Module 880: Infrastructure Maintenance
//! 
//! Transportation infrastructure maintenance management.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

/// Maintenance condition states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionState {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

/// Pavement condition index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PavementCondition {
    pub pci: f64,
    pub roughness_iri: f64,
    pub rut_depth_mm: f64,
    pub cracking_percent: f64,
    pub surface_age_years: u32,
}

/// Maintenance treatment types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreatmentType {
    Preventive,
    MinorRehab,
    MajorRehab,
    Reconstruction,
}

/// Treatment decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatmentDecision {
    pub treatment_type: TreatmentType,
    pub estimated_cost: f64,
    pub expected_benefit: f64,
    pub timing_years: u32,
    pub life_extension_years: u32,
}

/// Bridge inspection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeInspection {
    pub inspection_id: String,
    pub bridge_id: String,
    pub inspection_date: u64,
    pub condition_rating: f64,
    pub nbi_rating: u32,
    pub deficiencies: Vec<String>,
    pub next_inspection_date: u64,
}

impl InfrastructureMaintenance {
    /// Create new maintenance system
    pub fn new() -> Self {
        Self
    }

    /// Calculate pavement condition index
    pub fn calculate_pci(&self, distress: &DistressData) -> Result<f64> {
        let deduct_value = distress.cracking_pct * 0.4 + 
                          distress.rutting_mm * 0.3 +
                          distress.roughness_iri * 0.3;
        let pci = 100.0 - deduct_value.min(100.0);
        Ok(pci)
    }

    /// Recommend treatment
    pub fn recommend_treatment(&self, condition: &PavementCondition) -> Result<TreatmentDecision> {
        let (treatment, cost_per_m2) = if condition.pci > 75.0 {
            (TreatmentType::Preventive, 5.0)
        } else if condition.pci > 60.0 {
            (TreatmentType::MinorRehab, 25.0)
        } else if condition.pci > 40.0 {
            (TreatmentType::MajorRehab, 60.0)
        } else {
            (TreatmentType::Reconstruction, 150.0)
        };
        
        Ok(TreatmentDecision {
            treatment_type: treatment,
            estimated_cost: cost_per_m2 * 1000.0,
            expected_benefit: condition.pci + 30.0,
            timing_years: 1,
            life_extension_years: 10,
        })
    }

    /// Schedule bridge inspection
    pub fn schedule_inspection(&self, last_inspection: u64, condition: f64) -> Result<u64> {
        let interval_months = if condition > 6.0 { 24 } else { 12 };
        Ok(last_inspection + interval_months as u64 * 30 * 24 * 3600)
    }
}

impl Default for InfrastructureMaintenance {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InfrastructureMaintenance;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistressData {
    pub cracking_pct: f64,
    pub rutting_mm: f64,
    pub roughness_iri: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pci_calculation() {
        let system = InfrastructureMaintenance::new();
        let distress = DistressData {
            cracking_pct: 10.0,
            rutting_mm: 5.0,
            roughness_iri: 3.0,
        };
        let pci = system.calculate_pci(&distress);
        assert!(pci.is_ok());
    }
}
