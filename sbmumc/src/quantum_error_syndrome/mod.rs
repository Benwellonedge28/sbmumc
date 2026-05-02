//! Quantum Error Syndrome Module (592)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumErrorSyndrome {
    pub qes_id: String,
    pub syndrome_patterns: Vec<SyndromePattern>,
    pub error_correction_ability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyndromePattern {
    pub pattern_id: String,
    pub bits: Vec<u8>,
}

impl QuantumErrorSyndrome {
    pub fn new() -> Self {
        Self {
            qes_id: String::from("quantum_error_syndrome_v1"),
            syndrome_patterns: vec![],
            error_correction_ability: 0.99,
        }
    }
}

impl Default for QuantumErrorSyndrome {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_syndrome() {
        let s = QuantumErrorSyndrome::new();
        assert!(s.error_correction_ability > 0.95);
    }
}
