//! # SBMUMC Module 1438: Computability Theory
//!
//! Systems for computability theory and effective procedures.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComputabilityTopic {
    EffectiveEnumerability,
    RelativeComputability,
    TuringFunctionals,
    RecursiveFunctions,
    LambdaCalculus,
    RegisterMachines,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputabilityTheorySystem {
    pub system_id: String,
    pub computability_topic: ComputabilityTopic,
    pub universal_computation: f64,
    pub reduction_techniques: f64,
    pub oracle_computation: f64,
    pub speedup_theorems: f64,
}

impl ComputabilityTheorySystem {
    pub fn new(computability_topic: ComputabilityTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            computability_topic,
            universal_computation: 0.0,
            reduction_techniques: 0.0,
            oracle_computation: 0.0,
            speedup_theorems: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.computability_topic {
            ComputabilityTopic::EffectiveEnumerability => {
                self.universal_computation = 0.95 + rand_simple() * 0.05;
                self.reduction_techniques = 0.90 + rand_simple() * 0.10;
                self.oracle_computation = 0.85 + rand_simple() * 0.14;
            },
            ComputabilityTopic::RelativeComputability => {
                self.speedup_theorems = 0.95 + rand_simple() * 0.05;
                self.universal_computation = 0.90 + rand_simple() * 0.10;
                self.reduction_techniques = 0.85 + rand_simple() * 0.14;
            },
            ComputabilityTopic::TuringFunctionals => {
                self.oracle_computation = 0.95 + rand_simple() * 0.05;
                self.speedup_theorems = 0.90 + rand_simple() * 0.10;
                self.universal_computation = 0.85 + rand_simple() * 0.14;
            },
            ComputabilityTopic::RecursiveFunctions => {
                self.reduction_techniques = 0.95 + rand_simple() * 0.05;
                self.oracle_computation = 0.90 + rand_simple() * 0.10;
                self.speedup_theorems = 0.85 + rand_simple() * 0.14;
            },
            ComputabilityTopic::LambdaCalculus => {
                self.universal_computation = 0.95 + rand_simple() * 0.05;
                self.reduction_techniques = 0.90 + rand_simple() * 0.10;
                self.speedup_theorems = 0.85 + rand_simple() * 0.14;
            },
            ComputabilityTopic::RegisterMachines => {
                self.reduction_techniques = 0.95 + rand_simple() * 0.05;
                self.universal_computation = 0.90 + rand_simple() * 0.10;
                self.oracle_computation = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.speedup_theorems == 0.0 {
            self.speedup_theorems = (self.universal_computation + self.reduction_techniques) / 2.0 * (0.6 + rand_simple() * 0.3);
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
        let mut system = ComputabilityTheorySystem::new(ComputabilityTopic::TuringFunctionals);
        system.analyze_system().unwrap();
        assert!(system.oracle_computation > 0.8);
    }
}