//! # SBMUMC Module 1464: Applied Ethics
//!
//! Systems for applied ethics and practical moral problems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppliedEthicsDomain {
    MedicalEthics,
    BusinessEthics,
    MediaEthics,
    ProfessionalEthics,
    EngineeringEthics,
    ResearchEthics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedEthicsSystem {
    pub system_id: String,
    pub applied_ethics_domain: AppliedEthicsDomain,
    pub case_analysis: f64,
    pub principle_application: f64,
    pub stakeholder_consideration: f64,
    pub conflict_resolution: f64,
}

impl AppliedEthicsSystem {
    pub fn new(applied_ethics_domain: AppliedEthicsDomain) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            applied_ethics_domain,
            case_analysis: 0.0,
            principle_application: 0.0,
            stakeholder_consideration: 0.0,
            conflict_resolution: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.applied_ethics_domain {
            AppliedEthicsDomain::MedicalEthics => {
                self.case_analysis = 0.95 + rand_simple() * 0.05;
                self.principle_application = 0.90 + rand_simple() * 0.10;
                self.stakeholder_consideration = 0.85 + rand_simple() * 0.14;
            },
            AppliedEthicsDomain::BusinessEthics => {
                self.conflict_resolution = 0.95 + rand_simple() * 0.05;
                self.case_analysis = 0.90 + rand_simple() * 0.10;
                self.principle_application = 0.85 + rand_simple() * 0.14;
            },
            AppliedEthicsDomain::MediaEthics => {
                self.stakeholder_consideration = 0.95 + rand_simple() * 0.05;
                self.conflict_resolution = 0.90 + rand_simple() * 0.10;
                self.case_analysis = 0.85 + rand_simple() * 0.14;
            },
            AppliedEthicsDomain::ProfessionalEthics => {
                self.principle_application = 0.95 + rand_simple() * 0.05;
                self.stakeholder_consideration = 0.90 + rand_simple() * 0.10;
                self.conflict_resolution = 0.85 + rand_simple() * 0.14;
            },
            AppliedEthicsDomain::EngineeringEthics => {
                self.case_analysis = 0.95 + rand_simple() * 0.05;
                self.principle_application = 0.90 + rand_simple() * 0.10;
                self.conflict_resolution = 0.85 + rand_simple() * 0.14;
            },
            AppliedEthicsDomain::ResearchEthics => {
                self.stakeholder_consideration = 0.95 + rand_simple() * 0.05;
                self.case_analysis = 0.90 + rand_simple() * 0.10;
                self.principle_application = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.principle_application == 0.0 {
            self.principle_application = (self.case_analysis + self.stakeholder_consideration) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_medical() {
        let mut system = AppliedEthicsSystem::new(AppliedEthicsDomain::MedicalEthics);
        system.analyze_system().unwrap();
        assert!(system.case_analysis > 0.8);
    }
}