//! # SBMUMC Module 1020: Quantum Non-Equilibrium in Biology
//! 
//! Quantum non-equilibrium dynamics in biological systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NonequilibriumProcess {
    ActiveTransport,
    SignalTransduction,
    EnzymeCatalysis,
    Photosynthesis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumNonequilibriumState {
    pub state_id: String,
    pub process: NonequilibriumProcess,
    pub system: String,
    pub entropy_production_rate: f64,
    pub dissipation_rate: f64,
    pub coherence_sustained: bool,
    pub steady_state_approach_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumFluctuationTheorem {
    pub theorem_id: String,
    pub system: String,
    pub entropy_production: f64,
    pub time_reversal_asymmetry: f64,
    pub fluctuation_probability_ratio: f64,
    pub second_law_violation_probability: f64,
}

impl QuantumNonequilibriumState {
    pub fn new(process: NonequilibriumProcess, system: &str) -> Self {
        Self {
            state_id: format!("qns_{}", uuid_simple()),
            process,
            system: system.to_string(),
            entropy_production_rate: 0.0,
            dissipation_rate: 0.0,
            coherence_sustained: false,
            steady_state_approach_time: 0.0,
        }
    }

    pub fn configure(&mut self, entropy: f64, dissipation: f64, sustained: bool, time: f64) {
        self.entropy_production_rate = entropy;
        self.dissipation_rate = dissipation;
        self.coherence_sustained = sustained;
        self.steady_state_approach_time = time;
    }

    pub fn efficiency_at_cost(&self) -> f64 {
        if self.entropy_production_rate == 0.0 { 0.0 }
        else { 1.0 / (1.0 + self.dissipation_rate / self.entropy_production_rate) }
    }
}

impl QuantumFluctuationTheorem {
    pub fn new(system: &str) -> Self {
        Self {
            theorem_id: format!("qft_{}", uuid_simple()),
            system: system.to_string(),
            entropy_production: 0.0,
            time_reversal_asymmetry: 0.0,
            fluctuation_probability_ratio: 0.0,
            second_law_violation_probability: 0.0,
        }
    }

    pub fn configure(&mut self, entropy: f64, asymmetry: f64) {
        self.entropy_production = entropy;
        self.time_reversal_asymmetry = asymmetry;
        self.fluctuation_probability_ratio = (-asymmetry).exp();
        self.second_law_violation_probability = 1.0 / (1.0 + self.fluctuation_probability_ratio);
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
    fn test_quantum_nonequilibrium_state() {
        let mut state = QuantumNonequilibriumState::new(
            NonequilibriumProcess::ActiveTransport,
            "Molecular Motor",
        );
        state.configure(1e-21, 1e-22, true, 0.001);
        assert!(state.efficiency_at_cost() > 0.0);
    }

    #[test]
    fn test_quantum_fluctuation_theorem() {
        let mut theorem = QuantumFluctuationTheorem::new("Ion Channel");
        theorem.configure(5.0, 2.0);
        assert!(theorem.second_law_violation_probability < 0.15);
    }
}
