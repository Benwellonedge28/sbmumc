//! # SBMUMC Module 1316: Construction Management
//!
//! Systems for managing construction projects.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstructionPhase {
    PreConstruction,
    Procurement,
    SitePreparation,
    StructuralWork,
    EnvelopeInstallation,
    InteriorFinishing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionManagementSystem {
    pub system_id: String,
    pub construction_phase: ConstructionPhase,
    pub schedule_adherence: f64,
    pub budget_control: f64,
    pub quality_assurance: f64,
    pub safety_compliance: f64,
}

impl ConstructionManagementSystem {
    pub fn new(construction_phase: ConstructionPhase) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            construction_phase,
            schedule_adherence: 0.0,
            budget_control: 0.0,
            quality_assurance: 0.0,
            safety_compliance: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.construction_phase {
            ConstructionPhase::PreConstruction => {
                self.schedule_adherence = 0.85 + rand_simple() * 0.14;
                self.budget_control = 0.90 + rand_simple() * 0.10;
                self.quality_assurance = 0.80 + rand_simple() * 0.18;
            },
            ConstructionPhase::Procurement => {
                self.budget_control = 0.90 + rand_simple() * 0.10;
                self.schedule_adherence = 0.85 + rand_simple() * 0.14;
                self.quality_assurance = 0.75 + rand_simple() * 0.22;
            },
            ConstructionPhase::SitePreparation => {
                self.safety_compliance = 0.90 + rand_simple() * 0.10;
                self.schedule_adherence = 0.85 + rand_simple() * 0.14;
                self.quality_assurance = 0.80 + rand_simple() * 0.18;
            },
            ConstructionPhase::StructuralWork => {
                self.safety_compliance = 0.95 + rand_simple() * 0.05;
                self.quality_assurance = 0.90 + rand_simple() * 0.10;
                self.schedule_adherence = 0.85 + rand_simple() * 0.14;
            },
            ConstructionPhase::EnvelopeInstallation => {
                self.quality_assurance = 0.85 + rand_simple() * 0.14;
                self.schedule_adherence = 0.80 + rand_simple() * 0.18;
                self.budget_control = 0.75 + rand_simple() * 0.22;
            },
            ConstructionPhase::InteriorFinishing => {
                self.quality_assurance = 0.90 + rand_simple() * 0.10;
                self.safety_compliance = 0.85 + rand_simple() * 0.14;
                self.schedule_adherence = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.safety_compliance == 0.0 {
            self.safety_compliance = (self.schedule_adherence + self.quality_assurance) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_structural_work() {
        let mut system = ConstructionManagementSystem::new(ConstructionPhase::StructuralWork);
        system.analyze_system().unwrap();
        assert!(system.safety_compliance > 0.8);
    }
}
