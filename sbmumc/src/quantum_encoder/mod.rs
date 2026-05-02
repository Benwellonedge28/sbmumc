//! Quantum Encoder Module (566)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumEncoder {
    pub qe_id: String,
    pub encoding_scheme: String,
    pub redundancy: f64,
}

impl QuantumEncoder {
    pub fn new() -> Self {
        Self {
            qe_id: String::from("quantum_encoder_v1"),
            encoding_scheme: String::from(" Stabilizer"),
            redundancy: 2.0,
        }
    }
}

impl Default for QuantumEncoder {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_encoder() {
        let e = QuantumEncoder::new();
        assert!(e.redundancy >= 1.0);
    }
}
