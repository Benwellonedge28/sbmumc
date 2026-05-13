//! # SBMUMC Module 1447: Metaphysics of Time
//!
//! Systems for metaphysics of time and temporal structure.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeTheory {
    ATheory,
    BTheory,
    GrowingBlock,
    Presentism,
    Eternalism,
    Perdurantism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphysicsTimeSystem {
    pub system_id: String,
    pub time_theory: TimeTheory,
    pub temporal_passage: f64,
    pub temporal_ontology: f64,
    pub tensed_facts: f64,
    pub temporal_modality: f64,
}

impl MetaphysicsTimeSystem {
    pub fn new(time_theory: TimeTheory) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            time_theory,
            temporal_passage: 0.0,
            temporal_ontology: 0.0,
            tensed_facts: 0.0,
            temporal_modality: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.time_theory {
            TimeTheory::ATheory => {
                self.temporal_passage = 0.95 + rand_simple() * 0.05;
                self.temporal_ontology = 0.90 + rand_simple() * 0.10;
                self.tensed_facts = 0.85 + rand_simple() * 0.14;
            },
            TimeTheory::BTheory => {
                self.temporal_modality = 0.95 + rand_simple() * 0.05;
                self.temporal_passage = 0.90 + rand_simple() * 0.10;
                self.temporal_ontology = 0.85 + rand_simple() * 0.14;
            },
            TimeTheory::GrowingBlock => {
                self.tensed_facts = 0.95 + rand_simple() * 0.05;
                self.temporal_modality = 0.90 + rand_simple() * 0.10;
                self.temporal_passage = 0.85 + rand_simple() * 0.14;
            },
            TimeTheory::Presentism => {
                self.temporal_ontology = 0.95 + rand_simple() * 0.05;
                self.tensed_facts = 0.90 + rand_simple() * 0.10;
                self.temporal_modality = 0.85 + rand_simple() * 0.14;
            },
            TimeTheory::Eternalism => {
                self.temporal_passage = 0.95 + rand_simple() * 0.05;
                self.temporal_ontology = 0.90 + rand_simple() * 0.10;
                self.tensed_facts = 0.85 + rand_simple() * 0.14;
            },
            TimeTheory::Perdurantism => {
                self.temporal_modality = 0.95 + rand_simple() * 0.05;
                self.tensed_facts = 0.90 + rand_simple() * 0.10;
                self.temporal_ontology = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.tensed_facts == 0.0 {
            self.tensed_facts = (self.temporal_passage + self.temporal_ontology) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_atheory() {
        let mut system = MetaphysicsTimeSystem::new(TimeTheory::ATheory);
        system.analyze_system().unwrap();
        assert!(system.temporal_passage > 0.8);
    }
}