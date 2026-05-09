//! # SBMUMC Module 1191: Open Educational Resources
//!
//! Freely accessible educational materials and tools.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OERType {
    Courseware,
    Textbooks,
    Videos,
    Interactive,
    AssessmentItems,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenEducationalResourcesSystem {
    pub system_id: String,
    pub oer_type: OERType,
    pub quality_assurance: f64,
    pub accessibility_score: f64,
    pub adoption_rate: f64,
    pub impact_on_equity: f64,
}

impl OpenEducationalResourcesSystem {
    pub fn new(oer_type: OERType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            oer_type,
            quality_assurance: 0.0,
            accessibility_score: 0.0,
            adoption_rate: 0.0,
            impact_on_equity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.oer_type {
            OERType::Courseware => {
                self.quality_assurance = 0.80 + rand_simple() * 0.18;
                self.adoption_rate = 0.65 + rand_simple() * 0.30;
                self.impact_on_equity = 0.75 + rand_simple() * 0.22;
            },
            OERType::Textbooks => {
                self.quality_assurance = 0.75 + rand_simple() * 0.22;
                self.accessibility_score = 0.85 + rand_simple() * 0.14;
                self.impact_on_equity = 0.85 + rand_simple() * 0.14;
            },
            OERType::Videos => {
                self.quality_assurance = 0.70 + rand_simple() * 0.25;
                self.accessibility_score = 0.75 + rand_simple() * 0.22;
                self.adoption_rate = 0.80 + rand_simple() * 0.18;
            },
            OERType::Interactive => {
                self.quality_assurance = 0.75 + rand_simple() * 0.22;
                self.accessibility_score = 0.70 + rand_simple() * 0.25;
                self.adoption_rate = 0.60 + rand_simple() * 0.35;
            },
            OERType::AssessmentItems => {
                self.quality_assurance = 0.85 + rand_simple() * 0.14;
                self.accessibility_score = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.adoption_rate == 0.0 {
            self.adoption_rate = 0.55 + rand_simple() * 0.40;
        }
        if self.impact_on_equity == 0.0 {
            self.impact_on_equity = (self.accessibility_score + self.quality_assurance) / 2.0 * (0.7 + rand_simple() * 0.3);
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
    fn test_open_textbooks() {
        let mut system = OpenEducationalResourcesSystem::new(OERType::Textbooks);
        system.analyze_system().unwrap();
        assert!(system.impact_on_equity > 0.6);
    }
}