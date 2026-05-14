//! # SBMUMC Module 1538: Light Magic
//!
//! Systems for light magic and luminous forces.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LightMagicTopic {
    RadiantMagic,
    SolarMagic,
    IlluminationArts,
    PhotonicMagic,
    LuminousPower,
    DivineLight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightMagicSystem {
    pub system_id: String,
    pub light_magic_topic: LightMagicTopic,
    pub light_power: f64,
    pub radiance_force: f64,
    pub luminous_mastery: f64,
    pub illumination_magic: f64,
}

impl LightMagicSystem {
    pub fn new(light_magic_topic: LightMagicTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            light_magic_topic,
            light_power: 0.0,
            radiance_force: 0.0,
            luminous_mastery: 0.0,
            illumination_magic: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.light_magic_topic {
            LightMagicTopic::RadiantMagic => {
                self.light_power = 0.95 + rand_simple() * 0.05;
                self.radiance_force = 0.90 + rand_simple() * 0.10;
                self.luminous_mastery = 0.85 + rand_simple() * 0.14;
            },
            LightMagicTopic::SolarMagic => {
                self.illumination_magic = 0.95 + rand_simple() * 0.05;
                self.luminous_mastery = 0.90 + rand_simple() * 0.10;
                self.radiance_force = 0.85 + rand_simple() * 0.14;
            },
            LightMagicTopic::IlluminationArts => {
                self.radiance_force = 0.95 + rand_simple() * 0.05;
                self.light_power = 0.90 + rand_simple() * 0.10;
                self.illumination_magic = 0.85 + rand_simple() * 0.14;
            },
            LightMagicTopic::PhotonicMagic => {
                self.luminous_mastery = 0.95 + rand_simple() * 0.05;
                self.illumination_magic = 0.90 + rand_simple() * 0.10;
                self.light_power = 0.85 + rand_simple() * 0.14;
            },
            LightMagicTopic::LuminousPower => {
                self.light_power = 0.95 + rand_simple() * 0.05;
                self.radiance_force = 0.90 + rand_simple() * 0.10;
                self.illumination_magic = 0.85 + rand_simple() * 0.14;
            },
            LightMagicTopic::DivineLight => {
                self.illumination_magic = 0.95 + rand_simple() * 0.05;
                self.luminous_mastery = 0.90 + rand_simple() * 0.10;
                self.radiance_force = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.luminous_mastery == 0.0 {
            self.luminous_mastery = (self.light_power + self.radiance_force) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_radiant_magic() {
        let mut system = LightMagicSystem::new(LightMagicTopic::RadiantMagic);
        system.analyze_system().unwrap();
        assert!(system.light_power > 0.8);
    }
}