//! Quantum Metrology Module (552)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMetrology {
    pub qm_id: String,
    pub measurement_precision: f64,
    pub resource_count: u32,
}

impl QuantumMetrology {
    pub fn new() -> Self {
        Self {
            qm_id: String::from("quantum_metrology_v1"),
            measurement_precision: 1e-10,
            resource_count: 100,
        }
    }

    pub fn sql_limit(&self) -> f64 {
        1.0 / (self.resource_count as f64).sqrt()
    }

    pub fn heisenberg_limit(&self) -> f64 {
        1.0 / self.resource_count as f64
    }
}

impl Default for QuantumMetrology {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_metrology() {
        let m = QuantumMetrology::new();
        assert!(m.sql_limit() > m.heisenberg_limit());
    }
}
