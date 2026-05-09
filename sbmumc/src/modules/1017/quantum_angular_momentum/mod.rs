//! # SBMUMC Module 1017: Quantum Angular Momentum
//! 
//! Quantum angular momentum effects in biological systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AngularMomentumType {
    Orbital,
    Spin,
    Total,
    Rovibrational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumAngularMomentumState {
    pub state_id: String,
    pub momentum_type: AngularMomentumType,
    pub system: String,
    pub quantum_number: i32,
    pub magnetic_quantum_number: i32,
    pub degeneracy: u32,
    pub splitting_constant: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AngularMomentumCoupling {
    pub coupling_id: String,
    pub system: String,
    pub coupling_type: String,
    pub total_angular_momentum: f64,
    pub coupling_strength: f64,
    pub energy_splitting_cm1: f64,
}

impl QuantumAngularMomentumState {
    pub fn new(mtype: AngularMomentumType, system: &str, l: i32) -> Self {
        let degeneracy = (2 * l + 1).unsigned_abs();
        Self {
            state_id: format!("qam_{}", uuid_simple()),
            momentum_type: mtype,
            system: system.to_string(),
            quantum_number: l,
            magnetic_quantum_number: 0,
            degeneracy,
            splitting_constant: 0.0,
        }
    }

    pub fn set_magnetic_quantum(&mut self, m: i32) {
        self.magnetic_quantum_number = m;
    }

    pub fn configure(&mut self, splitting: f64) {
        self.splitting_constant = splitting;
    }

    pub fn lande_factor(&self) -> f64 {
        let l = self.quantum_number.abs() as f64;
        1.0 + (l * (l + 1.0) + 0.75 - 0.5) / (2.0 * l * (l + 1.0))
    }
}

impl AngularMomentumCoupling {
    pub fn new(system: &str, coupling_type: &str) -> Self {
        Self {
            coupling_id: format!("amc_{}", uuid_simple()),
            system: system.to_string(),
            coupling_type: coupling_type.to_string(),
            total_angular_momentum: 0.0,
            coupling_strength: 0.0,
            energy_splitting_cm1: 0.0,
        }
    }

    pub fn configure(&mut self, total_j: f64, strength: f64, splitting: f64) {
        self.total_angular_momentum = total_j;
        self.coupling_strength = strength;
        self.energy_splitting_cm1 = splitting;
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
    fn test_quantum_angular_momentum_state() {
        let mut state = QuantumAngularMomentumState::new(
            AngularMomentumType::Orbital,
            "Molecular Orbital",
            2,
        );
        state.set_magnetic_quantum(1);
        state.configure(1.5);
        assert!(state.lande_factor() > 0.0);
    }

    #[test]
    fn test_angular_momentum_coupling() {
        let mut coupling = AngularMomentumCoupling::new(
            "Spin-Orbit Coupling",
            "LS coupling",
        );
        coupling.configure(1.5, 0.8, 100.0);
        assert!(coupling.coupling_strength > 0.0);
    }
}
