//! # SBMUMC Module 1455: Ethics Foundations
//!
//! Systems for ethics and normative moral theory.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EthicalFramework {
    Consequentialism,
    Deontology,
    VirtueEthics,
    Contractualism,
    MoralParticularism,
    MoralNaturalism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicsFoundationsSystem {
    pub system_id: String,
    pub ethical_framework: EthicalFramework,
    pub normative_principles: f64,
    pub moral_reasons: f64,
    pub moral_motivation: f64,
    pub moral_judgment: f64,
}

impl EthicsFoundationsSystem {
    pub fn new(ethical_framework: EthicalFramework) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            ethical_framework,
            normative_principles: 0.0,
            moral_reasons: 0.0,
            moral_motivation: 0.0,
            moral_judgment: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.ethical_framework {
            EthicalFramework::Consequentialism => {
                self.normative_principles = 0.95 + rand_simple() * 0.05;
                self.moral_reasons = 0.90 + rand_simple() * 0.10;
                self.moral_motivation = 0.85 + rand_simple() * 0.14;
            },
            EthicalFramework::Deontology => {
                self.moral_judgment = 0.95 + rand_simple() * 0.05;
                self.normative_principles = 0.90 + rand_simple() * 0.10;
                self.moral_reasons = 0.85 + rand_simple() * 0.14;
            },
            EthicalFramework::VirtueEthics => {
                self.moral_motivation = 0.95 + rand_simple() * 0.05;
                self.moral_judgment = 0.90 + rand_simple() * 0.10;
                self.normative_principles = 0.85 + rand_simple() * 0.14;
            },
            EthicalFramework::Contractualism => {
                self.moral_reasons = 0.95 + rand_simple() * 0.05;
                self.moral_motivation = 0.90 + rand_simple() * 0.10;
                self.moral_judgment = 0.85 + rand_simple() * 0.14;
            },
            EthicalFramework::MoralParticularism => {
                self.normative_principles = 0.95 + rand_simple() * 0.05;
                self.moral_judgment = 0.90 + rand_simple() * 0.10;
                self.moral_reasons = 0.85 + rand_simple() * 0.14;
            },
            EthicalFramework::MoralNaturalism => {
                self.moral_reasons = 0.95 + rand_simple() * 0.05;
                self.moral_motivation = 0.90 + rand_simple() * 0.10;
                self.normative_principles = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.moral_motivation == 0.0 {
            self.moral_motivation = (self.normative_principles + self.moral_reasons) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_consequentialism() {
        let mut system = EthicsFoundationsSystem::new(EthicalFramework::Consequentialism);
        system.analyze_system().unwrap();
        assert!(system.normative_principles > 0.8);
    }
}