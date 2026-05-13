//! # SBMUMC Module 1465: Bioethics
//!
//! Systems for bioethics and biomedical ethics.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BioethicsTopic {
    EndOfLife,
    GeneticEthics,
    ReproductiveEthics,
    ResearchEthicsBio,
    AllocationResources,
    AnimalEthics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioethicsSystem {
    pub system_id: String,
    pub bioethics_topic: BioethicsTopic,
    pub life_ethics_principles: f64,
    pub human_dignity: f64,
    pub autonomy_respect: f64,
    pub beneficence_analysis: f64,
}

impl BioethicsSystem {
    pub fn new(bioethics_topic: BioethicsTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            bioethics_topic,
            life_ethics_principles: 0.0,
            human_dignity: 0.0,
            autonomy_respect: 0.0,
            beneficence_analysis: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.bioethics_topic {
            BioethicsTopic::EndOfLife => {
                self.life_ethics_principles = 0.95 + rand_simple() * 0.05;
                self.human_dignity = 0.90 + rand_simple() * 0.10;
                self.autonomy_respect = 0.85 + rand_simple() * 0.14;
            },
            BioethicsTopic::GeneticEthics => {
                self.beneficence_analysis = 0.95 + rand_simple() * 0.05;
                self.life_ethics_principles = 0.90 + rand_simple() * 0.10;
                self.human_dignity = 0.85 + rand_simple() * 0.14;
            },
            BioethicsTopic::ReproductiveEthics => {
                self.autonomy_respect = 0.95 + rand_simple() * 0.05;
                self.beneficence_analysis = 0.90 + rand_simple() * 0.10;
                self.life_ethics_principles = 0.85 + rand_simple() * 0.14;
            },
            BioethicsTopic::ResearchEthicsBio => {
                self.human_dignity = 0.95 + rand_simple() * 0.05;
                self.autonomy_respect = 0.90 + rand_simple() * 0.10;
                self.beneficence_analysis = 0.85 + rand_simple() * 0.14;
            },
            BioethicsTopic::AllocationResources => {
                self.life_ethics_principles = 0.95 + rand_simple() * 0.05;
                self.beneficence_analysis = 0.90 + rand_simple() * 0.10;
                self.autonomy_respect = 0.85 + rand_simple() * 0.14;
            },
            BioethicsTopic::AnimalEthics => {
                self.human_dignity = 0.95 + rand_simple() * 0.05;
                self.life_ethics_principles = 0.90 + rand_simple() * 0.10;
                self.beneficence_analysis = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.autonomy_respect == 0.0 {
            self.autonomy_respect = (self.life_ethics_principles + self.human_dignity) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_end_of_life() {
        let mut system = BioethicsSystem::new(BioethicsTopic::EndOfLife);
        system.analyze_system().unwrap();
        assert!(system.life_ethics_principles > 0.8);
    }
}