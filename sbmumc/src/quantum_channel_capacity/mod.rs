//! Quantum Channel Capacity Module (563)
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumChannelCapacity {
    pub qcc_id: String,
    pub channel_type: ChannelType,
    pub capacity_bits_per_use: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    Depolarizing,
    AmplitudeDamping,
    PhaseDamping,
    Erasure,
}

impl QuantumChannelCapacity {
    pub fn new() -> Self {
        Self {
            qcc_id: String::from("quantum_channel_capacity_v1"),
            channel_type: ChannelType::Depolarizing,
            capacity_bits_per_use: 0.5,
        }
    }
}

impl Default for QuantumChannelCapacity {
    fn default() -> Self { Self::new() }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_capacity() {
        let c = QuantumChannelCapacity::new();
        assert!(c.capacity_bits_per_use > 0.0);
    }
}
