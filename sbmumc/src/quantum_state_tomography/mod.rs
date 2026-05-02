//! Quantum State Tomography Module (573)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumStateTomography {
    pub qst_id: String,
    pub qubit_count: u32,
    pub measurement_count: u32,
    pub reconstruction_fidelity: f64,
}

impl QuantumStateTomography {
    pub fn new() -> Self {
        Self {
            qst_id: String::from("quantum_state_tomography_v1"),
            qubit_count: 2,
            measurement_count: 1000,
            reconstruction_fidelity: 0.98,
        }
    }

    pub fn sample_complexity(&self) -> u32 {
        4_u32.pow(self.qubit_count)
    }
}

impl Default for QuantumStateTomography {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_tomography() {
        let t = QuantumStateTomography::new();
        assert_eq!(t.sample_complexity(), 16);
    }
}
