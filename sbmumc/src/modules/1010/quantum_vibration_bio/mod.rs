//! # SBMUMC Module 1010: Quantum Vibration in Biology
//! 
//! Quantum vibrational effects in biological molecules.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VibrationalMode {
    Stretching,
    Bending,
    Torsional,
    Breathing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumVibrationalState {
    pub state_id: String,
    pub molecule: String,
    pub mode: VibrationalMode,
    pub frequency_cm1: f64,
    pub quantum_number: u32,
    pub thermal_population: f64,
    pub zero_point_energy_meV: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationalCouplingAnalysis {
    pub analysis_id: String,
    pub system: String,
    pub coupled_vibrational_modes: u32,
    pub coupling_strength_meV: f64,
    pub anharmonicity: f64,
    pub vibrational_energy_transfer_rate: f64,
}

impl QuantumVibrationalState {
    pub fn new(molecule: &str, mode: VibrationalMode, frequency: f64) -> Self {
        Self {
            state_id: format!("qvs_{}", uuid_simple()),
            molecule: molecule.to_string(),
            mode,
            frequency_cm1: frequency,
            quantum_number: 0,
            thermal_population: 0.0,
            zero_point_energy_meV: 0.0,
        }
    }

    pub fn set_state(&mut self, n: u32, thermal: f64) {
        self.quantum_number = n;
        self.thermal_population = thermal;
        self.zero_point_energy_meV = self.frequency_cm1 * 0.124;
    }

    pub fn vibrational_energy(&self) -> f64 {
        (self.quantum_number as f64 + 0.5) * self.frequency_cm1 * 0.124
    }
}

impl VibrationalCouplingAnalysis {
    pub fn new(system: &str) -> Self {
        Self {
            analysis_id: format!("vca_{}", uuid_simple()),
            system: system.to_string(),
            coupled_vibrational_modes: 0,
            coupling_strength_meV: 0.0,
            anharmonicity: 0.0,
            vibrational_energy_transfer_rate: 0.0,
        }
    }

    pub fn configure(&mut self, modes: u32, coupling: f64, anharmonic: f64, rate: f64) {
        self.coupled_vibrational_modes = modes;
        self.coupling_strength_meV = coupling;
        self.anharmonicity = anharmonic;
        self.vibrational_energy_transfer_rate = rate;
    }

    pub fn vibrational_coherence_quality(&self) -> f64 {
        let mode_factor = (self.coupled_vibrational_modes as f64 / 10.0).min(1.0);
        mode_factor * (1.0 - self.anharmonicity) * self.coupling_strength_meV / 100.0
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
    fn test_quantum_vibrational_state() {
        let mut state = QuantumVibrationalState::new(
            "Amide I",
            VibrationalMode::Stretching,
            1600.0,
        );
        state.set_state(1, 0.3);
        assert!(state.vibrational_energy() > 0.0);
    }

    #[test]
    fn test_vibrational_coupling_analysis() {
        let mut analysis = VibrationalCouplingAnalysis::new("Protein backbone");
        analysis.configure(5, 20.0, 0.1, 1000.0);
        assert!(analysis.vibrational_coherence_quality() > 0.0);
    }
}
