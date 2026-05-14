//! # SBMUMC Module 1525: Prophecy Sight
//!
//! Systems for prophecy sight and oracle visions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProphecySightTopic {
    OracleVision,
    FutureSight,
    PropheticDreams,
    SeerAbilities,
    DivinationPower,
    ProphecyChannels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProphecySightSystem {
    pub system_id: String,
    pub prophecy_sight_topic: ProphecySightTopic,
    pub prophetic_vision: f64,
    pub oracle_gift: f64,
    pub future_perception: f64,
    pub visionary_sight: f64,
}

impl ProphecySightSystem {
    pub fn new(prophecy_sight_topic: ProphecySightTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            prophecy_sight_topic,
            prophetic_vision: 0.0,
            oracle_gift: 0.0,
            future_perception: 0.0,
            visionary_sight: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.prophecy_sight_topic {
            ProphecySightTopic::OracleVision => {
                self.prophetic_vision = 0.95 + rand_simple() * 0.05;
                self.oracle_gift = 0.90 + rand_simple() * 0.10;
                self.future_perception = 0.85 + rand_simple() * 0.14;
            },
            ProphecySightTopic::FutureSight => {
                self.visionary_sight = 0.95 + rand_simple() * 0.05;
                self.future_perception = 0.90 + rand_simple() * 0.10;
                self.oracle_gift = 0.85 + rand_simple() * 0.14;
            },
            ProphecySightTopic::PropheticDreams => {
                self.oracle_gift = 0.95 + rand_simple() * 0.05;
                self.prophetic_vision = 0.90 + rand_simple() * 0.10;
                self.visionary_sight = 0.85 + rand_simple() * 0.14;
            },
            ProphecySightTopic::SeerAbilities => {
                self.future_perception = 0.95 + rand_simple() * 0.05;
                self.visionary_sight = 0.90 + rand_simple() * 0.10;
                self.prophetic_vision = 0.85 + rand_simple() * 0.14;
            },
            ProphecySightTopic::DivinationPower => {
                self.prophetic_vision = 0.95 + rand_simple() * 0.05;
                self.oracle_gift = 0.90 + rand_simple() * 0.10;
                self.visionary_sight = 0.85 + rand_simple() * 0.14;
            },
            ProphecySightTopic::ProphecyChannels => {
                self.visionary_sight = 0.95 + rand_simple() * 0.05;
                self.future_perception = 0.90 + rand_simple() * 0.10;
                self.oracle_gift = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.future_perception == 0.0 {
            self.future_perception = (self.prophetic_vision + self.oracle_gift) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_oracle_vision() {
        let mut system = ProphecySightSystem::new(ProphecySightTopic::OracleVision);
        system.analyze_system().unwrap();
        assert!(system.prophetic_vision > 0.8);
    }
}