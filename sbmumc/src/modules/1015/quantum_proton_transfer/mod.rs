//! # SBMUMC Module 1015: Quantum Proton Transfer
//! 
//! Quantum proton transfer in biological systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtonTransferMechanism {
    Grotthuss,
    Tunneling,
    Vehicle,
    Structural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumProtonTransfer {
    pub transfer_id: String,
    pub mechanism: ProtonTransferMechanism,
    pub donor: String,
    pub acceptor: String,
    pub barrier_height_kj_mol: f64,
    pub tunneling_contribution: f64,
    pub transfer_rate_s1: f64,
    pub kinetic_isotope_effect: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtonWireState {
    pub state_id: String,
    pub wire_name: String,
    pub length_A: f64,
    pub proton_affinity_gradient: f64,
    pub quantum_conduction: f64,
    pub proton_current_a: f64,
}

impl QuantumProtonTransfer {
    pub fn new(mechanism: ProtonTransferMechanism, donor: &str, acceptor: &str) -> Self {
        Self {
            transfer_id: format!("qpt_{}", uuid_simple()),
            mechanism,
            donor: donor.to_string(),
            acceptor: acceptor.to_string(),
            barrier_height_kj_mol: 0.0,
            tunneling_contribution: 0.0,
            transfer_rate_s1: 0.0,
            kinetic_isotope_effect: 0.0,
        }
    }

    pub fn configure(&mut self, barrier: f64, tunneling: f64, rate: f64, kie: f64) {
        self.barrier_height_kj_mol = barrier;
        self.tunneling_contribution = tunneling;
        self.transfer_rate_s1 = rate;
        self.kinetic_isotope_effect = kie;
    }

    pub fn quantum_contribution(&self) -> f64 {
        self.tunneling_contribution * (1.0 - 1.0 / self.kinetic_isotope_effect)
    }
}

impl ProtonWireState {
    pub fn new(name: &str, length: f64) -> Self {
        Self {
            state_id: format!("pws_{}", uuid_simple()),
            wire_name: name.to_string(),
            length_A: length,
            proton_affinity_gradient: 0.0,
            quantum_conduction: 0.0,
            proton_current_a: 0.0,
        }
    }

    pub fn configure(&mut self, gradient: f64, conduction: f64, current: f64) {
        self.proton_affinity_gradient = gradient;
        self.quantum_conduction = conduction;
        self.proton_current_a = current;
    }

    pub fn wire_efficiency(&self) -> f64 {
        self.quantum_conduction * self.proton_affinity_gradient / self.length_A
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
    fn test_quantum_proton_transfer() {
        let mut transfer = QuantumProtonTransfer::new(
            ProtonTransferMechanism::Tunneling,
            "Serine Protease His",
            "Aspartate",
        );
        transfer.configure(40.0, 0.3, 1e6, 5.0);
        assert!(transfer.quantum_contribution() > 0.0);
    }

    #[test]
    fn test_proton_wire_state() {
        let mut wire = ProtonWireState::new("Aquaporin Channel", 25.0);
        wire.configure(0.8, 0.6, 1e-9);
        assert!(wire.wire_efficiency() > 0.0);
    }
}
