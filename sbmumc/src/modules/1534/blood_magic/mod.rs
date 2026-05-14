//! # SBMUMC Module 1534: Blood Magic
//!
//! Systems for blood magic and vital essence.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BloodMagicTopic {
    VitalEssence,
    BloodRituals,
    LifeForceMagic,
    BloodPact,
    Hematomancy,
    SacrificialMagic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodMagicSystem {
    pub system_id: String,
    pub blood_magic_topic: BloodMagicTopic,
    pub blood_power: f64,
    pub life_force: f64,
    pub vital_essence: f64,
    pub blood_magic_skill: f64,
}

impl BloodMagicSystem {
    pub fn new(blood_magic_topic: BloodMagicTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            blood_magic_topic,
            blood_power: 0.0,
            life_force: 0.0,
            vital_essence: 0.0,
            blood_magic_skill: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.blood_magic_topic {
            BloodMagicTopic::VitalEssence => {
                self.blood_power = 0.95 + rand_simple() * 0.05;
                self.life_force = 0.90 + rand_simple() * 0.10;
                self.vital_essence = 0.85 + rand_simple() * 0.14;
            },
            BloodMagicTopic::BloodRituals => {
                self.blood_magic_skill = 0.95 + rand_simple() * 0.05;
                self.vital_essence = 0.90 + rand_simple() * 0.10;
                self.life_force = 0.85 + rand_simple() * 0.14;
            },
            BloodMagicTopic::LifeForceMagic => {
                self.life_force = 0.95 + rand_simple() * 0.05;
                self.blood_power = 0.90 + rand_simple() * 0.10;
                self.blood_magic_skill = 0.85 + rand_simple() * 0.14;
            },
            BloodMagicTopic::BloodPact => {
                self.vital_essence = 0.95 + rand_simple() * 0.05;
                self.blood_magic_skill = 0.90 + rand_simple() * 0.10;
                self.blood_power = 0.85 + rand_simple() * 0.14;
            },
            BloodMagicTopic::Hematomancy => {
                self.blood_power = 0.95 + rand_simple() * 0.05;
                self.life_force = 0.90 + rand_simple() * 0.10;
                self.blood_magic_skill = 0.85 + rand_simple() * 0.14;
            },
            BloodMagicTopic::SacrificialMagic => {
                self.blood_magic_skill = 0.95 + rand_simple() * 0.05;
                self.vital_essence = 0.90 + rand_simple() * 0.10;
                self.life_force = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.vital_essence == 0.0 {
            self.vital_essence = (self.blood_power + self.life_force) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_vital_essence() {
        let mut system = BloodMagicSystem::new(BloodMagicTopic::VitalEssence);
        system.analyze_system().unwrap();
        assert!(system.blood_power > 0.8);
    }
}