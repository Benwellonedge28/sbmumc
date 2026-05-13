//! # SBMUMC Module 1522: Occult Sciences
//!
//! Systems for occult sciences and hidden knowledge.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OccultSciencesTopic {
    OccultPhilosophy,
    SecretKnowledge,
    HiddenTruths,
    ArcaneSciences,
    OccultPractice,
    MysteriousArts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccultSciencesSystem {
    pub system_id: String,
    pub occult_sciences_topic: OccultSciencesTopic,
    pub occult_knowledge: f64,
    pub hidden_sciences: f64,
    pub arcane_wisdom: f64,
    pub mysterious_powers: f64,
}

impl OccultSciencesSystem {
    pub fn new(occult_sciences_topic: OccultSciencesTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            occult_sciences_topic,
            occult_knowledge: 0.0,
            hidden_sciences: 0.0,
            arcane_wisdom: 0.0,
            mysterious_powers: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.occult_sciences_topic {
            OccultSciencesTopic::OccultPhilosophy => {
                self.occult_knowledge = 0.95 + rand_simple() * 0.05;
                self.hidden_sciences = 0.90 + rand_simple() * 0.10;
                self.arcane_wisdom = 0.85 + rand_simple() * 0.14;
            },
            OccultSciencesTopic::SecretKnowledge => {
                self.mysterious_powers = 0.95 + rand_simple() * 0.05;
                self.arcane_wisdom = 0.90 + rand_simple() * 0.10;
                self.hidden_sciences = 0.85 + rand_simple() * 0.14;
            },
            OccultSciencesTopic::HiddenTruths => {
                self.hidden_sciences = 0.95 + rand_simple() * 0.05;
                self.occult_knowledge = 0.90 + rand_simple() * 0.10;
                self.mysterious_powers = 0.85 + rand_simple() * 0.14;
            },
            OccultSciencesTopic::ArcaneSciences => {
                self.arcane_wisdom = 0.95 + rand_simple() * 0.05;
                self.mysterious_powers = 0.90 + rand_simple() * 0.10;
                self.occult_knowledge = 0.85 + rand_simple() * 0.14;
            },
            OccultSciencesTopic::OccultPractice => {
                self.occult_knowledge = 0.95 + rand_simple() * 0.05;
                self.hidden_sciences = 0.90 + rand_simple() * 0.10;
                self.mysterious_powers = 0.85 + rand_simple() * 0.14;
            },
            OccultSciencesTopic::MysteriousArts => {
                self.mysterious_powers = 0.95 + rand_simple() * 0.05;
                self.arcane_wisdom = 0.90 + rand_simple() * 0.10;
                self.hidden_sciences = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.arcane_wisdom == 0.0 {
            self.arcane_wisdom = (self.occult_knowledge + self.hidden_sciences) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_occult_philosophy() {
        let mut system = OccultSciencesSystem::new(OccultSciencesTopic::OccultPhilosophy);
        system.analyze_system().unwrap();
        assert!(system.occult_knowledge > 0.8);
    }
}