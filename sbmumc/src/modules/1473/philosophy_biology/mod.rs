//! # SBMUMC Module 1473: Philosophy of Biology
//!
//! Systems for philosophy of biology and life sciences.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhilosophyBiologyTopic {
    EvolutionTheory,
    NaturalSelection,
    GeneticReductionism,
    BiologicalIndividuality,
    SpeciesConcepts,
    FunctionNotions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyBiologySystem {
    pub system_id: String,
    pub philosophy_biology_topic: PhilosophyBiologyTopic,
    pub life_organization: f64,
    pub biological_explanation: f64,
    pub evolutionary_mechanisms: f64,
    pub ontological_status: f64,
}

impl PhilosophyBiologySystem {
    pub fn new(philosophy_biology_topic: PhilosophyBiologyTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            philosophy_biology_topic,
            life_organization: 0.0,
            biological_explanation: 0.0,
            evolutionary_mechanisms: 0.0,
            ontological_status: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.philosophy_biology_topic {
            PhilosophyBiologyTopic::EvolutionTheory => {
                self.life_organization = 0.95 + rand_simple() * 0.05;
                self.biological_explanation = 0.90 + rand_simple() * 0.10;
                self.evolutionary_mechanisms = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyBiologyTopic::NaturalSelection => {
                self.ontological_status = 0.95 + rand_simple() * 0.05;
                self.life_organization = 0.90 + rand_simple() * 0.10;
                self.biological_explanation = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyBiologyTopic::GeneticReductionism => {
                self.evolutionary_mechanisms = 0.95 + rand_simple() * 0.05;
                self.ontological_status = 0.90 + rand_simple() * 0.10;
                self.life_organization = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyBiologyTopic::BiologicalIndividuality => {
                self.biological_explanation = 0.95 + rand_simple() * 0.05;
                self.evolutionary_mechanisms = 0.90 + rand_simple() * 0.10;
                self.ontological_status = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyBiologyTopic::SpeciesConcepts => {
                self.life_organization = 0.95 + rand_simple() * 0.05;
                self.ontological_status = 0.90 + rand_simple() * 0.10;
                self.biological_explanation = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyBiologyTopic::FunctionNotions => {
                self.biological_explanation = 0.95 + rand_simple() * 0.05;
                self.evolutionary_mechanisms = 0.90 + rand_simple() * 0.10;
                self.ontological_status = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.ontological_status == 0.0 {
            self.ontological_status = (self.life_organization + self.biological_explanation) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_evolution() {
        let mut system = PhilosophyBiologySystem::new(PhilosophyBiologyTopic::EvolutionTheory);
        system.analyze_system().unwrap();
        assert!(system.life_organization > 0.8);
    }
}