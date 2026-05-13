//! # SBMUMC Module 1511: Fae Realms
//!
//! Systems for fae realms and fairy kingdoms.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FaeRealmsTopic {
    SeelieCourt,
    UnseelieCourt,
    FairyQueen,
    FaeMagic,
    FaeRealmPassage,
    GlamourWeaving,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaeRealmsSystem {
    pub system_id: String,
    pub fae_realms_topic: FaeRealmsTopic,
    pub fae_magic: f64,
    pub fairy_realm: f64,
    pub enchantment_power: f64,
    pub otherworld_connection: f64,
}

impl FaeRealmsSystem {
    pub fn new(fae_realms_topic: FaeRealmsTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            fae_realms_topic,
            fae_magic: 0.0,
            fairy_realm: 0.0,
            enchantment_power: 0.0,
            otherworld_connection: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.fae_realms_topic {
            FaeRealmsTopic::SeelieCourt => {
                self.fae_magic = 0.95 + rand_simple() * 0.05;
                self.fairy_realm = 0.90 + rand_simple() * 0.10;
                self.enchantment_power = 0.85 + rand_simple() * 0.14;
            },
            FaeRealmsTopic::UnseelieCourt => {
                self.otherworld_connection = 0.95 + rand_simple() * 0.05;
                self.enchantment_power = 0.90 + rand_simple() * 0.10;
                self.fairy_realm = 0.85 + rand_simple() * 0.14;
            },
            FaeRealmsTopic::FairyQueen => {
                self.fairy_realm = 0.95 + rand_simple() * 0.05;
                self.fae_magic = 0.90 + rand_simple() * 0.10;
                self.otherworld_connection = 0.85 + rand_simple() * 0.14;
            },
            FaeRealmsTopic::FaeMagic => {
                self.enchantment_power = 0.95 + rand_simple() * 0.05;
                self.otherworld_connection = 0.90 + rand_simple() * 0.10;
                self.fae_magic = 0.85 + rand_simple() * 0.14;
            },
            FaeRealmsTopic::FaeRealmPassage => {
                self.fae_magic = 0.95 + rand_simple() * 0.05;
                self.fairy_realm = 0.90 + rand_simple() * 0.10;
                self.otherworld_connection = 0.85 + rand_simple() * 0.14;
            },
            FaeRealmsTopic::GlamourWeaving => {
                self.otherworld_connection = 0.95 + rand_simple() * 0.05;
                self.enchantment_power = 0.90 + rand_simple() * 0.10;
                self.fairy_realm = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.enchantment_power == 0.0 {
            self.enchantment_power = (self.fae_magic + self.fairy_realm) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_seelie_court() {
        let mut system = FaeRealmsSystem::new(FaeRealmsTopic::SeelieCourt);
        system.analyze_system().unwrap();
        assert!(system.fae_magic > 0.8);
    }
}