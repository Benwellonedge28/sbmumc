//! # SBMUMC Module 1541: Dimensional Magic
//!
//! Systems for dimensional magic and realm shifting.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DimensionalMagicTopic {
    PortalMagic,
    DimensionShift,
    RealmWalking,
    DimensionalRifts,
    MultiverseAccess,
    PlaneShift,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionalMagicSystem {
    pub system_id: String,
    pub dimensional_magic_topic: DimensionalMagicTopic,
    pub dimensional_power: f64,
    pub realm_access: f64,
    pub portal_mastery: f64,
    pub dimensional_control: f64,
}

impl DimensionalMagicSystem {
    pub fn new(dimensional_magic_topic: DimensionalMagicTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            dimensional_magic_topic,
            dimensional_power: 0.0,
            realm_access: 0.0,
            portal_mastery: 0.0,
            dimensional_control: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.dimensional_magic_topic {
            DimensionalMagicTopic::PortalMagic => {
                self.dimensional_power = 0.95 + rand_simple() * 0.05;
                self.realm_access = 0.90 + rand_simple() * 0.10;
                self.portal_mastery = 0.85 + rand_simple() * 0.14;
            },
            DimensionalMagicTopic::DimensionShift => {
                self.dimensional_control = 0.95 + rand_simple() * 0.05;
                self.portal_mastery = 0.90 + rand_simple() * 0.10;
                self.realm_access = 0.85 + rand_simple() * 0.14;
            },
            DimensionalMagicTopic::RealmWalking => {
                self.realm_access = 0.95 + rand_simple() * 0.05;
                self.dimensional_power = 0.90 + rand_simple() * 0.10;
                self.dimensional_control = 0.85 + rand_simple() * 0.14;
            },
            DimensionalMagicTopic::DimensionalRifts => {
                self.portal_mastery = 0.95 + rand_simple() * 0.05;
                self.dimensional_control = 0.90 + rand_simple() * 0.10;
                self.dimensional_power = 0.85 + rand_simple() * 0.14;
            },
            DimensionalMagicTopic::MultiverseAccess => {
                self.dimensional_power = 0.95 + rand_simple() * 0.05;
                self.realm_access = 0.90 + rand_simple() * 0.10;
                self.dimensional_control = 0.85 + rand_simple() * 0.14;
            },
            DimensionalMagicTopic::PlaneShift => {
                self.dimensional_control = 0.95 + rand_simple() * 0.05;
                self.portal_mastery = 0.90 + rand_simple() * 0.10;
                self.realm_access = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.portal_mastery == 0.0 {
            self.portal_mastery = (self.dimensional_power + self.realm_access) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_portal_magic() {
        let mut system = DimensionalMagicSystem::new(DimensionalMagicTopic::PortalMagic);
        system.analyze_system().unwrap();
        assert!(system.dimensional_power > 0.8);
    }
}