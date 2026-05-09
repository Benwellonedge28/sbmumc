//! # SBMUMC Module 1011: Quantum Resonance in Biology
//! 
//! Quantum resonance phenomena in biological systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResonanceType {
    Vibrational,
    Magnetic,
    Electronic,
    Nuclear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumResonanceState {
    pub state_id: String,
    pub resonance_type: ResonanceType,
    pub system: String,
    pub resonance_frequency_hz: f64,
    pub quality_factor: f64,
    pub resonance_width_hz: f64,
    pub coupling_strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceDynamics {
    pub dynamics_id: String,
    pub biological_system: String,
    pub lifetime_ns: f64,
    pub decoherence_time_ns: f64,
    pub coherence_quality: f64,
    pub resonance_enhancement: f64,
}

impl QuantumResonanceState {
    pub fn new(rtype: ResonanceType, system: &str, freq: f64) -> Self {
        Self {
            state_id: format!("qrs_{}", uuid_simple()),
            resonance_type: rtype,
            system: system.to_string(),
            resonance_frequency_hz: freq,
            quality_factor: 0.0,
            resonance_width_hz: 0.0,
            coupling_strength: 0.0,
        }
    }

    pub fn configure(&mut self, q: f64, width: f64, coupling: f64) {
        self.quality_factor = q;
        self.resonance_width_hz = width;
        self.coupling_strength = coupling;
    }

    pub fn resonance_sharpness(&self) -> f64 {
        self.resonance_frequency_hz / self.resonance_width_hz.max(1.0)
    }
}

impl ResonanceDynamics {
    pub fn new(system: &str) -> Self {
        Self {
            dynamics_id: format!("rd_{}", uuid_simple()),
            biological_system: system.to_string(),
            lifetime_ns: 0.0,
            decoherence_time_ns: 0.0,
            coherence_quality: 0.0,
            resonance_enhancement: 0.0,
        }
    }

    pub fn configure(&mut self, lifetime: f64, decoherence: f64) {
        self.lifetime_ns = lifetime;
        self.decoherence_time_ns = decoherence;
        self.coherence_quality = lifetime / decoherence.max(1.0);
        self.resonance_enhancement = self.coherence_quality * 10.0;
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
    fn test_quantum_resonance_state() {
        let mut state = QuantumResonanceState::new(
            ResonanceType::Magnetic,
            "Cryptochrome",
            1.42e9,
        );
        state.configure(1000.0, 1.42e6, 0.5);
        assert!(state.resonance_sharpness() > 0.0);
    }

    #[test]
    fn test_resonance_dynamics() {
        let mut dynamics = ResonanceDynamics::new("Bird compass");
        dynamics.configure(100.0, 50.0);
        assert!(dynamics.resonance_enhancement > 0.0);
    }
}
