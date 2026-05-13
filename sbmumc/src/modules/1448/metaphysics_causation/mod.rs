//! # SBMUMC Module 1448: Metaphysics of Causation
//!
//! Systems for metaphysics of causation and causal powers.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CausationTheory {
    Counterfactual,
    Probabilistic,
    Process,
    Dispositional,
    DifferenceMaking,
    PowersOntology,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphysicsCausationSystem {
    pub system_id: String,
    pub causation_theory: CausationTheory,
    pub causal_dependence: f64,
    pub causal_powers: f64,
    pub causal_mechanism: f64,
    pub causal_explanation: f64,
}

impl MetaphysicsCausationSystem {
    pub fn new(causation_theory: CausationTheory) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            causation_theory,
            causal_dependence: 0.0,
            causal_powers: 0.0,
            causal_mechanism: 0.0,
            causal_explanation: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.causation_theory {
            CausationTheory::Counterfactual => {
                self.causal_dependence = 0.95 + rand_simple() * 0.05;
                self.causal_powers = 0.90 + rand_simple() * 0.10;
                self.causal_mechanism = 0.85 + rand_simple() * 0.14;
            },
            CausationTheory::Probabilistic => {
                self.causal_explanation = 0.95 + rand_simple() * 0.05;
                self.causal_dependence = 0.90 + rand_simple() * 0.10;
                self.causal_powers = 0.85 + rand_simple() * 0.14;
            },
            CausationTheory::Process => {
                self.causal_mechanism = 0.95 + rand_simple() * 0.05;
                self.causal_explanation = 0.90 + rand_simple() * 0.10;
                self.causal_dependence = 0.85 + rand_simple() * 0.14;
            },
            CausationTheory::Dispositional => {
                self.causal_powers = 0.95 + rand_simple() * 0.05;
                self.causal_mechanism = 0.90 + rand_simple() * 0.10;
                self.causal_explanation = 0.85 + rand_simple() * 0.14;
            },
            CausationTheory::DifferenceMaking => {
                self.causal_dependence = 0.95 + rand_simple() * 0.05;
                self.causal_explanation = 0.90 + rand_simple() * 0.10;
                self.causal_mechanism = 0.85 + rand_simple() * 0.14;
            },
            CausationTheory::PowersOntology => {
                self.causal_explanation = 0.95 + rand_simple() * 0.05;
                self.causal_powers = 0.90 + rand_simple() * 0.10;
                self.causal_dependence = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.causal_mechanism == 0.0 {
            self.causal_mechanism = (self.causal_dependence + self.causal_powers) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_counterfactual() {
        let mut system = MetaphysicsCausationSystem::new(CausationTheory::Counterfactual);
        system.analyze_system().unwrap();
        assert!(system.causal_dependence > 0.8);
    }
}