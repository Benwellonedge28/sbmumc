//! # SBMUMC Module 1509: Elemental Magic
//!
//! Systems for elemental magic and primal forces.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementalMagicTopic {
    FireElemental,
    WaterElemental,
    EarthElemental,
    AirElemental,
    AetherElement,
    ElementalInvocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementalMagicSystem {
    pub system_id: String,
    pub elemental_magic_topic: ElementalMagicTopic,
    pub elemental_mastery: f64,
    pub primal_forces: f64,
    pub nature_elements: f64,
    pub elemental_balance: f64,
}

impl ElementalMagicSystem {
    pub fn new(elemental_magic_topic: ElementalMagicTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            elemental_magic_topic,
            elemental_mastery: 0.0,
            primal_forces: 0.0,
            nature_elements: 0.0,
            elemental_balance: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.elemental_magic_topic {
            ElementalMagicTopic::FireElemental => {
                self.elemental_mastery = 0.95 + rand_simple() * 0.05;
                self.primal_forces = 0.90 + rand_simple() * 0.10;
                self.nature_elements = 0.85 + rand_simple() * 0.14;
            },
            ElementalMagicTopic::WaterElemental => {
                self.elemental_balance = 0.95 + rand_simple() * 0.05;
                self.nature_elements = 0.90 + rand_simple() * 0.10;
                self.primal_forces = 0.85 + rand_simple() * 0.14;
            },
            ElementalMagicTopic::EarthElemental => {
                self.primal_forces = 0.95 + rand_simple() * 0.05;
                self.elemental_mastery = 0.90 + rand_simple() * 0.10;
                self.elemental_balance = 0.85 + rand_simple() * 0.14;
            },
            ElementalMagicTopic::AirElemental => {
                self.nature_elements = 0.95 + rand_simple() * 0.05;
                self.elemental_balance = 0.90 + rand_simple() * 0.10;
                self.elemental_mastery = 0.85 + rand_simple() * 0.14;
            },
            ElementalMagicTopic::AetherElement => {
                self.elemental_mastery = 0.95 + rand_simple() * 0.05;
                self.primal_forces = 0.90 + rand_simple() * 0.10;
                self.elemental_balance = 0.85 + rand_simple() * 0.14;
            },
            ElementalMagicTopic::ElementalInvocation => {
                self.elemental_balance = 0.95 + rand_simple() * 0.05;
                self.nature_elements = 0.90 + rand_simple() * 0.10;
                self.primal_forces = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.nature_elements == 0.0 {
            self.nature_elements = (self.elemental_mastery + self.primal_forces) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_fire_elemental() {
        let mut system = ElementalMagicSystem::new(ElementalMagicTopic::FireElemental);
        system.analyze_system().unwrap();
        assert!(system.elemental_mastery > 0.8);
    }
}