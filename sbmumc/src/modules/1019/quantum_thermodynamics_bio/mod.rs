//! # SBMUMC Module 1019: Quantum Thermodynamics in Biology
//! 
//! Quantum thermodynamic principles in biological systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThermodynamicRegime {
    Classical,
    Quantum,
    Nonequilibrium,
    FarFromEquilibrium,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumThermodynamicState {
    pub state_id: String,
    pub regime: ThermodynamicRegime,
    pub system: String,
    pub temperature_k: f64,
    pub free_energy_j: f64,
    pub entropy_j_k: f64,
    pub quantum_contribution: f64,
    pub efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicalEnergyConverter {
    pub converter_id: String,
    pub conversion_type: String,
    pub max_efficiency: f64,
    pub actual_efficiency: f64,
    pub quantum_correction: f64,
    pub thermodynamic_force: f64,
}

impl QuantumThermodynamicState {
    pub fn new(regime: ThermodynamicRegime, system: &str) -> Self {
        Self {
            state_id: format!("qts_{}", uuid_simple()),
            regime,
            system: system.to_string(),
            temperature_k: 0.0,
            free_energy_j: 0.0,
            entropy_j_k: 0.0,
            quantum_contribution: 0.0,
            efficiency: 0.0,
        }
    }

    pub fn configure(&mut self, temp: f64, free_e: f64, entropy: f64, quantum: f64) {
        self.temperature_k = temp;
        self.free_energy_j = free_e;
        self.entropy_j_k = entropy;
        self.quantum_contribution = quantum;
        self.efficiency = free_e / (temp * entropy.max(0.001));
    }

    pub fn carnot_limitation(&self) -> f64 {
        let thermal_energy = 1.38e-23 * self.temperature_k;
        if thermal_energy == 0.0 { 0.0 }
        else { (self.free_energy_j / thermal_energy).max(0.0).min(1.0) }
    }
}

impl BiologicalEnergyConverter {
    pub fn new(conversion_type: &str) -> Self {
        Self {
            converter_id: format!("bec_{}", uuid_simple()),
            conversion_type: conversion_type.to_string(),
            max_efficiency: 0.0,
            actual_efficiency: 0.0,
            quantum_correction: 0.0,
            thermodynamic_force: 0.0,
        }
    }

    pub fn configure(&mut self, max_eff: f64, actual: f64, quantum: f64, force: f64) {
        self.max_efficiency = max_eff;
        self.actual_efficiency = actual;
        self.quantum_correction = quantum;
        self.thermodynamic_force = force;
    }

    pub fn thermodynamic_optimality(&self) -> f64 {
        self.actual_efficiency / self.max_efficiency.max(0.001) * (1.0 + self.quantum_correction)
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
    fn test_quantum_thermodynamic_state() {
        let mut state = QuantumThermodynamicState::new(
            ThermodynamicRegime::Quantum,
            "ATP Synthase",
        );
        state.configure(310.0, 1e-19, 1e-21, 0.1);
        assert!(state.carnot_limitation() >= 0.0);
    }

    #[test]
    fn test_biological_energy_converter() {
        let mut converter = BiologicalEnergyConverter::new("ATP Synthesis");
        converter.configure(0.95, 0.7, 0.15, 20.0);
        assert!(converter.thermodynamic_optimality() > 0.0);
    }
}
