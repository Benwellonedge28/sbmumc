//! Quantum Measurement Module (576)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMeasurement {
    pub qm_id: String,
    pub measurement_type: MeasurementType,
    pub basis: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasurementType {
    Projective,
    POVM,
    Weak,
    Adaptive,
}

impl QuantumMeasurement {
    pub fn new() -> Self {
        Self {
            qm_id: String::from("quantum_measurement_v1"),
            measurement_type: MeasurementType::POVM,
            basis: vec![String::from("Z"), String::from("X"), String::from("Y")],
        }
    }
}

impl Default for QuantumMeasurement {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_measurement() {
        let m = QuantumMeasurement::new();
        assert_eq!(m.basis.len(), 3);
    }
}
