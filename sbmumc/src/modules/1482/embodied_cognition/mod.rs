//! # SBMUMC Module 1482: Embodied Cognition
//!
//! Systems for embodied cognition and extended mind.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmbodiedCognitionTopic {
    EmbodiedMind,
    ExtendedMind,
    Enactivism,
    SensorimotorTheory,
    CellularCognition,
    SituatedCognition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbodiedCognitionSystem {
    pub system_id: String,
    pub embodied_cognition_topic: EmbodiedCognitionTopic,
    pub bodily_engagement: f64,
    pub environmental_coupling: f64,
    pub cognitive_extension: f64,
    pub phenomenological_integration: f64,
}

impl EmbodiedCognitionSystem {
    pub fn new(embodied_cognition_topic: EmbodiedCognitionTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            embodied_cognition_topic,
            bodily_engagement: 0.0,
            environmental_coupling: 0.0,
            cognitive_extension: 0.0,
            phenomenological_integration: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.embodied_cognition_topic {
            EmbodiedCognitionTopic::EmbodiedMind => {
                self.bodily_engagement = 0.95 + rand_simple() * 0.05;
                self.environmental_coupling = 0.90 + rand_simple() * 0.10;
                self.cognitive_extension = 0.85 + rand_simple() * 0.14;
            },
            EmbodiedCognitionTopic::ExtendedMind => {
                self.phenomenological_integration = 0.95 + rand_simple() * 0.05;
                self.bodily_engagement = 0.90 + rand_simple() * 0.10;
                self.environmental_coupling = 0.85 + rand_simple() * 0.14;
            },
            EmbodiedCognitionTopic::Enactivism => {
                self.cognitive_extension = 0.95 + rand_simple() * 0.05;
                self.phenomenological_integration = 0.90 + rand_simple() * 0.10;
                self.bodily_engagement = 0.85 + rand_simple() * 0.14;
            },
            EmbodiedCognitionTopic::SensorimotorTheory => {
                self.environmental_coupling = 0.95 + rand_simple() * 0.05;
                self.cognitive_extension = 0.90 + rand_simple() * 0.10;
                self.phenomenological_integration = 0.85 + rand_simple() * 0.14;
            },
            EmbodiedCognitionTopic::CellularCognition => {
                self.bodily_engagement = 0.95 + rand_simple() * 0.05;
                self.environmental_coupling = 0.90 + rand_simple() * 0.10;
                self.phenomenological_integration = 0.85 + rand_simple() * 0.14;
            },
            EmbodiedCognitionTopic::SituatedCognition => {
                self.cognitive_extension = 0.95 + rand_simple() * 0.05;
                self.bodily_engagement = 0.90 + rand_simple() * 0.10;
                self.environmental_coupling = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.cognitive_extension == 0.0 {
            self.cognitive_extension = (self.bodily_engagement + self.environmental_coupling) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_embodied_mind() {
        let mut system = EmbodiedCognitionSystem::new(EmbodiedCognitionTopic::EmbodiedMind);
        system.analyze_system().unwrap();
        assert!(system.bodily_engagement > 0.8);
    }
}