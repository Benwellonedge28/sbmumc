//! # SBMUMC Module 1463: Meta Ethics
//!
//! Systems for meta-ethics and moral semantics.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaEthicsTopic {
    MoralRealism,
    AntiRealismMeta,
    ErrorTheory,
    NonCognitivism,
    Expressivism,
    MoralSemantics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaEthicsSystem {
    pub system_id: String,
    pub meta_ethics_topic: MetaEthicsTopic,
    pub moral_truth_candidates: f64,
    pub moral_reasons_nature: f64,
    pub normative_judgments: f64,
    pub moral_motivation_nature: f64,
}

impl MetaEthicsSystem {
    pub fn new(meta_ethics_topic: MetaEthicsTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            meta_ethics_topic,
            moral_truth_candidates: 0.0,
            moral_reasons_nature: 0.0,
            normative_judgments: 0.0,
            moral_motivation_nature: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.meta_ethics_topic {
            MetaEthicsTopic::MoralRealism => {
                self.moral_truth_candidates = 0.95 + rand_simple() * 0.05;
                self.moral_reasons_nature = 0.90 + rand_simple() * 0.10;
                self.normative_judgments = 0.85 + rand_simple() * 0.14;
            },
            MetaEthicsTopic::AntiRealismMeta => {
                self.moral_motivation_nature = 0.95 + rand_simple() * 0.05;
                self.moral_truth_candidates = 0.90 + rand_simple() * 0.10;
                self.moral_reasons_nature = 0.85 + rand_simple() * 0.14;
            },
            MetaEthicsTopic::ErrorTheory => {
                self.normative_judgments = 0.95 + rand_simple() * 0.05;
                self.moral_motivation_nature = 0.90 + rand_simple() * 0.10;
                self.moral_truth_candidates = 0.85 + rand_simple() * 0.14;
            },
            MetaEthicsTopic::NonCognitivism => {
                self.moral_reasons_nature = 0.95 + rand_simple() * 0.05;
                self.normative_judgments = 0.90 + rand_simple() * 0.10;
                self.moral_motivation_nature = 0.85 + rand_simple() * 0.14;
            },
            MetaEthicsTopic::Expressivism => {
                self.moral_truth_candidates = 0.95 + rand_simple() * 0.05;
                self.normative_judgments = 0.90 + rand_simple() * 0.10;
                self.moral_reasons_nature = 0.85 + rand_simple() * 0.14;
            },
            MetaEthicsTopic::MoralSemantics => {
                self.moral_motivation_nature = 0.95 + rand_simple() * 0.05;
                self.moral_reasons_nature = 0.90 + rand_simple() * 0.10;
                self.normative_judgments = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.moral_reasons_nature == 0.0 {
            self.moral_reasons_nature = (self.moral_truth_candidates + self.normative_judgments) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_realism() {
        let mut system = MetaEthicsSystem::new(MetaEthicsTopic::MoralRealism);
        system.analyze_system().unwrap();
        assert!(system.moral_truth_candidates > 0.8);
    }
}