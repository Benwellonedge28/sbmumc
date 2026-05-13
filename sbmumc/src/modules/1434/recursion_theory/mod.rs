//! # SBMUMC Module 1434: Recursion Theory
//!
//! Systems for recursion theory and computability.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecursionTopic {
    TuringDegrees,
    Reducibilities,
    ComplexityDegrees,
    EnumerationDegrees,
    WeihrauchDegrees,
    ReverseRecursion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursionTheorySystem {
    pub system_id: String,
    pub recursion_topic: RecursionTopic,
    pub halting_problem: f64,
    pub degree_structure: f64,
    pub oracle_construction: f64,
    pub priority_method: f64,
}

impl RecursionTheorySystem {
    pub fn new(recursion_topic: RecursionTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            recursion_topic,
            halting_problem: 0.0,
            degree_structure: 0.0,
            oracle_construction: 0.0,
            priority_method: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.recursion_topic {
            RecursionTopic::TuringDegrees => {
                self.halting_problem = 0.95 + rand_simple() * 0.05;
                self.degree_structure = 0.90 + rand_simple() * 0.10;
                self.oracle_construction = 0.85 + rand_simple() * 0.14;
            },
            RecursionTopic::Reducibilities => {
                self.priority_method = 0.95 + rand_simple() * 0.05;
                self.halting_problem = 0.90 + rand_simple() * 0.10;
                self.degree_structure = 0.85 + rand_simple() * 0.14;
            },
            RecursionTopic::ComplexityDegrees => {
                self.oracle_construction = 0.95 + rand_simple() * 0.05;
                self.priority_method = 0.90 + rand_simple() * 0.10;
                self.halting_problem = 0.85 + rand_simple() * 0.14;
            },
            RecursionTopic::EnumerationDegrees => {
                self.degree_structure = 0.95 + rand_simple() * 0.05;
                self.oracle_construction = 0.90 + rand_simple() * 0.10;
                self.priority_method = 0.85 + rand_simple() * 0.14;
            },
            RecursionTopic::WeihrauchDegrees => {
                self.halting_problem = 0.95 + rand_simple() * 0.05;
                self.priority_method = 0.90 + rand_simple() * 0.10;
                self.oracle_construction = 0.85 + rand_simple() * 0.14;
            },
            RecursionTopic::ReverseRecursion => {
                self.degree_structure = 0.95 + rand_simple() * 0.05;
                self.halting_problem = 0.90 + rand_simple() * 0.10;
                self.priority_method = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.oracle_construction == 0.0 {
            self.oracle_construction = (self.halting_problem + self.degree_structure) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_turing() {
        let mut system = RecursionTheorySystem::new(RecursionTopic::TuringDegrees);
        system.analyze_system().unwrap();
        assert!(system.halting_problem > 0.8);
    }
}