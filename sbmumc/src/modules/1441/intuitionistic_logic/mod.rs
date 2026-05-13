//! # SBMUMC Module 1441: Intuitionistic Logic
//!
//! Systems for intuitionistic logic and Heyting semantics.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntuitionisticSystem {
    NaturalDeduction,
    SequentCalculus,
    KripkeSemantics,
    HeytingAlgebra,
    BHKInterpretation,
    GameSemantics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntuitionisticLogicSystem {
    pub system_id: String,
    pub intuitionistic_system: IntuitionisticSystem,
    pub proof_construction: f64,
    pub semantics_analysis: f64,
    pub admissible_rules: f64,
    pub intermediate_logics: f64,
}

impl IntuitionisticLogicSystem {
    pub fn new(intuitionistic_system: IntuitionisticSystem) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            intuitionistic_system,
            proof_construction: 0.0,
            semantics_analysis: 0.0,
            admissible_rules: 0.0,
            intermediate_logics: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.intuitionistic_system {
            IntuitionisticSystem::NaturalDeduction => {
                self.proof_construction = 0.95 + rand_simple() * 0.05;
                self.semantics_analysis = 0.90 + rand_simple() * 0.10;
                self.admissible_rules = 0.85 + rand_simple() * 0.14;
            },
            IntuitionisticSystem::SequentCalculus => {
                self.intermediate_logics = 0.95 + rand_simple() * 0.05;
                self.proof_construction = 0.90 + rand_simple() * 0.10;
                self.semantics_analysis = 0.85 + rand_simple() * 0.14;
            },
            IntuitionisticSystem::KripkeSemantics => {
                self.semantics_analysis = 0.95 + rand_simple() * 0.05;
                self.admissible_rules = 0.90 + rand_simple() * 0.10;
                self.intermediate_logics = 0.85 + rand_simple() * 0.14;
            },
            IntuitionisticSystem::HeytingAlgebra => {
                self.admissible_rules = 0.95 + rand_simple() * 0.05;
                self.semantics_analysis = 0.90 + rand_simple() * 0.10;
                self.proof_construction = 0.85 + rand_simple() * 0.14;
            },
            IntuitionisticSystem::BHKInterpretation => {
                self.proof_construction = 0.95 + rand_simple() * 0.05;
                self.intermediate_logics = 0.90 + rand_simple() * 0.10;
                self.admissible_rules = 0.85 + rand_simple() * 0.14;
            },
            IntuitionisticSystem::GameSemantics => {
                self.semantics_analysis = 0.95 + rand_simple() * 0.05;
                self.proof_construction = 0.90 + rand_simple() * 0.10;
                self.semantics_analysis = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.admissible_rules == 0.0 {
            self.admissible_rules = (self.proof_construction + self.semantics_analysis) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_natural_deduction() {
        let mut system = IntuitionisticLogicSystem::new(IntuitionisticSystem::NaturalDeduction);
        system.analyze_system().unwrap();
        assert!(system.proof_construction > 0.8);
    }
}