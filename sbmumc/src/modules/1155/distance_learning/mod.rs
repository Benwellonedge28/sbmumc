//! # SBMUMC Module 1155: Distance Learning
//!
//! Remote education delivery systems and methodologies.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistanceLearningPlatform {
    Synchronous,
    Asynchronous,
    Hybrid,
    SelfPaced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceLearningSystem {
    pub system_id: String,
    pub platform: DistanceLearningPlatform,
    pub accessibility_score: f64,
    pub learner_autonomy: f64,
    pub completion_rate: f64,
    pub digital_inclusion: f64,
}

impl DistanceLearningSystem {
    pub fn new(platform: DistanceLearningPlatform) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            platform,
            accessibility_score: 0.0,
            learner_autonomy: 0.0,
            completion_rate: 0.0,
            digital_inclusion: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.platform {
            DistanceLearningPlatform::Synchronous => {
                self.accessibility_score = 0.70 + rand_simple() * 0.25;
                self.learner_autonomy = 0.40 + rand_simple() * 0.35;
                self.completion_rate = 0.65 + rand_simple() * 0.30;
            },
            DistanceLearningPlatform::Asynchronous => {
                self.accessibility_score = 0.80 + rand_simple() * 0.18;
                self.learner_autonomy = 0.75 + rand_simple() * 0.22;
                self.completion_rate = 0.55 + rand_simple() * 0.35;
            },
            DistanceLearningPlatform::Hybrid => {
                self.accessibility_score = 0.75 + rand_simple() * 0.22;
                self.learner_autonomy = 0.60 + rand_simple() * 0.30;
                self.completion_rate = 0.70 + rand_simple() * 0.25;
            },
            DistanceLearningPlatform::SelfPaced => {
                self.accessibility_score = 0.85 + rand_simple() * 0.14;
                self.learner_autonomy = 0.90 + rand_simple() * 0.10;
                self.completion_rate = 0.45 + rand_simple() * 0.40;
            },
        }

        self.digital_inclusion = self.accessibility_score * (0.7 + rand_simple() * 0.3);
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
    fn test_async_learning() {
        let mut system = DistanceLearningSystem::new(DistanceLearningPlatform::Asynchronous);
        system.analyze_system().unwrap();
        assert!(system.accessibility_score > 0.6);
    }
}