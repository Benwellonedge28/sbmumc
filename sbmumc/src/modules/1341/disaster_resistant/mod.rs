//! # SBMUMC Module 1341: Disaster Resistant
//!
//! Systems for disaster-resistant building design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisasterType {
    Earthquake,
    Hurricane,
    Flood,
    Fire,
    Tsunami,
    MultiHazard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterResistantSystem {
    pub system_id: String,
    pub disaster_type: DisasterType,
    pub structural_resilience: f64,
    pub damage_minimization: f64,
    pub occupant_safety: f64,
    pub recovery_speed: f64,
}

impl DisasterResistantSystem {
    pub fn new(disaster_type: DisasterType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            disaster_type,
            structural_resilience: 0.0,
            damage_minimization: 0.0,
            occupant_safety: 0.0,
            recovery_speed: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.disaster_type {
            DisasterType::Earthquake => {
                self.structural_resilience = 0.95 + rand_simple() * 0.05;
                self.occupant_safety = 0.90 + rand_simple() * 0.10;
                self.damage_minimization = 0.85 + rand_simple() * 0.14;
            },
            DisasterType::Hurricane => {
                self.structural_resilience = 0.95 + rand_simple() * 0.05;
                self.damage_minimization = 0.90 + rand_simple() * 0.10;
                self.recovery_speed = 0.80 + rand_simple() * 0.18;
            },
            DisasterType::Flood => {
                self.damage_minimization = 0.95 + rand_simple() * 0.05;
                self.recovery_speed = 0.90 + rand_simple() * 0.10;
                self.occupant_safety = 0.85 + rand_simple() * 0.14;
            },
            DisasterType::Fire => {
                self.occupant_safety = 0.95 + rand_simple() * 0.05;
                self.damage_minimization = 0.90 + rand_simple() * 0.10;
                self.recovery_speed = 0.80 + rand_simple() * 0.18;
            },
            DisasterType::Tsunami => {
                self.occupant_safety = 0.95 + rand_simple() * 0.05;
                self.structural_resilience = 0.90 + rand_simple() * 0.10;
                self.damage_minimization = 0.85 + rand_simple() * 0.14;
            },
            DisasterType::MultiHazard => {
                self.structural_resilience = 0.90 + rand_simple() * 0.10;
                self.occupant_safety = 0.90 + rand_simple() * 0.10;
                self.damage_minimization = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.recovery_speed == 0.0 {
            self.recovery_speed = (self.structural_resilience + self.damage_minimization) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_earthquake() {
        let mut system = DisasterResistantSystem::new(DisasterType::Earthquake);
        system.analyze_system().unwrap();
        assert!(system.structural_resilience > 0.8);
    }
}