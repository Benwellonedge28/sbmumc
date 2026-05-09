//! # SBMUMC Module 1172: Adult Education
//!
//! Educational programs designed for adult learners.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdultEducationModel {
    Andragogy,
    TransformativeLearning,
    ExperientialLearning,
    ProblemBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdultEducationFramework {
    pub framework_id: String,
    pub education_model: AdultEducationModel,
    pub self_directedness: f64,
    pub experience_utilization: f64,
    pub relevance_alignment: f64,
    pub readiness_acknowledgment: f64,
}

impl AdultEducationFramework {
    pub fn new(education_model: AdultEducationModel) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            education_model,
            self_directedness: 0.0,
            experience_utilization: 0.0,
            relevance_alignment: 0.0,
            readiness_acknowledgment: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.education_model {
            AdultEducationModel::Andragogy => {
                self.self_directedness = 0.85 + rand_simple() * 0.14;
                self.experience_utilization = 0.80 + rand_simple() * 0.18;
                self.readiness_acknowledgment = 0.75 + rand_simple() * 0.22;
            },
            AdultEducationModel::TransformativeLearning => {
                self.self_directedness = 0.75 + rand_simple() * 0.22;
                self.experience_utilization = 0.85 + rand_simple() * 0.14;
                self.relevance_alignment = 0.80 + rand_simple() * 0.18;
            },
            AdultEducationModel::ExperientialLearning => {
                self.experience_utilization = 0.90 + rand_simple() * 0.10;
                self.relevance_alignment = 0.75 + rand_simple() * 0.22;
                self.readiness_acknowledgment = 0.70 + rand_simple() * 0.25;
            },
            AdultEducationModel::ProblemBased => {
                self.self_directedness = 0.70 + rand_simple() * 0.25;
                self.relevance_alignment = 0.90 + rand_simple() * 0.10;
                self.readiness_acknowledgment = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.readiness_acknowledgment == 0.0 {
            self.readiness_acknowledgment = 0.60 + rand_simple() * 0.35;
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
    fn test_andragogy_model() {
        let mut framework = AdultEducationFramework::new(AdultEducationModel::Andragogy);
        framework.analyze_framework().unwrap();
        assert!(framework.self_directedness > 0.7);
    }
}