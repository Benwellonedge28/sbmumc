//! # SBMUMC Module 1160: Special Education
//!
//! Educational approaches for students with diverse needs.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecialNeedsCategory {
    Learning,
    Physical,
    Behavioral,
    Communication,
    Multiple,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialEducationFramework {
    pub framework_id: String,
    pub needs_category: SpecialNeedsCategory,
    pub individualized_program: f64,
    pub inclusion_effectiveness: f64,
    pub resource_adequacy: f64,
    pub family_partnership: f64,
}

impl SpecialEducationFramework {
    pub fn new(needs_category: SpecialNeedsCategory) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            needs_category,
            individualized_program: 0.0,
            inclusion_effectiveness: 0.0,
            resource_adequacy: 0.0,
            family_partnership: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.needs_category {
            SpecialNeedsCategory::Learning => {
                self.individualized_program = 0.85 + rand_simple() * 0.14;
                self.inclusion_effectiveness = 0.75 + rand_simple() * 0.22;
            },
            SpecialNeedsCategory::Physical => {
                self.individualized_program = 0.80 + rand_simple() * 0.18;
                self.resource_adequacy = 0.85 + rand_simple() * 0.14;
            },
            SpecialNeedsCategory::Behavioral => {
                self.individualized_program = 0.75 + rand_simple() * 0.22;
                self.inclusion_effectiveness = 0.65 + rand_simple() * 0.30;
                self.family_partnership = 0.80 + rand_simple() * 0.18;
            },
            SpecialNeedsCategory::Communication => {
                self.individualized_program = 0.80 + rand_simple() * 0.18;
                self.inclusion_effectiveness = 0.70 + rand_simple() * 0.25;
            },
            SpecialNeedsCategory::Multiple => {
                self.individualized_program = 0.90 + rand_simple() * 0.10;
                self.resource_adequacy = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.resource_adequacy == 0.0 {
            self.resource_adequacy = 0.60 + rand_simple() * 0.35;
        }
        if self.family_partnership == 0.0 {
            self.family_partnership = 0.65 + rand_simple() * 0.30;
        }
        if self.inclusion_effectiveness == 0.0 {
            self.inclusion_effectiveness = 0.55 + rand_simple() * 0.40;
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
    fn test_learning_disabilities() {
        let mut framework = SpecialEducationFramework::new(SpecialNeedsCategory::Learning);
        framework.analyze_framework().unwrap();
        assert!(framework.individualized_program > 0.7);
    }
}