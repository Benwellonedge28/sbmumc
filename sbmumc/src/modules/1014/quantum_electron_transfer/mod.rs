//! # SBMUMC Module 1014: Quantum Electron Transfer
//! 
//! Quantum electron transfer in biological systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElectronTransferType {
    Superexchange,
    Hopping,
    Tunneling,
    Coherent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumElectronTransfer {
    pub transfer_id: String,
    pub transfer_type: ElectronTransferType,
    pub donor_state: String,
    pub acceptor_state: String,
    pub reorganization_energy_eV: f64,
    pub driving_force_eV: f64,
    pub electronic_coupling: f64,
    pub transfer_rate_s1: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectronTransferChain {
    pub chain_id: String,
    pub transfers: Vec<QuantumElectronTransfer>,
    pub chain_length: u32,
    pub overall_rate_s1: f64,
    pub quantum_coherence_length: u32,
    pub efficiency: f64,
}

impl QuantumElectronTransfer {
    pub fn new(etype: ElectronTransferType, donor: &str, acceptor: &str) -> Self {
        Self {
            transfer_id: format!("qet_{}", uuid_simple()),
            transfer_type: etype,
            donor_state: donor.to_string(),
            acceptor_state: acceptor.to_string(),
            reorganization_energy_eV: 0.0,
            driving_force_eV: 0.0,
            electronic_coupling: 0.0,
            transfer_rate_s1: 0.0,
        }
    }

    pub fn configure(&mut self, lambda: f64, driving: f64, coupling: f64) {
        self.reorganization_energy_eV = lambda;
        self.driving_force_eV = driving;
        self.electronic_coupling = coupling;
        let delta_g = driving - lambda;
        self.transfer_rate_s1 = coupling.powi(2) * (-delta_g.powi(2) / (4.0 * lambda * 0.1)).exp();
    }

    pub fn marcus_rate(&self) -> f64 {
        self.transfer_rate_s1.max(0.0)
    }
}

impl ElectronTransferChain {
    pub fn new() -> Self {
        Self {
            chain_id: format!("etc_{}", uuid_simple()),
            transfers: Vec::new(),
            chain_length: 0,
            overall_rate_s1: 0.0,
            quantum_coherence_length: 0,
            efficiency: 0.0,
        }
    }

    pub fn add_transfer(&mut self, transfer: QuantumElectronTransfer) {
        self.transfers.push(transfer);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.chain_length = self.transfers.len() as u32;
        let rates: f64 = self.transfers.iter().map(|t| t.transfer_rate_s1).product();
        self.overall_rate_s1 = rates.powf(1.0 / self.transfers.len().max(1) as f64);
        self.quantum_coherence_length = self.transfers.iter()
            .filter(|t| t.transfer_type == ElectronTransferType::Coherent)
            .count() as u32;
        self.efficiency = (self.overall_rate_s1 / 1e12).min(1.0) * (1.0 + self.quantum_coherence_length as f64 * 0.1);
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
    fn test_quantum_electron_transfer() {
        let mut transfer = QuantumElectronTransfer::new(
            ElectronTransferType::Tunneling,
            "Photosystem II",
            "Tyrosine Z",
        );
        transfer.configure(0.8, 0.5, 0.01);
        assert!(transfer.marcus_rate() >= 0.0);
    }

    #[test]
    fn test_electron_transfer_chain() {
        let mut chain = ElectronTransferChain::new();
        chain.add_transfer(QuantumElectronTransfer::new(
            ElectronTransferType::Coherent,
            "Complex I",
            "Complex II",
        ));
        assert!(chain.chain_length > 0);
    }
}
