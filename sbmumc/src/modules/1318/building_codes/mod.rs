//! # SBMUMC Module 1318: Building Codes
//!
//! Systems for understanding building regulations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeCategory {
    FireSafety,
    Structural,
    Electrical,
    Plumbing,
    Energy,
    Accessibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingCodesSystem {
    pub system_id: String,
    pub code_category: CodeCategory,
    pub code_compliance: f64,
    pub regulatory_coverage: f64,
    pub enforcement_rate: f64,
    pub updating_frequency: f64,
}

impl BuildingCodesSystem {
    pub fn new(code_category: CodeCategory) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            code_category,
            code_compliance: 0.0,
            regulatory_coverage: 0.0,
            enforcement_rate: 0.0,
            updating_frequency: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.code_category {
            CodeCategory::FireSafety => {
                self.code_compliance = 0.95 + rand_simple() * 0.05;
                self.enforcement_rate = 0.90 + rand_simple() * 0.10;
                self.regulatory_coverage = 0.85 + rand_simple() * 0.14;
            },
            CodeCategory::Structural => {
                self.regulatory_coverage = 0.95 + rand_simple() * 0.05;
                self.code_compliance = 0.90 + rand_simple() * 0.10;
                self.enforcement_rate = 0.85 + rand_simple() * 0.14;
            },
            CodeCategory::Electrical => {
                self.code_compliance = 0.90 + rand_simple() * 0.10;
                self.enforcement_rate = 0.85 + rand_simple() * 0.14;
                self.regulatory_coverage = 0.80 + rand_simple() * 0.18;
            },
            CodeCategory::Plumbing => {
                self.enforcement_rate = 0.85 + rand_simple() * 0.14;
                self.code_compliance = 0.85 + rand_simple() * 0.14;
                self.regulatory_coverage = 0.80 + rand_simple() * 0.18;
            },
            CodeCategory::Energy => {
                self.regulatory_coverage = 0.85 + rand_simple() * 0.14;
                self.code_compliance = 0.80 + rand_simple() * 0.18;
                self.updating_frequency = 0.90 + rand_simple() * 0.10;
            },
            CodeCategory::Accessibility => {
                self.code_compliance = 0.90 + rand_simple() * 0.10;
                self.regulatory_coverage = 0.90 + rand_simple() * 0.10;
                self.enforcement_rate = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.updating_frequency == 0.0 {
            self.updating_frequency = 0.7 + rand_simple() * 0.25;
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
    fn test_structural() {
        let mut system = BuildingCodesSystem::new(CodeCategory::Structural);
        system.analyze_system().unwrap();
        assert!(system.regulatory_coverage > 0.8);
    }
}
