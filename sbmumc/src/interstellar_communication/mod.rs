//! Interstellar Communication Module (688)
//!
//! Communication systems for interstellar distances, including lasercom and gravitational wave signaling.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterstellarCommMethod {
    Lasercom,
    Radio,
    Neutrino,
    GravitationalWave,
    QuantumEntanglement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterstellarCommunication {
    pub comm_system_name: String,
    pub communication_method: InterstellarCommMethod,
    pub target_distance_ly: f64,
    pub data_rate_bps: f64,
    pub transmission_power: f64,       // W
    pub wavelength: f64,               // nm
    pub received_power: f64,           // W
    pub bit_error_rate: f64,
    pub message_delay_years: f64,
    pub antenna_diameter: f64,        // m
}

impl InterstellarCommunication {
    pub fn new(comm_system_name: String, method: InterstellarCommMethod) -> Self {
        Self {
            comm_system_name,
            communication_method: method,
            target_distance_ly: 0.0,
            data_rate_bps: 0.0,
            transmission_power: 0.0,
            wavelength: 1064.0,
            received_power: 0.0,
            bit_error_rate: 0.0,
            message_delay_years: 0.0,
            antenna_diameter: 0.0,
        }
    }

    pub fn calculate_signal_delay(&self) -> f64 {
        self.target_distance_ly
    }

    pub fn link_budget(&self) -> f64 {
        let pl = 20.0 * (self.target_distance_ly * 9.46e15 * self.wavelength * 1e-9).log10();
        10.0 * (self.transmission_power / 1e-3).log10() - pl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interstellar_comm() {
        let comm = InterstellarCommunication::new("Laser Beacon".into(), InterstellarCommMethod::Lasercom);
        assert_eq!(comm.comm_system_name, "Laser Beacon");
    }
}
