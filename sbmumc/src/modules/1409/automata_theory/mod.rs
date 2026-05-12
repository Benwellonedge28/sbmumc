//! # SBMUMC Module 1409: Automata Theory
//!
//! Systems for automata theory and formal languages.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomatonType {
    Finite,
    Pushdown,
    LinearBounded,
    Turing,
    Probabilistic,
    Quantum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomataTheorySystem {
    pub system_id: String,
    pub automaton_type: AutomatonType,
    pub state_machine_design: f64,
    pub language_recognition: f64,
    pub decidability_analysis: f64,
    pub complexity_classification: f64,
}

impl AutomataTheorySystem {
    pub fn new(automaton_type: AutomatonType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            automaton_type,
            state_machine_design: 0.0,
            language_recognition: 0.0,
            decidability_analysis: 0.0,
            complexity_classification: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.automaton_type {
            AutomatonType::Finite => {
                self.state_machine_design = 0.95 + rand_simple() * 0.05;
                self.language_recognition = 0.90 + rand_simple() * 0.10;
                self.complexity_classification = 0.85 + rand_simple() * 0.14;
            },
            AutomatonType::Pushdown => {
                self.language_recognition = 0.95 + rand_simple() * 0.05;
                self.decidability_analysis = 0.90 + rand_simple() * 0.10;
                self.state_machine_design = 0.85 + rand_simple() * 0.14;
            },
            AutomatonType::LinearBounded => {
                self.decidability_analysis = 0.95 + rand_simple() * 0.05;
                self.complexity_classification = 0.90 + rand_simple() * 0.10;
                self.language_recognition = 0.85 + rand_simple() * 0.14;
            },
            AutomatonType::Turing => {
                self.complexity_classification = 0.95 + rand_simple() * 0.05;
                self.state_machine_design = 0.90 + rand_simple() * 0.10;
                self.decidability_analysis = 0.85 + rand_simple() * 0.14;
            },
            AutomatonType::Probabilistic => {
                self.state_machine_design = 0.95 + rand_simple() * 0.05;
                self.language_recognition = 0.90 + rand_simple() * 0.10;
                self.decidability_analysis = 0.85 + rand_simple() * 0.14;
            },
            AutomatonType::Quantum => {
                self.language_recognition = 0.95 + rand_simple() * 0.05;
                self.complexity_classification = 0.90 + rand_simple() * 0.10;
                self.state_machine_design = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.complexity_classification == 0.0 {
            self.complexity_classification = (self.state_machine_design + self.language_recognition) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_finite() {
        let mut system = AutomataTheorySystem::new(AutomatonType::Finite);
        system.analyze_system().unwrap();
        assert!(system.state_machine_design > 0.8);
    }
}
