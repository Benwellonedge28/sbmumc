//! Quantum Key Distribution Module (501)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumKeyDistribution {
    pub qkd_id: String,
    pub protocol: QkdProtocol,
    pub key_rate_kbps: f64,
    pub distance_km: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QkdProtocol {
    BB84,
    E91,
    SARG04,
    MeasurementDeviceIndependent,
    TwinField,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QkdKey {
    pub key_id: String,
    pub raw_bits: Vec<u8>,
    pub error_rate: f64,
    pub security_level: SecurityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    InformationTheoretic,
    Computational,
    DeviceIndependent,
}

impl QuantumKeyDistribution {
    pub fn new() -> Self {
        Self {
            qkd_id: String::from("quantum_key_distribution_v1"),
            protocol: QkdProtocol::BB84,
            key_rate_kbps: 0.0,
            distance_km: 0.0,
        }
    }

    pub fn generate_key(&mut self, length: usize) -> QkdKey {
        QkdKey {
            key_id: format!("key_{}", length),
            raw_bits: (0..length).map(|_| rand_u8()).collect(),
            error_rate: 0.01,
            security_level: SecurityLevel::InformationTheoretic,
        }
    }
}

fn rand_u8() -> u8 { use std::time::SystemTime; (SystemTime::now().elapsed().unwrap().as_nanos() % 256) as u8 }

impl Default for QuantumKeyDistribution {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_qkd_creation() {
        let qkd = QuantumKeyDistribution::new();
        assert_eq!(qkd.qkd_id, "quantum_key_distribution_v1");
    }
}
