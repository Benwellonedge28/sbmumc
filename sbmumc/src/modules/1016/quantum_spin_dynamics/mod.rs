//! # SBMUMC Module 1016: Quantum Spin Dynamics
//! 
//! Quantum spin dynamics in biological systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpinState {
    Singlet,
    Triplet,
    Doublet,
    Quartet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSpinDynamics {
    pub dynamics_id: String,
    pub initial_state: SpinState,
    pub final_state: SpinState,
    pub system: String,
    pub spin_lattice_relaxation_ms: f64,
    pub spin_spin_relaxation_ms: f64,
    pub spin_coherence_time_ns: f64,
    pub intersystem_crossing_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadicalPairDynamics {
    pub dynamics_id: String,
    pub radical_pair: String,
    pub singlet_yield: f64,
    pub triplet_yield: f64,
    pub spin_conversion_efficiency: f64,
    pub magnetic_field_effect: f64,
    pub navigation_capability: f64,
}

impl QuantumSpinDynamics {
    pub fn new(initial: SpinState, final_state: SpinState, system: &str) -> Self {
        Self {
            dynamics_id: format!("qsd_{}", uuid_simple()),
            initial_state: initial,
            final_state: final_state,
            system: system.to_string(),
            spin_lattice_relaxation_ms: 0.0,
            spin_spin_relaxation_ms: 0.0,
            spin_coherence_time_ns: 0.0,
            intersystem_crossing_rate: 0.0,
        }
    }

    pub fn configure(&mut self, t1: f64, t2: f64, coherence: f64, isc: f64) {
        self.spin_lattice_relaxation_ms = t1;
        self.spin_spin_relaxation_ms = t2;
        self.spin_coherence_time_ns = coherence;
        self.intersystem_crossing_rate = isc;
    }

    pub fn spin_coherence_quality(&self) -> f64 {
        self.spin_coherence_time_ns / (self.spin_lattice_relaxation_ms * 1000.0).max(1.0)
    }
}

impl RadicalPairDynamics {
    pub fn new(radical_pair: &str) -> Self {
        Self {
            dynamics_id: format!("rpd_{}", uuid_simple()),
            radical_pair: radical_pair.to_string(),
            singlet_yield: 0.0,
            triplet_yield: 0.0,
            spin_conversion_efficiency: 0.0,
            magnetic_field_effect: 0.0,
            navigation_capability: 0.0,
        }
    }

    pub fn configure(&mut self, singlet: f64, triplet: f64, mfe: f64) {
        self.singlet_yield = singlet;
        self.triplet_yield = triplet;
        self.spin_conversion_efficiency = (singlet + triplet) / 2.0;
        self.magnetic_field_effect = mfe;
        self.navigation_capability = self.spin_conversion_efficiency * mfe;
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
    fn test_quantum_spin_dynamics() {
        let mut dynamics = QuantumSpinDynamics::new(
            SpinState::Singlet,
            SpinState::Triplet,
            "Photosynthetic RC",
        );
        dynamics.configure(100.0, 50.0, 100.0, 1e6);
        assert!(dynamics.spin_coherence_quality() > 0.0);
    }

    #[test]
    fn test_radical_pair_dynamics() {
        let mut dynamics = RadicalPairDynamics::new("Cryptochrome Radical Pair");
        dynamics.configure(0.4, 0.6, 0.3);
        assert!(dynamics.navigation_capability > 0.0);
    }
}
