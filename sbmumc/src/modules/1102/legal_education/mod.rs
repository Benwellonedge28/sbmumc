//! # SBMUMC Module 1102: Legal Education
//!
//! Law schools, professional training, and legal执业.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegalEducationModel {
    Undergraduate,
    Graduate,
    Apprenticeship,
    Integrated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalEducationSystem {
    pub system_id: String,
    pub education_model: LegalEducationModel,
    pub law_school_count: usize,
    var bar_pass_rate: f64,
    pub practical_training_quality: f64,
    pub graduate_employability: f64,
}

impl LegalEducationSystem {
    pub fn new(model: LegalEducationModel, schools: usize) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            education_model: model,
            law_school_count: schools,
            var bar_pass_rate: 0.0,
            self.practical_training_quality = 0.0,
            self.graduate_employability = 0.0,
        }
    }

    pub fn assess_system(&mut self) -> Result<()> {
        match self.education_model {
            LegalEducationModel::Graduate => {
                self.bar_pass_rate = 0.75 + rand_simple() * 0.22;
                self.practical_training_quality = 0.60 + rand_simple() * 0.30;
            },
            LegalEducationModel::Apprenticeship => {
                self.bar_pass_rate = 0.65 + rand_simple() * 0.30;
                self.practical_training_quality = 0.80 + rand_simple() * 0.18;
            },
            _ => {
                self.bar_pass_rate = 0.60 + rand_simple() * 0.35;
                self.practical_training_quality = 0.55 + rand_simple() * 0.35;
            }
        }

        self.graduate_employability = self.bar_pass_rate * (0.7 + rand_simple() * 0.3);
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
    fn test_graduate_model() {
        let mut system = LegalEducationSystem::new(LegalEducationModel::Graduate, 200);
        system.assess_system().unwrap();
        assert!(system.bar_pass_rate > 0.5);
    }
}