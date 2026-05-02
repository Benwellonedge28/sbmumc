//! Quantum Sensing Module (550)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSensing {
    pub qs_id: String,
    pub sensing_type: SensingType,
    pub precision_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensingType {
    Magnetometry,
    Gravimetry,
    Photometry,
    AtomicClock,
    Gyroscope,
}

impl QuantumSensing {
    pub fn new() -> Self {
        Self {
            qs_id: String::from("quantum_sensing_v1"),
            sensing_type: SensingType::Magnetometry,
            precision_level: 1e-15,
        }
    }

    pub fn heisenberg_limit(&self) -> f64 {
        1.0 / self.precision_level.sqrt()
    }
}

impl Default for QuantumSensing {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_sensing() {
        let sensor = QuantumSensing::new();
        assert!(sensor.precision_level < 1e-10);
    }
}
