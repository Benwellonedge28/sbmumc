//! # SBMUMC Module 1221: Agricultural Education
//!
//! Teaching and training in agricultural sciences and practices.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EducationLevel {
    Vocational,
    Undergraduate,
    Graduate,
    Extension,
    FarmerTraining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalEducationSystem {
    pub system_id: String,
    pub education_level: EducationLevel,
    pub knowledge_transfer: f64,
    pub skill_development: f64,
    pub adoption_rate: f64,
    pub innovation_capacity: f64,
}

impl AgriculturalEducationSystem {
    pub fn new(education_level: EducationLevel) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            education_level,
            knowledge_transfer: 0.0,
            skill_development: 0.0,
            adoption_rate: 0.0,
            innovation_capacity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.education_level {
            EducationLevel::Vocational => {
                self.skill_development = 0.85 + rand_simple() * 0.14;
                self.adoption_rate = 0.80 + rand_simple() * 0.18;
            },
            EducationLevel::Undergraduate => {
                self.knowledge_transfer = 0.80 + rand_simple() * 0.18;
                self.innovation_capacity = 0.70 + rand_simple() * 0.25;
            },
            EducationLevel::Graduate => {
                self.knowledge_transfer = 0.85 + rand_simple() * 0.14;
                self.innovation_capacity = 0.85 + rand_simple() * 0.14;
            },
            EducationLevel::Extension => {
                self.adoption_rate = 0.85 + rand_simple() * 0.14;
                self.skill_development = 0.75 + rand_simple() * 0.22;
            },
            EducationLevel::FarmerTraining => {
                self.skill_development = 0.80 + rand_simple() * 0.18;
                self.adoption_rate = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.innovation_capacity == 0.0 {
            self.innovation_capacity = (self.knowledge_transfer + self.skill_development) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_extension_education() {
        let mut system = AgriculturalEducationSystem::new(EducationLevel::Extension);
        system.analyze_system().unwrap();
        assert!(system.adoption_rate > 0.6);
    }
}