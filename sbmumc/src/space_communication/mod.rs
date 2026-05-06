//! Space Communication Module (666)
//!
//! Deep space communications, networking protocols, and signal processing.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationBand {
    S,
    C,
    X,
    Ku,
    Ka,
    Q,
    V,
    Optical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceCommunication {
    pub comm_system_name: String,
    pub frequency_band: CommunicationBand,
    pub transmit_power: f64,          // W
    pub antenna_diameter: f64,         // m
    pub data_rate: f64,               // Mbps
    pub distance: f64,                 // AU
    pub signal_strength: f64,         // dBm
    pub noise_temperature: f64,        // K
    pub error_correction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSpaceNetwork {
    pub network_name: String,
    pub ground_stations: Vec<String>,
    pub tracking_capability: Vec<String>,
    pub data_throughput: f64,         // TB/day
    pub uptime: f64,                   // percent
    pub signal_delay: f64,             // minutes (one-way)
}

impl SpaceCommunication {
    pub fn new(comm_system_name: String, frequency_band: CommunicationBand) -> Self {
        Self {
            comm_system_name,
            frequency_band,
            transmit_power: 0.0,
            antenna_diameter: 0.0,
            data_rate: 0.0,
            distance: 0.0,
            signal_strength: 0.0,
            noise_temperature: 0.0,
            error_correction: "LDPC".into(),
        }
    }

    pub fn calculate_free_space_loss(&self) -> f64 {
        let c = 299792458.0;
        let freq = match self.frequency_band {
            CommunicationBand::S => 2.0e9,
            CommunicationBand::X => 8.0e9,
            CommunicationBand::Ka => 32.0e9,
            _ => 10.0e9,
        };
        let wavelength = c / freq;
        let d = self.distance * 1.496e11; // AU to meters
        20.0 * (wavelength.powi(2) / (16.0 * std::f64::consts::PI * d).powi(2)).log10()
    }

    pub fn link_budget(&self) -> f64 {
        let transmit_power_db = 10.0 * (self.transmit_power / 1.0).log10();
        let antenna_gain_db = 10.0 * (self.antenna_diameter * 15.0).log10();
        let path_loss = self.calculate_free_space_loss();
        transmit_power_db + antenna_gain_db * 2.0 - path_loss
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_communication() {
        let comm = SpaceCommunication::new("Mars Relay".into(), CommunicationBand::Ka);
        assert!(matches!(comm.frequency_band, CommunicationBand::Ka));
    }
}
