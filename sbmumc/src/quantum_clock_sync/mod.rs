//! Quantum Clock Sync Module (570)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumClockSync {
    pub qcs_id: String,
    pub synchronization_precision_fs: f64,
    pub network_size: u32,
}

impl QuantumClockSync {
    pub fn new() -> Self {
        Self {
            qcs_id: String::from("quantum_clock_sync_v1"),
            synchronization_precision_fs: 100.0,
            network_size: 100,
        }
    }
}

impl Default for QuantumClockSync {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_clock_sync() {
        let cs = QuantumClockSync::new();
        assert!(cs.synchronization_precision_fs > 0.0);
    }
}
