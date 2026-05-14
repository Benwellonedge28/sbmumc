//! # SBMUMC Module 1535: Dream Weaving
//!
//! Systems for dream weaving and sleep magic.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DreamWeavingTopic {
    DreamMagic,
    SleepSpells,
    DreamWalking,
    NightmareCraft,
    LucidDreamingMagic,
    DreamRealmAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamWeavingSystem {
    pub system_id: String,
    pub dream_weaving_topic: DreamWeavingTopic,
    pub dream_power: f64,
    pub subconscious_magic: f64,
    pub dream_walking: f64,
    pub sleep_manipulation: f64,
}

impl DreamWeavingSystem {
    pub fn new(dream_weaving_topic: DreamWeavingTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            dream_weaving_topic,
            dream_power: 0.0,
            subconscious_magic: 0.0,
            dream_walking: 0.0,
            sleep_manipulation: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.dream_weaving_topic {
            DreamWeavingTopic::DreamMagic => {
                self.dream_power = 0.95 + rand_simple() * 0.05;
                self.subconscious_magic = 0.90 + rand_simple() * 0.10;
                self.dream_walking = 0.85 + rand_simple() * 0.14;
            },
            DreamWeavingTopic::SleepSpells => {
                self.sleep_manipulation = 0.95 + rand_simple() * 0.05;
                self.dream_walking = 0.90 + rand_simple() * 0.10;
                self.subconscious_magic = 0.85 + rand_simple() * 0.14;
            },
            DreamWeavingTopic::DreamWalking => {
                self.subconscious_magic = 0.95 + rand_simple() * 0.05;
                self.dream_power = 0.90 + rand_simple() * 0.10;
                self.sleep_manipulation = 0.85 + rand_simple() * 0.14;
            },
            DreamWeavingTopic::NightmareCraft => {
                self.dream_walking = 0.95 + rand_simple() * 0.05;
                self.sleep_manipulation = 0.90 + rand_simple() * 0.10;
                self.dream_power = 0.85 + rand_simple() * 0.14;
            },
            DreamWeavingTopic::LucidDreamingMagic => {
                self.dream_power = 0.95 + rand_simple() * 0.05;
                self.subconscious_magic = 0.90 + rand_simple() * 0.10;
                self.sleep_manipulation = 0.85 + rand_simple() * 0.14;
            },
            DreamWeavingTopic::DreamRealmAccess => {
                self.sleep_manipulation = 0.95 + rand_simple() * 0.05;
                self.dream_walking = 0.90 + rand_simple() * 0.10;
                self.subconscious_magic = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.dream_walking == 0.0 {
            self.dream_walking = (self.dream_power + self.subconscious_magic) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_dream_magic() {
        let mut system = DreamWeavingSystem::new(DreamWeavingTopic::DreamMagic);
        system.analyze_system().unwrap();
        assert!(system.dream_power > 0.8);
    }
}