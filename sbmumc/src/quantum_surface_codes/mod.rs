//! Quantum Surface Codes Module (586)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSurfaceCodes {
    pub qsc_id: String,
    pub lattice_size: u32,
    pub distance: u32,
    pub logical_qubits: u32,
}

impl QuantumSurfaceCodes {
    pub fn new() -> Self {
        Self {
            qsc_id: String::from("quantum_surface_codes_v1"),
            lattice_size: 17,
            distance: 3,
            logical_qubits: 1,
        }
    }
}

impl Default for QuantumSurfaceCodes {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_surface() {
        let s = QuantumSurfaceCodes::new();
        assert!(s.distance >= 3);
    }
}
