//! # SBMUMC Module 1536: Chaos Magic
//!
//! Systems for chaos magic and unpredictable forces.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChaosMagicTopic {
    ChaoticForces,
    UnpredictableMagic,
    ChaosTheoryMagic,
    WildMagic,
   EntropyMagic,
    RandomizedSpellcasting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosMagicSystem {
    pub system_id: String,
    pub chaos_magic_topic: ChaosMagicTopic,
    pub chaotic_power: f64,
    pub unpredictable_force: f64,
    pub entropy_mastery: f64,
    pub wild_magic_skill: f64,
}

impl ChaosMagicSystem {
    pub fn new(chaos_magic_topic: ChaosMagicTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            chaos_magic_topic,
            chaotic_power: 0.0,
            unpredictable_force: 0.0,
            entropy_mastery: 0.0,
            wild_magic_skill: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.chaos_magic_topic {
            ChaosMagicTopic::ChaoticForces => {
                self.chaotic_power = 0.95 + rand_simple() * 0.05;
                self.unpredictable_force = 0.90 + rand_simple() * 0.10;
                self.entropy_mastery = 0.85 + rand_simple() * 0.14;
            },
            ChaosMagicTopic::UnpredictableMagic => {
                self.wild_magic_skill = 0.95 + rand_simple() * 0.05;
                self.entropy_mastery = 0.90 + rand_simple() * 0.10;
                self.unpredictable_force = 0.85 + rand_simple() * 0.14;
            },
            ChaosMagicTopic::ChaosTheoryMagic => {
                self.unpredictable_force = 0.95 + rand_simple() * 0.05;
                self.chaotic_power = 0.90 + rand_simple() * 0.10;
                self.wild_magic_skill = 0.85 + rand_simple() * 0.14;
            },
            ChaosMagicTopic::WildMagic => {
                self.entropy_mastery = 0.95 + rand_simple() * 0.05;
                self.wild_magic_skill = 0.90 + rand_simple() * 0.10;
                self.chaotic_power = 0.85 + rand_simple() * 0.14;
            },
            ChaosMagicTopic::EntropyMagic => {
                self.chaotic_power = 0.95 + rand_simple() * 0.05;
                self.unpredictable_force = 0.90 + rand_simple() * 0.10;
                self.wild_magic_skill = 0.85 + rand_simple() * 0.14;
            },
            ChaosMagicTopic::RandomizedSpellcasting => {
                self.wild_magic_skill = 0.95 + rand_simple() * 0.05;
                self.entropy_mastery = 0.90 + rand_simple() * 0.10;
                self.unpredictable_force = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.entropy_mastery == 0.0 {
            self.entropy_mastery = (self.chaotic_power + self.unpredictable_force) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_chaotic_forces() {
        let mut system = ChaosMagicSystem::new(ChaosMagicTopic::ChaoticForces);
        system.analyze_system().unwrap();
        assert!(system.chaotic_power > 0.8);
    }
}