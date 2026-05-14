//! # SBMUMC Module 1533: Necromantic Arts
//!
//! Systems for necromantic arts and death magic.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NecromanticArtsTopic {
    DeathMagic,
    SoulManipulation,
    CorpseAnimation,
    DeathRites,
    GraveMagic,
    UndeadCreation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NecromanticArtsSystem {
    pub system_id: String,
    pub necromantic_arts_topic: NecromanticArtsTopic,
    pub death_magic: f64,
    pub soul_manipulation: f64,
    pub undead_power: f64,
    pub death_wisdom: f64,
}

impl NecromanticArtsSystem {
    pub fn new(necromantic_arts_topic: NecromanticArtsTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            necromantic_arts_topic,
            death_magic: 0.0,
            soul_manipulation: 0.0,
            undead_power: 0.0,
            death_wisdom: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.necromantic_arts_topic {
            NecromanticArtsTopic::DeathMagic => {
                self.death_magic = 0.95 + rand_simple() * 0.05;
                self.soul_manipulation = 0.90 + rand_simple() * 0.10;
                self.undead_power = 0.85 + rand_simple() * 0.14;
            },
            NecromanticArtsTopic::SoulManipulation => {
                self.death_wisdom = 0.95 + rand_simple() * 0.05;
                self.undead_power = 0.90 + rand_simple() * 0.10;
                self.soul_manipulation = 0.85 + rand_simple() * 0.14;
            },
            NecromanticArtsTopic::CorpseAnimation => {
                self.soul_manipulation = 0.95 + rand_simple() * 0.05;
                self.death_magic = 0.90 + rand_simple() * 0.10;
                self.death_wisdom = 0.85 + rand_simple() * 0.14;
            },
            NecromanticArtsTopic::DeathRites => {
                self.undead_power = 0.95 + rand_simple() * 0.05;
                self.death_wisdom = 0.90 + rand_simple() * 0.10;
                self.death_magic = 0.85 + rand_simple() * 0.14;
            },
            NecromanticArtsTopic::GraveMagic => {
                self.death_magic = 0.95 + rand_simple() * 0.05;
                self.soul_manipulation = 0.90 + rand_simple() * 0.10;
                self.death_wisdom = 0.85 + rand_simple() * 0.14;
            },
            NecromanticArtsTopic::UndeadCreation => {
                self.death_wisdom = 0.95 + rand_simple() * 0.05;
                self.undead_power = 0.90 + rand_simple() * 0.10;
                self.soul_manipulation = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.undead_power == 0.0 {
            self.undead_power = (self.death_magic + self.soul_manipulation) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_death_magic() {
        let mut system = NecromanticArtsSystem::new(NecromanticArtsTopic::DeathMagic);
        system.analyze_system().unwrap();
        assert!(system.death_magic > 0.8);
    }
}