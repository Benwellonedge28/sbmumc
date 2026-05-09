//! # SBMUMC Module 1018: Quantum Optics in Biology
//! 
//! Quantum optical phenomena in biological systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpticalQuantumEffect {
    SinglePhoton,
    EntangledPhotons,
    SqueezedLight,
    QuantumInterference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicalOpticsState {
    pub state_id: String,
    pub effect: OpticalQuantumEffect,
    pub system: String,
    pub photon_statistics: String,
    pub second_order_correlation: f64,
    pub quantum_efficiency: f64,
    pub bandwidth_nm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotonAbsorptionDynamics {
    pub dynamics_id: String,
    pub chromophore: String,
    pub absorption_cross_section: f64,
    pub quantum_yield: f64,
    pub excited_state_lifetime_ps: f64,
    pub decoherence_time_fs: f64,
}

impl BiologicalOpticsState {
    pub fn new(effect: OpticalQuantumEffect, system: &str) -> Self {
        Self {
            state_id: format!("bos_{}", uuid_simple()),
            effect,
            system: system.to_string(),
            photon_statistics: "thermal".to_string(),
            second_order_correlation: 0.0,
            quantum_efficiency: 0.0,
            bandwidth_nm: 0.0,
        }
    }

    pub fn configure(&mut self, g2: f64, efficiency: f64, bandwidth: f64) {
        self.second_order_correlation = g2;
        self.quantum_efficiency = efficiency;
        self.bandwidth_nm = bandwidth;
    }

    pub fn nonclassicality(&self) -> f64 {
        if self.second_order_correlation < 1.0 {
            1.0 - self.second_order_correlation
        } else {
            0.0
        }
    }
}

impl PhotonAbsorptionDynamics {
    pub fn new(chromophore: &str) -> Self {
        Self {
            dynamics_id: format!("pad_{}", uuid_simple()),
            chromophore: chromophore.to_string(),
            absorption_cross_section: 0.0,
            quantum_yield: 0.0,
            excited_state_lifetime_ps: 0.0,
            decoherence_time_fs: 0.0,
        }
    }

    pub fn configure(&mut self, cross_section: f64, yield_: f64, lifetime: f64, decoherence: f64) {
        self.absorption_cross_section = cross_section;
        self.quantum_yield = yield_;
        self.excited_state_lifetime_ps = lifetime;
        self.decoherence_time_fs = decoherence;
    }

    pub fn photon_detection_efficiency(&self) -> f64 {
        self.quantum_yield * (self.decoherence_time_fs / self.excited_state_lifetime_ps / 1000.0)
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
    fn test_biological_optics_state() {
        let mut state = BiologicalOpticsState::new(
            OpticalQuantumEffect::SinglePhoton,
            "Retinal",
        );
        state.configure(0.5, 0.8, 50.0);
        assert!(state.nonclassicality() > 0.0);
    }

    #[test]
    fn test_photon_absorption_dynamics() {
        let mut dynamics = PhotonAbsorptionDynamics::new("Rhodopsin");
        dynamics.configure(1e-16, 0.7, 200.0, 100.0);
        assert!(dynamics.photon_detection_efficiency() > 0.0);
    }
}
