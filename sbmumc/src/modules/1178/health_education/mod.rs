//! # SBMUMC Module 1178: Health Education
//!
//! Knowledge and skills for health and wellness.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthEducationTopic {
    Nutrition,
    MentalHealth,
    SexualHealth,
    SubstancePrevention,
    InjuryPrevention,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthEducationFramework {
    pub framework_id: String,
    pub topic: HealthEducationTopic,
    pub knowledge_acquisition: f64,
    pub behavioral_change: f64,
    pub health_literacy: f64,
    pub preventive_behaviors: f64,
}

impl HealthEducationFramework {
    pub fn new(topic: HealthEducationTopic) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            topic,
            knowledge_acquisition: 0.0,
            behavioral_change: 0.0,
            health_literacy: 0.0,
            preventive_behaviors: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.topic {
            HealthEducationTopic::Nutrition => {
                self.knowledge_acquisition = 0.80 + rand_simple() * 0.18;
                self.behavioral_change = 0.65 + rand_simple() * 0.30;
                self.preventive_behaviors = 0.70 + rand_simple() * 0.25;
            },
            HealthEducationTopic::MentalHealth => {
                self.knowledge_acquisition = 0.75 + rand_simple() * 0.22;
                self.health_literacy = 0.80 + rand_simple() * 0.18;
                self.behavioral_change = 0.70 + rand_simple() * 0.25;
            },
            HealthEducationTopic::SexualHealth => {
                self.knowledge_acquisition = 0.85 + rand_simple() * 0.14;
                self.preventive_behaviors = 0.85 + rand_simple() * 0.14;
            },
            HealthEducationTopic::SubstancePrevention => {
                self.knowledge_acquisition = 0.80 + rand_simple() * 0.18;
                self.preventive_behaviors = 0.80 + rand_simple() * 0.18;
            },
            HealthEducationTopic::InjuryPrevention => {
                self.knowledge_acquisition = 0.75 + rand_simple() * 0.22;
                self.preventive_behaviors = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.health_literacy == 0.0 {
            self.health_literacy = (self.knowledge_acquisition + self.behavioral_change) / 2.0;
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
    fn test_sexual_health_education() {
        let mut framework = HealthEducationFramework::new(HealthEducationTopic::SexualHealth);
        framework.analyze_framework().unwrap();
        assert!(framework.preventive_behaviors > 0.7);
    }
}