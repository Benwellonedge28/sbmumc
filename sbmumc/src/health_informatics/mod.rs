//! Health Informatics Module (728)
//!
//! Healthcare information systems, EHR, and clinical data analytics.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InformaticsSystem {
    EHR,
    LIS,
    RIS,
    PACS,
    CPOE,
    CDSS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthInformationSystem {
    pub system_id: String,
    pub system_type: InformaticsSystem,
    pub vendor: String,
    pub interoperability_standard: String,
    pub data_standards_compliance: f64,
    pub user_satisfaction_score: f64,
    pub implementation_status: String,
    pub security_certification: String,
}

impl HealthInformationSystem {
    pub fn new(system_id: String) -> Self {
        Self {
            system_id,
            system_type: InformaticsSystem::EHR,
            vendor: "Unknown".into(),
            interoperability_standard: "HL7".into(),
            data_standards_compliance: 0.0,
            user_satisfaction_score: 0.0,
            implementation_status: "Planned".into(),
            security_certification: "SOC2".into(),
        }
    }

    pub fn interoperability_score(&self) -> f64 {
        self.data_standards_compliance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_health_informatics() {
        let system = HealthInformationSystem::new("HI-001".into());
        assert_eq!(system.system_id, "HI-001");
    }
}
