//! # SBMUMC Module 1444: Epistemology
//!
//! Systems for epistemology and theory of knowledge.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EpistemologyTheories {
    Foundationalism,
    Coherentism,
    Infinitism,
    Pragmatism,
    ReliabilityTheory,
    Contextualism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpistemologySystem {
    pub system_id: String,
    pub epistemology_theories: EpistemologyTheories,
    pub justification_structure: f64,
    pub knowledge_conditions: f64,
    pub epistemic_value: f64,
    pub epistemic_luck: f64,
}

impl EpistemologySystem {
    pub fn new(epistemology_theories: EpistemologyTheories) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            epistemology_theories,
            justification_structure: 0.0,
            knowledge_conditions: 0.0,
            epistemic_value: 0.0,
            epistemic_luck: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.epistemology_theories {
            EpistemologyTheories::Foundationalism => {
                self.justification_structure = 0.95 + rand_simple() * 0.05;
                self.knowledge_conditions = 0.90 + rand_simple() * 0.10;
                self.epistemic_value = 0.85 + rand_simple() * 0.14;
            },
            EpistemologyTheories::Coherentism => {
                self.epistemic_luck = 0.95 + rand_simple() * 0.05;
                self.justification_structure = 0.90 + rand_simple() * 0.10;
                self.knowledge_conditions = 0.85 + rand_simple() * 0.14;
            },
            EpistemologyTheories::Infinitism => {
                self.knowledge_conditions = 0.95 + rand_simple() * 0.05;
                self.epistemic_luck = 0.90 + rand_simple() * 0.10;
                self.justification_structure = 0.85 + rand_simple() * 0.14;
            },
            EpistemologyTheories::Pragmatism => {
                self.epistemic_value = 0.95 + rand_simple() * 0.05;
                self.justification_structure = 0.90 + rand_simple() * 0.10;
                self.epistemic_luck = 0.85 + rand_simple() * 0.14;
            },
            EpistemologyTheories::ReliabilityTheory => {
                self.justification_structure = 0.95 + rand_simple() * 0.05;
                self.knowledge_conditions = 0.90 + rand_simple() * 0.10;
                self.epistemic_luck = 0.85 + rand_simple() * 0.14;
            },
            EpistemologyTheories::Contextualism => {
                self.knowledge_conditions = 0.95 + rand_simple() * 0.05;
                self.epistemic_value = 0.90 + rand_simple() * 0.10;
                self.justification_structure = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.knowledge_conditions == 0.0 {
            self.knowledge_conditions = (self.justification_structure + self.epistemic_value) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_foundationalism() {
        let mut system = EpistemologySystem::new(EpistemologyTheories::Foundationalism);
        system.analyze_system().unwrap();
        assert!(system.justification_structure > 0.8);
    }
}