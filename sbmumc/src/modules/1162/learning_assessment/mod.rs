//! # SBMUMC Module 1162: Learning Assessment
//!
//! Evaluation and measurement of learning outcomes.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssessmentMode {
    Traditional,
    Alternative,
    Authentic,
    Diagnostic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningAssessmentSystem {
    pub system_id: String,
    pub assessment_mode: AssessmentMode,
    pub measurement_accuracy: f64,
    pub diagnostic_value: f64,
    pub feedback_utility: f64,
    pub learning_alignment: f64,
}

impl LearningAssessmentSystem {
    pub fn new(assessment_mode: AssessmentMode) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            assessment_mode,
            measurement_accuracy: 0.0,
            diagnostic_value: 0.0,
            feedback_utility: 0.0,
            learning_alignment: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.assessment_mode {
            AssessmentMode::Traditional => {
                self.measurement_accuracy = 0.80 + rand_simple() * 0.18;
                self.diagnostic_value = 0.55 + rand_simple() * 0.40;
                self.feedback_utility = 0.50 + rand_simple() * 0.40;
            },
            AssessmentMode::Alternative => {
                self.measurement_accuracy = 0.70 + rand_simple() * 0.25;
                self.diagnostic_value = 0.75 + rand_simple() * 0.22;
                self.feedback_utility = 0.70 + rand_simple() * 0.25;
            },
            AssessmentMode::Authentic => {
                self.measurement_accuracy = 0.65 + rand_simple() * 0.30;
                self.diagnostic_value = 0.70 + rand_simple() * 0.25;
                self.feedback_utility = 0.85 + rand_simple() * 0.14;
                self.learning_alignment = 0.90 + rand_simple() * 0.10;
            },
            AssessmentMode::Diagnostic => {
                self.measurement_accuracy = 0.75 + rand_simple() * 0.22;
                self.diagnostic_value = 0.90 + rand_simple() * 0.10;
                self.feedback_utility = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.learning_alignment == 0.0 {
            self.learning_alignment = (self.measurement_accuracy + self.feedback_utility) / 2.0;
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
    fn test_authentic_assessment() {
        let mut system = LearningAssessmentSystem::new(AssessmentMode::Authentic);
        system.analyze_system().unwrap();
        assert!(system.feedback_utility > 0.7);
    }
}