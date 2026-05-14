//! # SBMUMC Module 1539: Shadow Magic
//!
//! Systems for shadow magic and darkness arts.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShadowMagicTopic {
    UmbralMagic,
    DarkArts,
    ShadowWeaving,
    DarknessPower,
    NocturnalMagic,
    ShadowForm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowMagicSystem {
    pub system_id: String,
    pub shadow_magic_topic: ShadowMagicTopic,
    pub shadow_power: f64,
    pub darkness_force: f64,
    pub umbral_mastery: f64,
    pub shadow_manipulation: f64,
}

impl ShadowMagicSystem {
    pub fn new(shadow_magic_topic: ShadowMagicTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            shadow_magic_topic,
            shadow_power: 0.0,
            darkness_force: 0.0,
            umbral_mastery: 0.0,
            shadow_manipulation: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.shadow_magic_topic {
            ShadowMagicTopic::UmbralMagic => {
                self.shadow_power = 0.95 + rand_simple() * 0.05;
                self.darkness_force = 0.90 + rand_simple() * 0.10;
                self.umbral_mastery = 0.85 + rand_simple() * 0.14;
            },
            ShadowMagicTopic::DarkArts => {
                self.shadow_manipulation = 0.95 + rand_simple() * 0.05;
                self.umbral_mastery = 0.90 + rand_simple() * 0.10;
                self.darkness_force = 0.85 + rand_simple() * 0.14;
            },
            ShadowMagicTopic::ShadowWeaving => {
                self.darkness_force = 0.95 + rand_simple() * 0.05;
                self.shadow_power = 0.90 + rand_simple() * 0.10;
                self.shadow_manipulation = 0.85 + rand_simple() * 0.14;
            },
            ShadowMagicTopic::DarknessPower => {
                self.umbral_mastery = 0.95 + rand_simple() * 0.05;
                self.shadow_manipulation = 0.90 + rand_simple() * 0.10;
                self.shadow_power = 0.85 + rand_simple() * 0.14;
            },
            ShadowMagicTopic::NocturnalMagic => {
                self.shadow_power = 0.95 + rand_simple() * 0.05;
                self.darkness_force = 0.90 + rand_simple() * 0.10;
                self.shadow_manipulation = 0.85 + rand_simple() * 0.14;
            },
            ShadowMagicTopic::ShadowForm => {
                self.shadow_manipulation = 0.95 + rand_simple() * 0.05;
                self.umbral_mastery = 0.90 + rand_simple() * 0.10;
                self.darkness_force = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.umbral_mastery == 0.0 {
            self.umbral_mastery = (self.shadow_power + self.darkness_force) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_umbral_magic() {
        let mut system = ShadowMagicSystem::new(ShadowMagicTopic::UmbralMagic);
        system.analyze_system().unwrap();
        assert!(system.shadow_power > 0.8);
    }
}