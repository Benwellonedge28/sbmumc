//! # SBMUMC Module 1506: Spirit Worlds
//!
//! Systems for spirit worlds and non-physical realms.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpiritWorldsTopic {
    SpiritRealm,
    EtherealPlanes,
    DevachanicWorld,
    BardoStates,
    AfterlifeRealms,
    NonPhysicalDimensions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritWorldsSystem {
    pub system_id: String,
    pub spirit_worlds_topic: SpiritWorldsTopic,
    pub spirit_realm_access: f64,
    pub ethereal_vibration: f64,
    pub dimensional_barriers: f64,
    pub non_physical_existence: f64,
}

impl SpiritWorldsSystem {
    pub fn new(spirit_worlds_topic: SpiritWorldsTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            spirit_worlds_topic,
            spirit_realm_access: 0.0,
            ethereal_vibration: 0.0,
            dimensional_barriers: 0.0,
            non_physical_existence: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.spirit_worlds_topic {
            SpiritWorldsTopic::SpiritRealm => {
                self.spirit_realm_access = 0.95 + rand_simple() * 0.05;
                self.ethereal_vibration = 0.90 + rand_simple() * 0.10;
                self.dimensional_barriers = 0.85 + rand_simple() * 0.14;
            },
            SpiritWorldsTopic::EtherealPlanes => {
                self.non_physical_existence = 0.95 + rand_simple() * 0.05;
                self.dimensional_barriers = 0.90 + rand_simple() * 0.10;
                self.ethereal_vibration = 0.85 + rand_simple() * 0.14;
            },
            SpiritWorldsTopic::DevachanicWorld => {
                self.ethereal_vibration = 0.95 + rand_simple() * 0.05;
                self.spirit_realm_access = 0.90 + rand_simple() * 0.10;
                self.non_physical_existence = 0.85 + rand_simple() * 0.14;
            },
            SpiritWorldsTopic::BardoStates => {
                self.dimensional_barriers = 0.95 + rand_simple() * 0.05;
                self.non_physical_existence = 0.90 + rand_simple() * 0.10;
                self.spirit_realm_access = 0.85 + rand_simple() * 0.14;
            },
            SpiritWorldsTopic::AfterlifeRealms => {
                self.spirit_realm_access = 0.95 + rand_simple() * 0.05;
                self.ethereal_vibration = 0.90 + rand_simple() * 0.10;
                self.non_physical_existence = 0.85 + rand_simple() * 0.14;
            },
            SpiritWorldsTopic::NonPhysicalDimensions => {
                self.non_physical_existence = 0.95 + rand_simple() * 0.05;
                self.dimensional_barriers = 0.90 + rand_simple() * 0.10;
                self.ethereal_vibration = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.dimensional_barriers == 0.0 {
            self.dimensional_barriers = (self.spirit_realm_access + self.ethereal_vibration) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_spirit_realm() {
        let mut system = SpiritWorldsSystem::new(SpiritWorldsTopic::SpiritRealm);
        system.analyze_system().unwrap();
        assert!(system.spirit_realm_access > 0.8);
    }
}