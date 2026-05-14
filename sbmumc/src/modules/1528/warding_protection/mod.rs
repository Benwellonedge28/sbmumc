//! # SBMUMC Module 1528: Warding Protection
//!
//! Systems for warding and protective magic.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WardingProtectionTopic {
    ProtectiveWards,
    MagicalShielding,
    EnergyProtection,
    PsychicProtection,
    SacredProtection,
    WardingSymbols,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WardingProtectionSystem {
    pub system_id: String,
    pub warding_protection_topic: WardingProtectionTopic,
    pub protective_magic: f64,
    pub magical_barriers: f64,
    pub spiritual_shield: f64,
    pub energy_protection: f64,
}

impl WardingProtectionSystem {
    pub fn new(warding_protection_topic: WardingProtectionTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            warding_protection_topic,
            protective_magic: 0.0,
            magical_barriers: 0.0,
            spiritual_shield: 0.0,
            energy_protection: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.warding_protection_topic {
            WardingProtectionTopic::ProtectiveWards => {
                self.protective_magic = 0.95 + rand_simple() * 0.05;
                self.magical_barriers = 0.90 + rand_simple() * 0.10;
                self.spiritual_shield = 0.85 + rand_simple() * 0.14;
            },
            WardingProtectionTopic::MagicalShielding => {
                self.energy_protection = 0.95 + rand_simple() * 0.05;
                self.spiritual_shield = 0.90 + rand_simple() * 0.10;
                self.magical_barriers = 0.85 + rand_simple() * 0.14;
            },
            WardingProtectionTopic::EnergyProtection => {
                self.magical_barriers = 0.95 + rand_simple() * 0.05;
                self.protective_magic = 0.90 + rand_simple() * 0.10;
                self.energy_protection = 0.85 + rand_simple() * 0.14;
            },
            WardingProtectionTopic::PsychicProtection => {
                self.spiritual_shield = 0.95 + rand_simple() * 0.05;
                self.energy_protection = 0.90 + rand_simple() * 0.10;
                self.protective_magic = 0.85 + rand_simple() * 0.14;
            },
            WardingProtectionTopic::SacredProtection => {
                self.protective_magic = 0.95 + rand_simple() * 0.05;
                self.magical_barriers = 0.90 + rand_simple() * 0.10;
                self.energy_protection = 0.85 + rand_simple() * 0.14;
            },
            WardingProtectionTopic::WardingSymbols => {
                self.energy_protection = 0.95 + rand_simple() * 0.05;
                self.spiritual_shield = 0.90 + rand_simple() * 0.10;
                self.magical_barriers = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.spiritual_shield == 0.0 {
            self.spiritual_shield = (self.protective_magic + self.magical_barriers) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_protective_wards() {
        let mut system = WardingProtectionSystem::new(WardingProtectionTopic::ProtectiveWards);
        system.analyze_system().unwrap();
        assert!(system.protective_magic > 0.8);
    }
}