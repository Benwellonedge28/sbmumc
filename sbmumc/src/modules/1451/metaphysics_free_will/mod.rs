//! # SBMUMC Module 1451: Metaphysics of Free Will
//!
//! Systems for metaphysics of free will and agency.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FreeWillTheory {
    Libertarianism,
    Compatibilism,
    HardDeterminism,
    SemiCompatibilism,
    EventCausal,
    AgentCausal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphysicsFreeWillSystem {
    pub system_id: String,
    pub free_will_theory: FreeWillTheory,
    pub agency_theory: f64,
    pub moral_responsibility: f64,
    pub control_conditions: f64,
    pub volition_capacity: f64,
}

impl MetaphysicsFreeWillSystem {
    pub fn new(free_will_theory: FreeWillTheory) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            free_will_theory,
            agency_theory: 0.0,
            moral_responsibility: 0.0,
            control_conditions: 0.0,
            volition_capacity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.free_will_theory {
            FreeWillTheory::Libertarianism => {
                self.agency_theory = 0.95 + rand_simple() * 0.05;
                self.moral_responsibility = 0.90 + rand_simple() * 0.10;
                self.control_conditions = 0.85 + rand_simple() * 0.14;
            },
            FreeWillTheory::Compatibilism => {
                self.volition_capacity = 0.95 + rand_simple() * 0.05;
                self.agency_theory = 0.90 + rand_simple() * 0.10;
                self.moral_responsibility = 0.85 + rand_simple() * 0.14;
            },
            FreeWillTheory::HardDeterminism => {
                self.control_conditions = 0.95 + rand_simple() * 0.05;
                self.volition_capacity = 0.90 + rand_simple() * 0.10;
                self.agency_theory = 0.85 + rand_simple() * 0.14;
            },
            FreeWillTheory::SemiCompatibilism => {
                self.moral_responsibility = 0.95 + rand_simple() * 0.05;
                self.control_conditions = 0.90 + rand_simple() * 0.10;
                self.volition_capacity = 0.85 + rand_simple() * 0.14;
            },
            FreeWillTheory::EventCausal => {
                self.agency_theory = 0.95 + rand_simple() * 0.05;
                self.moral_responsibility = 0.90 + rand_simple() * 0.10;
                self.volition_capacity = 0.85 + rand_simple() * 0.14;
            },
            FreeWillTheory::AgentCausal => {
                self.control_conditions = 0.95 + rand_simple() * 0.05;
                self.agency_theory = 0.90 + rand_simple() * 0.10;
                self.moral_responsibility = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.moral_responsibility == 0.0 {
            self.moral_responsibility = (self.agency_theory + self.control_conditions) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_libertarianism() {
        let mut system = MetaphysicsFreeWillSystem::new(FreeWillTheory::Libertarianism);
        system.analyze_system().unwrap();
        assert!(system.agency_theory > 0.8);
    }
}