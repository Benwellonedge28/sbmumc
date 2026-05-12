//! # SBMUMC Module 1339: Laboratory Design
//!
//! Systems for laboratory facility design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LaboratoryType {
    Research,
    Testing,
    Medical,
    Educational,
    Industrial,
    CleanRoom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaboratoryDesignSystem {
    pub system_id: String,
    pub laboratory_type: LaboratoryType,
    pub safety_compliance: f64,
    pub functional_flexibility: f64,
    pub equipment_integration: f64,
    pub research_productivity: f64,
}

impl LaboratoryDesignSystem {
    pub fn new(laboratory_type: LaboratoryType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            laboratory_type,
            safety_compliance: 0.0,
            functional_flexibility: 0.0,
            equipment_integration: 0.0,
            research_productivity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.laboratory_type {
            LaboratoryType::Research => {
                self.functional_flexibility = 0.95 + rand_simple() * 0.05;
                self.safety_compliance = 0.90 + rand_simple() * 0.10;
                self.research_productivity = 0.85 + rand_simple() * 0.14;
            },
            LaboratoryType::Testing => {
                self.safety_compliance = 0.95 + rand_simple() * 0.05;
                self.equipment_integration = 0.90 + rand_simple() * 0.10;
                self.functional_flexibility = 0.80 + rand_simple() * 0.18;
            },
            LaboratoryType::Medical => {
                self.safety_compliance = 0.95 + rand_simple() * 0.05;
                self.research_productivity = 0.90 + rand_simple() * 0.10;
                self.equipment_integration = 0.85 + rand_simple() * 0.14;
            },
            LaboratoryType::Educational => {
                self.functional_flexibility = 0.90 + rand_simple() * 0.10;
                self.research_productivity = 0.85 + rand_simple() * 0.14;
                self.safety_compliance = 0.85 + rand_simple() * 0.14;
            },
            LaboratoryType::Industrial => {
                self.equipment_integration = 0.95 + rand_simple() * 0.05;
                self.safety_compliance = 0.90 + rand_simple() * 0.10;
                self.functional_flexibility = 0.75 + rand_simple() * 0.22;
            },
            LaboratoryType::CleanRoom => {
                self.safety_compliance = 0.95 + rand_simple() * 0.05;
                self.functional_flexibility = 0.85 + rand_simple() * 0.14;
                self.equipment_integration = 0.90 + rand_simple() * 0.10;
            },
        }

        if self.research_productivity == 0.0 {
            self.research_productivity = (self.functional_flexibility + self.equipment_integration) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_research() {
        let mut system = LaboratoryDesignSystem::new(LaboratoryType::Research);
        system.analyze_system().unwrap();
        assert!(system.functional_flexibility > 0.8);
    }
}