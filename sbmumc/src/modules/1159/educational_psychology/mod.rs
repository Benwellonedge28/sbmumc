//! # SBMUMC Module 1159: Educational Psychology
//!
//! Psychological principles applied to teaching and learning.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DevelopmentalStage {
    Early,
    Middle,
    Late,
    Adult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationalPsychologyModel {
    pub model_id: String,
    pub developmental_stage: DevelopmentalStage,
    pub cognitive_development: f64,
    pub social_emotional_growth: f64,
    pub motivation_factors: f64,
    pub learning_readiness: f64,
}

impl EducationalPsychologyModel {
    pub fn new(developmental_stage: DevelopmentalStage) -> Self {
        Self {
            model_id: crate::core::uuid_simple(),
            developmental_stage,
            cognitive_development: 0.0,
            social_emotional_growth: 0.0,
            motivation_factors: 0.0,
            learning_readiness: 0.0,
        }
    }

    pub fn analyze_model(&mut self) -> Result<()> {
        match self.developmental_stage {
            DevelopmentalStage::Early => {
                self.cognitive_development = 0.55 + rand_simple() * 0.40;
                self.social_emotional_growth = 0.60 + rand_simple() * 0.35;
                self.motivation_factors = 0.75 + rand_simple() * 0.22;
            },
            DevelopmentalStage::Middle => {
                self.cognitive_development = 0.70 + rand_simple() * 0.25;
                self.social_emotional_growth = 0.65 + rand_simple() * 0.30;
                self.motivation_factors = 0.60 + rand_simple() * 0.35;
            },
            DevelopmentalStage::Late => {
                self.cognitive_development = 0.85 + rand_simple() * 0.14;
                self.social_emotional_growth = 0.75 + rand_simple() * 0.22;
                self.motivation_factors = 0.70 + rand_simple() * 0.25;
            },
            DevelopmentalStage::Adult => {
                self.cognitive_development = 0.80 + rand_simple() * 0.18;
                self.social_emotional_growth = 0.80 + rand_simple() * 0.18;
                self.motivation_factors = 0.85 + rand_simple() * 0.14;
            },
        }

        self.learning_readiness = (self.cognitive_development + self.motivation_factors) / 2.0;
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
    fn test_adult_learning() {
        let mut model = EducationalPsychologyModel::new(DevelopmentalStage::Adult);
        model.analyze_model().unwrap();
        assert!(model.motivation_factors > 0.7);
    }
}