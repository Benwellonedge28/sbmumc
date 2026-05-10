//! # SBMUMC Module 1320: Architectural Profession
//!
//! Systems for architectural practice and profession.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProfessionalActivity {
    DesignServices,
    ProjectManagement,
    ContractAdministration,
    SiteObservation,
    FeasibilityStudies,
    ConservationAdvice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitecturalProfessionSystem {
    pub system_id: String,
    pub professional_activity: ProfessionalActivity,
    pub service_quality: f64,
    pub professional_ethics: f64,
    pub client_satisfaction: f64,
    pub market_positioning: f64,
}

impl ArchitecturalProfessionSystem {
    pub fn new(professional_activity: ProfessionalActivity) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            professional_activity,
            service_quality: 0.0,
            professional_ethics: 0.0,
            client_satisfaction: 0.0,
            market_positioning: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.professional_activity {
            ProfessionalActivity::DesignServices => {
                self.service_quality = 0.95 + rand_simple() * 0.05;
                self.client_satisfaction = 0.90 + rand_simple() * 0.10;
                self.professional_ethics = 0.85 + rand_simple() * 0.14;
            },
            ProfessionalActivity::ProjectManagement => {
                self.client_satisfaction = 0.90 + rand_simple() * 0.10;
                self.service_quality = 0.85 + rand_simple() * 0.14;
                self.market_positioning = 0.80 + rand_simple() * 0.18;
            },
            ProfessionalActivity::ContractAdministration => {
                self.professional_ethics = 0.95 + rand_simple() * 0.05;
                self.service_quality = 0.90 + rand_simple() * 0.10;
                self.client_satisfaction = 0.85 + rand_simple() * 0.14;
            },
            ProfessionalActivity::SiteObservation => {
                self.service_quality = 0.90 + rand_simple() * 0.10;
                self.professional_ethics = 0.90 + rand_simple() * 0.10;
                self.client_satisfaction = 0.85 + rand_simple() * 0.14;
            },
            ProfessionalActivity::FeasibilityStudies => {
                self.client_satisfaction = 0.85 + rand_simple() * 0.14;
                self.service_quality = 0.85 + rand_simple() * 0.14;
                self.market_positioning = 0.80 + rand_simple() * 0.18;
            },
            ProfessionalActivity::ConservationAdvice => {
                self.professional_ethics = 0.90 + rand_simple() * 0.10;
                self.service_quality = 0.85 + rand_simple() * 0.14;
                self.client_satisfaction = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.market_positioning == 0.0 {
            self.market_positioning = (self.service_quality + self.client_satisfaction) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_design_services() {
        let mut system = ArchitecturalProfessionSystem::new(ProfessionalActivity::DesignServices);
        system.analyze_system().unwrap();
        assert!(system.service_quality > 0.8);
    }
}
