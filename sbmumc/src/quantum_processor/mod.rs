//! Quantum Processor Module (568)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumProcessor {
    pub qp_id: String,
    pub qubit_count: u32,
    pub gate_fidelity: f64,
    pub connectivity: ConnectivityType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectivityType {
    AllToAll,
    Grid,
    HeavyHex,
    Linear,
}

impl QuantumProcessor {
    pub fn new() -> Self {
        Self {
            qp_id: String::from("quantum_processor_v1"),
            qubit_count: 1024,
            gate_fidelity: 0.999,
            connectivity: ConnectivityType::HeavyHex,
        }
    }
}

impl Default for QuantumProcessor {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_processor() {
        let p = QuantumProcessor::new();
        assert!(p.gate_fidelity > 0.99);
    }
}
