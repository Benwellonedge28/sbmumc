//! Quantum Control Systems Module (577)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumControlSystems {
    pub qcs_id: String,
    pub control_algorithm: ControlAlgorithm,
    pub pulse_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlAlgorithm {
    GRAPE,
    CRAB,
    NelderMead,
    GradientAscent,
}

impl QuantumControlSystems {
    pub fn new() -> Self {
        Self {
            qcs_id: String::from("quantum_control_systems_v1"),
            control_algorithm: ControlAlgorithm::GRAPE,
            pulse_count: 100,
        }
    }
}

impl Default for QuantumControlSystems {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_control() {
        let c = QuantumControlSystems::new();
        assert_eq!(c.control_algorithm, ControlAlgorithm::GRAPE);
    }
}
