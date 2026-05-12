//! # SBMUMC Module 1386: Immersive Experiences
//!
//! Systems for immersive entertainment experiences.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImmersionType {
    VirtualReality,
    AugmentedReality,
    MixedReality,
    SpatialAudio,
    HapticFeedback,
    MultiSensory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmersiveExperienceSystem {
    pub system_id: String,
    pub immersion_type: ImmersionType,
    pub presence_building: f64,
    pub sensory_integration: f64,
    pub environmental_storytelling: f64,
    pub user_agency: f64,
}

impl ImmersiveExperienceSystem {
    pub fn new(immersion_type: ImmersionType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            immersion_type,
            presence_building: 0.0,
            sensory_integration: 0.0,
            environmental_storytelling: 0.0,
            user_agency: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.immersion_type {
            ImmersionType::VirtualReality => {
                self.presence_building = 0.95 + rand_simple() * 0.05;
                self.sensory_integration = 0.90 + rand_simple() * 0.10;
                self.environmental_storytelling = 0.85 + rand_simple() * 0.14;
            },
            ImmersionType::AugmentedReality => {
                self.user_agency = 0.95 + rand_simple() * 0.05;
                self.environmental_storytelling = 0.90 + rand_simple() * 0.10;
                self.sensory_integration = 0.85 + rand_simple() * 0.14;
            },
            ImmersionType::MixedReality => {
                self.presence_building = 0.95 + rand_simple() * 0.05;
                self.user_agency = 0.90 + rand_simple() * 0.10;
                self.sensory_integration = 0.85 + rand_simple() * 0.14;
            },
            ImmersionType::SpatialAudio => {
                self.sensory_integration = 0.95 + rand_simple() * 0.05;
                self.presence_building = 0.90 + rand_simple() * 0.10;
                self.environmental_storytelling = 0.85 + rand_simple() * 0.14;
            },
            ImmersionType::HapticFeedback => {
                self.sensory_integration = 0.95 + rand_simple() * 0.05;
                self.user_agency = 0.90 + rand_simple() * 0.10;
                self.presence_building = 0.85 + rand_simple() * 0.14;
            },
            ImmersionType::MultiSensory => {
                self.environmental_storytelling = 0.95 + rand_simple() * 0.05;
                self.presence_building = 0.90 + rand_simple() * 0.10;
                self.user_agency = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.environmental_storytelling == 0.0 {
            self.environmental_storytelling = (self.presence_building + self.sensory_integration) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_virtual_reality() {
        let mut system = ImmersiveExperienceSystem::new(ImmersionType::VirtualReality);
        system.analyze_system().unwrap();
        assert!(system.presence_building > 0.8);
    }
}
