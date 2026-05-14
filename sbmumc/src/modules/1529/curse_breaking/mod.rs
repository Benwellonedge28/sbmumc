//! # SBMUMC Module 1529: Curse Breaking
//!
//! Systems for curse breaking and hex removal.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurseBreakingTopic {
    HexRemoval,
    CurseLifting,
    DarkMagicRemoval,
    HexBreaking,
    NegativeUndoing,
    CurseDispel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurseBreakingSystem {
    pub system_id: String,
    pub curse_breaking_topic: CurseBreakingTopic,
    pub curse_removal: f64,
    pub hex_dispel: f64,
    pub negative_release: f64,
    pub magical_cleansing: f64,
}

impl CurseBreakingSystem {
    pub fn new(curse_breaking_topic: CurseBreakingTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            curse_breaking_topic,
            curse_removal: 0.0,
            hex_dispel: 0.0,
            negative_release: 0.0,
            magical_cleansing: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.curse_breaking_topic {
            CurseBreakingTopic::HexRemoval => {
                self.curse_removal = 0.95 + rand_simple() * 0.05;
                self.hex_dispel = 0.90 + rand_simple() * 0.10;
                self.negative_release = 0.85 + rand_simple() * 0.14;
            },
            CurseBreakingTopic::CurseLifting => {
                self.magical_cleansing = 0.95 + rand_simple() * 0.05;
                self.negative_release = 0.90 + rand_simple() * 0.10;
                self.hex_dispel = 0.85 + rand_simple() * 0.14;
            },
            CurseBreakingTopic::DarkMagicRemoval => {
                self.hex_dispel = 0.95 + rand_simple() * 0.05;
                self.curse_removal = 0.90 + rand_simple() * 0.10;
                self.magical_cleansing = 0.85 + rand_simple() * 0.14;
            },
            CurseBreakingTopic::HexBreaking => {
                self.negative_release = 0.95 + rand_simple() * 0.05;
                self.magical_cleansing = 0.90 + rand_simple() * 0.10;
                self.curse_removal = 0.85 + rand_simple() * 0.14;
            },
            CurseBreakingTopic::NegativeUndoing => {
                self.curse_removal = 0.95 + rand_simple() * 0.05;
                self.hex_dispel = 0.90 + rand_simple() * 0.10;
                self.magical_cleansing = 0.85 + rand_simple() * 0.14;
            },
            CurseBreakingTopic::CurseDispel => {
                self.magical_cleansing = 0.95 + rand_simple() * 0.05;
                self.negative_release = 0.90 + rand_simple() * 0.10;
                self.hex_dispel = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.negative_release == 0.0 {
            self.negative_release = (self.curse_removal + self.hex_dispel) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_hex_removal() {
        let mut system = CurseBreakingSystem::new(CurseBreakingTopic::HexRemoval);
        system.analyze_system().unwrap();
        assert!(system.curse_removal > 0.8);
    }
}