//! Quantum Cryptography Advanced Module (549)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumCryptographyAdvanced {
    pub qca_id: String,
    pub protocol: CryptoProtocol,
    pub security_level_bits: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CryptoProtocol {
    BB84,
    E91,
    SARG04,
    MeasurementDeviceIndependent,
    TwinFieldQKD,
}

impl QuantumCryptographyAdvanced {
    pub fn new() -> Self {
        Self {
            qca_id: String::from("quantum_cryptography_advanced_v1"),
            protocol: CryptoProtocol::TwinFieldQKD,
            security_level_bits: 256,
        }
    }
}

impl Default for QuantumCryptographyAdvanced {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_qcrypto() {
        let qc = QuantumCryptographyAdvanced::new();
        assert_eq!(qc.security_level_bits, 256);
    }
}
