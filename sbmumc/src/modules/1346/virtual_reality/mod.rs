//! # SBMUMC Module 1346: Virtual Reality
//!
//! Systems for virtual reality experiences.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VRApplication {
    Gaming,
    Training,
    Education,
    Therapy,
    Social,
    Visualization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualRealitySystem {
    pub system_id: String,
    pub vr_application: VRApplication,
    pub immersion_quality: f64,
    pub interactivity: f64,
    pub comfort_level: f64,
    pub content_quality: f64,
}

impl VirtualRealitySystem {
    pub fn new(vr_application: VRApplication) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            vr_application,
            immersion_quality: 0.0,
            interactivity: 0.0,
            comfort_level: 0.0,
            content_quality: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.vr_application {
            VRApplication::Gaming => {
                self.immersion_quality = 0.95 + rand_simple() * 0.05;
                self.interactivity = 0.90 + rand_simple() * 0.10;
                self.content_quality = 0.85 + rand_simple() * 0.14;
            },
            VRApplication::Training => {
                self.content_quality = 0.95 + rand_simple() * 0.05;
                self.interactivity = 0.90 + rand_simple() * 0.10;
                self.comfort_level = 0.85 + rand_simple() * 0.14;
            },
            VRApplication::Education => {
                self.content_quality = 0.95 + rand_simple() * 0.05;
                self.immersion_quality = 0.85 + rand_simple() * 0.14;
                self.interactivity = 0.85 + rand_simple() * 0.14;
            },
            VRApplication::Therapy => {
                self.comfort_level = 0.95 + rand_simple() * 0.05;
                self.content_quality = 0.90 + rand_simple() * 0.10;
                self.interactivity = 0.85 + rand_simple() * 0.14;
            },
            VRApplication::Social => {
                self.interactivity = 0.95 + rand_simple() * 0.05;
                self.immersion_quality = 0.85 + rand_simple() * 0.14;
                self.comfort_level = 0.80 + rand_simple() * 0.18;
            },
            VRApplication::Visualization => {
                self.immersion_quality = 0.95 + rand_simple() * 0.05;
                self.content_quality = 0.90 + rand_simple() * 0.10;
                self.interactivity = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.comfort_level == 0.0 {
            self.comfort_level = (self.immersion_quality + self.interactivity) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_gaming() {
        let mut system = VirtualRealitySystem::new(VRApplication::Gaming);
        system.analyze_system().unwrap();
        assert!(system.immersion_quality > 0.8);
    }
}