//! # SBMUMC Module 1508: Demonic Domain
//!
//! Systems for demonic domain and dark entities.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DemonicDomainTopic {
    DemonicEntities,
    ShadowRealm,
    InfernalPowers,
    DarkConsciousness,
    PossessionEntities,
    DarkArchons,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemonicDomainSystem {
    pub system_id: String,
    pub demonic_domain_topic: DemonicDomainTopic,
    pub shadow_presence: f64,
    pub infernal_energy: f64,
    pub dark_influence: f64,
    pub malevolent_forces: f64,
}

impl DemonicDomainSystem {
    pub fn new(demonic_domain_topic: DemonicDomainTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            demonic_domain_topic,
            shadow_presence: 0.0,
            infernal_energy: 0.0,
            dark_influence: 0.0,
            malevolent_forces: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.demonic_domain_topic {
            DemonicDomainTopic::DemonicEntities => {
                self.shadow_presence = 0.95 + rand_simple() * 0.05;
                self.infernal_energy = 0.90 + rand_simple() * 0.10;
                self.dark_influence = 0.85 + rand_simple() * 0.14;
            },
            DemonicDomainTopic::ShadowRealm => {
                self.malevolent_forces = 0.95 + rand_simple() * 0.05;
                self.dark_influence = 0.90 + rand_simple() * 0.10;
                self.infernal_energy = 0.85 + rand_simple() * 0.14;
            },
            DemonicDomainTopic::InfernalPowers => {
                self.infernal_energy = 0.95 + rand_simple() * 0.05;
                self.shadow_presence = 0.90 + rand_simple() * 0.10;
                self.malevolent_forces = 0.85 + rand_simple() * 0.14;
            },
            DemonicDomainTopic::DarkConsciousness => {
                self.dark_influence = 0.95 + rand_simple() * 0.05;
                self.malevolent_forces = 0.90 + rand_simple() * 0.10;
                self.shadow_presence = 0.85 + rand_simple() * 0.14;
            },
            DemonicDomainTopic::PossessionEntities => {
                self.shadow_presence = 0.95 + rand_simple() * 0.05;
                self.infernal_energy = 0.90 + rand_simple() * 0.10;
                self.malevolent_forces = 0.85 + rand_simple() * 0.14;
            },
            DemonicDomainTopic::DarkArchons => {
                self.malevolent_forces = 0.95 + rand_simple() * 0.05;
                self.dark_influence = 0.90 + rand_simple() * 0.10;
                self.infernal_energy = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.dark_influence == 0.0 {
            self.dark_influence = (self.shadow_presence + self.infernal_energy) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_demonic_entities() {
        let mut system = DemonicDomainSystem::new(DemonicDomainTopic::DemonicEntities);
        system.analyze_system().unwrap();
        assert!(system.shadow_presence > 0.8);
    }
}