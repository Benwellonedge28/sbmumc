//! # SBMUMC Module 1532: Spirit Summoning
//!
//! Systems for spirit summoning and entity calling.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpiritSummoningTopic {
    EntityCalling,
    SpiritSummon,
    SupernaturalConjure,
    EntityEvocation,
    SpiritBinding,
    DemonicSummon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritSummoningSystem {
    pub system_id: String,
    pub spirit_summoning_topic: SpiritSummoningTopic,
    pub summoning_power: f64,
    pub entity_call: f64,
    pub spirit_control: f64,
    pub conjuration_magic: f64,
}

impl SpiritSummoningSystem {
    pub fn new(spirit_summoning_topic: SpiritSummoningTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            spirit_summoning_topic,
            summoning_power: 0.0,
            entity_call: 0.0,
            spirit_control: 0.0,
            conjuration_magic: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.spirit_summoning_topic {
            SpiritSummoningTopic::EntityCalling => {
                self.summoning_power = 0.95 + rand_simple() * 0.05;
                self.entity_call = 0.90 + rand_simple() * 0.10;
                self.spirit_control = 0.85 + rand_simple() * 0.14;
            },
            SpiritSummoningTopic::SpiritSummon => {
                self.conjuration_magic = 0.95 + rand_simple() * 0.05;
                self.spirit_control = 0.90 + rand_simple() * 0.10;
                self.entity_call = 0.85 + rand_simple() * 0.14;
            },
            SpiritSummoningTopic::SupernaturalConjure => {
                self.entity_call = 0.95 + rand_simple() * 0.05;
                self.summoning_power = 0.90 + rand_simple() * 0.10;
                self.conjuration_magic = 0.85 + rand_simple() * 0.14;
            },
            SpiritSummoningTopic::EntityEvocation => {
                self.spirit_control = 0.95 + rand_simple() * 0.05;
                self.conjuration_magic = 0.90 + rand_simple() * 0.10;
                self.summoning_power = 0.85 + rand_simple() * 0.14;
            },
            SpiritSummoningTopic::SpiritBinding => {
                self.summoning_power = 0.95 + rand_simple() * 0.05;
                self.entity_call = 0.90 + rand_simple() * 0.10;
                self.conjuration_magic = 0.85 + rand_simple() * 0.14;
            },
            SpiritSummoningTopic::DemonicSummon => {
                self.conjuration_magic = 0.95 + rand_simple() * 0.05;
                self.spirit_control = 0.90 + rand_simple() * 0.10;
                self.entity_call = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.spirit_control == 0.0 {
            self.spirit_control = (self.summoning_power + self.entity_call) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_entity_calling() {
        let mut system = SpiritSummoningSystem::new(SpiritSummoningTopic::EntityCalling);
        system.analyze_system().unwrap();
        assert!(system.summoning_power > 0.8);
    }
}