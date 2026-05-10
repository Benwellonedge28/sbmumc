//! # SBMUMC Module 1322: Adaptive Architecture
//!
//! Systems for buildings that adapt to changing conditions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdaptiveCapability {
    KineticElements,
    ReconfigurableSpaces,
    ResponsiveEnvelopes,
    SmartMaterials,
    ModularConstruction,
    TransformableInteriors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveArchitectureSystem {
    pub system_id: String,
    pub adaptive_capability: AdaptiveCapability,
    pub adaptation_speed: f64,
    pub user_control: f64,
    pub energy_adaptation: f64,
    pub durability: f64,
}

impl AdaptiveArchitectureSystem {
    pub fn new(adaptive_capability: AdaptiveCapability) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            adaptive_capability,
            adaptation_speed: 0.0,
            user_control: 0.0,
            energy_adaptation: 0.0,
            durability: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.adaptive_capability {
            AdaptiveCapability::KineticElements => {
                self.adaptation_speed = 0.90 + rand_simple() * 0.10;
                self.user_control = 0.85 + rand_simple() * 0.14;
                self.durability = 0.70 + rand_simple() * 0.25;
            },
            AdaptiveCapability::ReconfigurableSpaces => {
                self.user_control = 0.95 + rand_simple() * 0.05;
                self.adaptation_speed = 0.80 + rand_simple() * 0.18;
                self.energy_adaptation = 0.75 + rand_simple() * 0.22;
            },
            AdaptiveCapability::ResponsiveEnvelopes => {
                self.energy_adaptation = 0.90 + rand_simple() * 0.10;
                self.adaptation_speed = 0.85 + rand_simple() * 0.14;
                self.durability = 0.80 + rand_simple() * 0.18;
            },
            AdaptiveCapability::SmartMaterials => {
                self.durability = 0.90 + rand_simple() * 0.10;
                self.energy_adaptation = 0.85 + rand_simple() * 0.14;
                self.adaptation_speed = 0.75 + rand_simple() * 0.22;
            },
            AdaptiveCapability::ModularConstruction => {
                self.adaptation_speed = 0.85 + rand_simple() * 0.14;
                self.durability = 0.90 + rand_simple() * 0.10;
                self.user_control = 0.80 + rand_simple() * 0.18;
            },
            AdaptiveCapability::TransformableInteriors => {
                self.user_control = 0.90 + rand_simple() * 0.10;
                self.durability = 0.85 + rand_simple() * 0.14;
                self.adaptation_speed = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.energy_adaptation == 0.0 {
            self.energy_adaptation = (self.adaptation_speed + self.durability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_responsive_envelopes() {
        let mut system = AdaptiveArchitectureSystem::new(AdaptiveCapability::ResponsiveEnvelopes);
        system.analyze_system().unwrap();
        assert!(system.energy_adaptation > 0.7);
    }
}
