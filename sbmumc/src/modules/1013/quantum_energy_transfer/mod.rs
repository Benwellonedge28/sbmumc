//! # SBMUMC Module 1013: Quantum Energy Transfer
//! 
//! Quantum energy transfer mechanisms in biological systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnergyTransferMechanism {
    ForsterResonance,
    Dexter,
    Coherent,
    Incoherent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumEnergyTransfer {
    pub transfer_id: String,
    pub mechanism: EnergyTransferMechanism,
    pub donor: String,
    pub acceptor: String,
    pub transfer_rate_s1: f64,
    pub quantum_yield: f64,
    pub distance_nm: f64,
    pub directionality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTransferNetwork {
    pub network_id: String,
    pub transfers: Vec<QuantumEnergyTransfer>,
    pub total_efficiency: f64,
    pub coherence_regions: u32,
    pub network_robustness: f64,
}

impl QuantumEnergyTransfer {
    pub fn new(mechanism: EnergyTransferMechanism, donor: &str, acceptor: &str) -> Self {
        Self {
            transfer_id: format!("qet_{}", uuid_simple()),
            mechanism,
            donor: donor.to_string(),
            acceptor: acceptor.to_string(),
            transfer_rate_s1: 0.0,
            quantum_yield: 0.0,
            distance_nm: 0.0,
            directionality: 0.0,
        }
    }

    pub fn configure(&mut self, rate: f64, yield_: f64, distance: f64) {
        self.transfer_rate_s1 = rate;
        self.quantum_yield = yield_;
        self.distance_nm = distance;
        self.directionality = yield_ * (1.0 / (1.0 + distance * 0.1));
    }

    pub fn transfer_efficiency(&self) -> f64 {
        self.quantum_yield * self.directionality
    }
}

impl EnergyTransferNetwork {
    pub fn new() -> Self {
        Self {
            network_id: format!("etn_{}", uuid_simple()),
            transfers: Vec::new(),
            total_efficiency: 0.0,
            coherence_regions: 0,
            network_robustness: 0.0,
        }
    }

    pub fn add_transfer(&mut self, transfer: QuantumEnergyTransfer) {
        self.transfers.push(transfer);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_efficiency = self.transfers.iter()
            .map(|t| t.transfer_efficiency())
            .sum::<f64>() / self.transfers.len().max(1) as f64;
        self.coherence_regions = self.transfers.iter()
            .filter(|t| t.mechanism == EnergyTransferMechanism::Coherent)
            .count() as u32;
        self.network_robustness = self.total_efficiency * (1.0 + self.coherence_regions as f64 * 0.1);
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_energy_transfer() {
        let mut transfer = QuantumEnergyTransfer::new(
            EnergyTransferMechanism::ForsterResonance,
            "Bacteriochlorophyll",
            "Reaction Center",
        );
        transfer.configure(1e12, 0.95, 2.0);
        assert!(transfer.transfer_efficiency() > 0.0);
    }

    #[test]
    fn test_energy_transfer_network() {
        let mut network = EnergyTransferNetwork::new();
        network.add_transfer(QuantumEnergyTransfer::new(
            EnergyTransferMechanism::Coherent,
            "FMO Complex A",
            "FMO Complex B",
        ));
        assert!(network.total_efficiency >= 0.0);
    }
}
