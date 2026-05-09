//! # SBMUMC Module 1146: Learning Sciences
//!
//! Scientific study of learning processes and mechanisms.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningTheory {
    InformationProcessing,
    Connectionist,
    EmbodiedCognition,
    SituatedLearning,
    SocialLearning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningScienceModel {
    pub model_id: String,
    pub theory: LearningTheory,
    pub cognitive_load_score: f64,
    pub memory_consolidation: f64,
    pub transfer_effectiveness: f64,
    pub metacognitive_development: f64,
}

impl LearningScienceModel {
    pub fn new(theory: LearningTheory) -> Self {
        Self {
            model_id: crate::core::uuid_simple(),
            theory,
            cognitive_load_score: 0.0,
            memory_consolidation: 0.0,
            transfer_effectiveness: 0.0,
            metacognitive_development: 0.0,
        }
    }

    pub fn analyze_model(&mut self) -> Result<()> {
        match self.theory {
            LearningTheory::InformationProcessing => {
                self.cognitive_load_score = 0.65 + rand_simple() * 0.30;
                self.memory_consolidation = 0.75 + rand_simple() * 0.22;
            },
            LearningTheory::Connectionist => {
                self.cognitive_load_score = 0.70 + rand_simple() * 0.28;
                self.memory_consolidation = 0.80 + rand_simple() * 0.18;
            },
            LearningTheory::EmbodiedCognition => {
                self.cognitive_load_score = 0.55 + rand_simple() * 0.40;
                self.memory_consolidation = 0.85 + rand_simple() * 0.14;
            },
            _ => {
                self.cognitive_load_score = 0.60 + rand_simple() * 0.35;
                self.memory_consolidation = 0.70 + rand_simple() * 0.25;
            },
        }

        self.transfer_effectiveness = self.memory_consolidation * (0.7 + rand_simple() * 0.3);
        self.metacognitive_development = 0.50 + rand_simple() * 0.45;
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
    fn test_embodied_cognition() {
        let mut model = LearningScienceModel::new(LearningTheory::EmbodiedCognition);
        model.analyze_model().unwrap();
        assert!(model.memory_consolidation > 0.7);
    }
}