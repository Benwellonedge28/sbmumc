//! Quantum Cluster States Module (584)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumClusterStates {
    pub qcs_id: String,
    pub lattice_type: LatticeType,
    pub cluster_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LatticeType {
    Square,
    Hexagonal,
    Cubic,
}

impl QuantumClusterStates {
    pub fn new() -> Self {
        Self {
            qcs_id: String::from("quantum_cluster_states_v1"),
            lattice_type: LatticeType::Square,
            cluster_size: 100,
        }
    }
}

impl Default for QuantumClusterStates {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_cluster() {
        let c = QuantumClusterStates::new();
        assert!(c.cluster_size > 0);
    }
}
