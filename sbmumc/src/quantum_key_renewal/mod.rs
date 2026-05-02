//! Quantum Key Renewal Module (571)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumKeyRenewal {
    pub qkr_id: String,
    pub renewal_rate_kbs: f64,
    pub key_lifespan_s: u32,
}

impl QuantumKeyRenewal {
    pub fn new() -> Self {
        Self {
            qkr_id: String::from("quantum_key_renewal_v1"),
            renewal_rate_kbs: 1000.0,
            key_lifespan_s: 3600,
        }
    }
}

impl Default for QuantumKeyRenewal {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_key_renewal() {
        let kr = QuantumKeyRenewal::new();
        assert!(kr.renewal_rate_kbs > 0.0);
    }
}
