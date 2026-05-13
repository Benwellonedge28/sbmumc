//! # SBMUMC Module 1443: Ontology
//!
//! Systems for ontology and the nature of being.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OntologyFramework {
    Mereology,
    Tropes,
    Events,
    Property,
    Substance,
    Processes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologySystem {
    pub system_id: String,
    pub ontology_framework: OntologyFramework,
    pub existence_theory: f64,
    pub identity_criteria: f64,
    pub ontological_dependence: f64,
    pub grounding_relations: f64,
}

impl OntologySystem {
    pub fn new(ontology_framework: OntologyFramework) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            ontology_framework,
            existence_theory: 0.0,
            identity_criteria: 0.0,
            ontological_dependence: 0.0,
            grounding_relations: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.ontology_framework {
            OntologyFramework::Mereology => {
                self.existence_theory = 0.95 + rand_simple() * 0.05;
                self.identity_criteria = 0.90 + rand_simple() * 0.10;
                self.ontological_dependence = 0.85 + rand_simple() * 0.14;
            },
            OntologyFramework::Tropes => {
                self.grounding_relations = 0.95 + rand_simple() * 0.05;
                self.existence_theory = 0.90 + rand_simple() * 0.10;
                self.identity_criteria = 0.85 + rand_simple() * 0.14;
            },
            OntologyFramework::Events => {
                self.ontological_dependence = 0.95 + rand_simple() * 0.05;
                self.grounding_relations = 0.90 + rand_simple() * 0.10;
                self.existence_theory = 0.85 + rand_simple() * 0.14;
            },
            OntologyFramework::Property => {
                self.identity_criteria = 0.95 + rand_simple() * 0.05;
                self.ontological_dependence = 0.90 + rand_simple() * 0.10;
                self.grounding_relations = 0.85 + rand_simple() * 0.14;
            },
            OntologyFramework::Substance => {
                self.existence_theory = 0.95 + rand_simple() * 0.05;
                self.identity_criteria = 0.90 + rand_simple() * 0.10;
                self.grounding_relations = 0.85 + rand_simple() * 0.14;
            },
            OntologyFramework::Processes => {
                self.grounding_relations = 0.95 + rand_simple() * 0.05;
                self.ontological_dependence = 0.90 + rand_simple() * 0.10;
                self.identity_criteria = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.grounding_relations == 0.0 {
            self.grounding_relations = (self.existence_theory + self.identity_criteria) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_mereology() {
        let mut system = OntologySystem::new(OntologyFramework::Mereology);
        system.analyze_system().unwrap();
        assert!(system.existence_theory > 0.8);
    }
}