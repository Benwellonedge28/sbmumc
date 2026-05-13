//! # SBMUMC Module 1445: Metaphysics of Consciousness
//!
//! Systems for metaphysics of consciousness and phenomenal experience.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsciousnessTheory {
    Dualism,
    Physicalism,
    Panpsychism,
    Illusionism,
    HigherOrder,
    GlobalWorkspace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphysicsConsciousnessSystem {
    pub system_id: String,
    pub consciousness_theory: ConsciousnessTheory,
    pub phenomenal_properties: f64,
    pub access_consciousness: f64,
    pub self_awareness: f64,
    pub consciousness_explanatory: f64,
}

impl MetaphysicsConsciousnessSystem {
    pub fn new(consciousness_theory: ConsciousnessTheory) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            consciousness_theory,
            phenomenal_properties: 0.0,
            access_consciousness: 0.0,
            self_awareness: 0.0,
            consciousness_explanatory: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.consciousness_theory {
            ConsciousnessTheory::Dualism => {
                self.phenomenal_properties = 0.95 + rand_simple() * 0.05;
                self.access_consciousness = 0.90 + rand_simple() * 0.10;
                self.self_awareness = 0.85 + rand_simple() * 0.14;
            },
            ConsciousnessTheory::Physicalism => {
                self.consciousness_explanatory = 0.95 + rand_simple() * 0.05;
                self.phenomenal_properties = 0.90 + rand_simple() * 0.10;
                self.access_consciousness = 0.85 + rand_simple() * 0.14;
            },
            ConsciousnessTheory::Panpsychism => {
                self.self_awareness = 0.95 + rand_simple() * 0.05;
                self.consciousness_explanatory = 0.90 + rand_simple() * 0.10;
                self.phenomenal_properties = 0.85 + rand_simple() * 0.14;
            },
            ConsciousnessTheory::Illusionism => {
                self.access_consciousness = 0.95 + rand_simple() * 0.05;
                self.self_awareness = 0.90 + rand_simple() * 0.10;
                self.consciousness_explanatory = 0.85 + rand_simple() * 0.14;
            },
            ConsciousnessTheory::HigherOrder => {
                self.phenomenal_properties = 0.95 + rand_simple() * 0.05;
                self.consciousness_explanatory = 0.90 + rand_simple() * 0.10;
                self.self_awareness = 0.85 + rand_simple() * 0.14;
            },
            ConsciousnessTheory::GlobalWorkspace => {
                self.access_consciousness = 0.95 + rand_simple() * 0.05;
                self.phenomenal_properties = 0.90 + rand_simple() * 0.10;
                self.consciousness_explanatory = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.self_awareness == 0.0 {
            self.self_awareness = (self.phenomenal_properties + self.access_consciousness) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_dualism() {
        let mut system = MetaphysicsConsciousnessSystem::new(ConsciousnessTheory::Dualism);
        system.analyze_system().unwrap();
        assert!(system.phenomenal_properties > 0.8);
    }
}