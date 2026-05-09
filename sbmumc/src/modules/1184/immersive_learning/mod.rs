//! # SBMUMC Module 1184: Immersive Learning
//!
//! Learning experiences using VR, AR, and mixed reality technologies.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImmersiveTechnology {
    VirtualReality,
    AugmentedReality,
    MixedReality,
    Simulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmersiveLearningSystem {
    pub system_id: String,
    pub immersive_technology: ImmersiveTechnology,
    pub presence_immersion: f64,
    pub embodied_learning: f64,
    pub contextual_transfer: f64,
    pub safety_availability: f64,
}

impl ImmersiveLearningSystem {
    pub fn new(immersive_technology: ImmersiveTechnology) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            immersive_technology,
            presence_immersion: 0.0,
            embodied_learning: 0.0,
            contextual_transfer: 0.0,
            safety_availability: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.immersive_technology {
            ImmersiveTechnology::VirtualReality => {
                self.presence_immersion = 0.90 + rand_simple() * 0.10;
                self.embodied_learning = 0.85 + rand_simple() * 0.14;
                self.contextual_transfer = 0.70 + rand_simple() * 0.25;
            },
            ImmersiveTechnology::AugmentedReality => {
                self.presence_immersion = 0.60 + rand_simple() * 0.35;
                self.embodied_learning = 0.70 + rand_simple() * 0.25;
                self.contextual_transfer = 0.85 + rand_simple() * 0.14;
                self.safety_availability = 0.90 + rand_simple() * 0.10;
            },
            ImmersiveTechnology::MixedReality => {
                self.presence_immersion = 0.80 + rand_simple() * 0.18;
                self.embodied_learning = 0.85 + rand_simple() * 0.14;
                self.contextual_transfer = 0.80 + rand_simple() * 0.18;
            },
            ImmersiveTechnology::Simulation => {
                self.presence_immersion = 0.75 + rand_simple() * 0.22;
                self.contextual_transfer = 0.85 + rand_simple() * 0.14;
                self.safety_availability = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.safety_availability == 0.0 {
            self.safety_availability = 0.65 + rand_simple() * 0.30;
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
    fn test_vr_immersion() {
        let mut system = ImmersiveLearningSystem::new(ImmersiveTechnology::VirtualReality);
        system.analyze_system().unwrap();
        assert!(system.presence_immersion > 0.7);
    }
}