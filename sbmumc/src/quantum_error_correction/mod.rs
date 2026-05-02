//! Quantum Error Correction Module (548)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumErrorCorrection {
    pub qec_id: String,
    pub code_type: CodeType,
    pub logical_qubits: u32,
    pub physical_qubits: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeType {
    Surface,
    Color,
    Steane,
    Repetition,
    LatticeSurgery,
}

impl QuantumErrorCorrection {
    pub fn new() -> Self {
        Self {
            qec_id: String::from("quantum_error_correction_v1"),
            code_type: CodeType::Surface,
            logical_qubits: 1,
            physical_qubits: 17,
        }
    }

    pub fn encode_overhead(&self) -> f64 {
        self.physical_qubits as f64 / self.logical_qubits as f64
    }
}

impl Default for QuantumErrorCorrection {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_qec() {
        let qec = QuantumErrorCorrection::new();
        assert_eq!(qec.encode_overhead(), 17.0);
    }
}
