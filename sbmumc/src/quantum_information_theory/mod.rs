//! Quantum Information Theory Module (562)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumInformationTheory {
    pub qit_id: String,
    pub entropy_type: EntropyType,
    pub channel_capacity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntropyType {
    VonNeumann,
    Renyi,
    Tsallis,
    Relative,
}

impl QuantumInformationTheory {
    pub fn new() -> Self {
        Self {
            qit_id: String::from("quantum_information_theory_v1"),
            entropy_type: EntropyType::VonNeumann,
            channel_capacity: 1.0,
        }
    }

    pub fn calculate_entropy(&self, eigenvalues: &[f64]) -> f64 {
        eigenvalues.iter().map(|x| if *x > 0.0 { -x * x.ln() } else { 0.0 }).sum()
    }
}

impl Default for QuantumInformationTheory {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_info_theory() {
        let it = QuantumInformationTheory::new();
        let entropy = it.calculate_entropy(&[0.5, 0.5]);
        assert!(entropy > 0.0);
    }
}
