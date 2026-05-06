//! Interstellar Probe Module (668)
//!
//! Interstellar mission design and probe engineering for interstellar exploration.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterstellarProbe {
    pub probe_name: String,
    pub target_system: String,
    pub distance_ly: f64,
    pub velocity: f64,               // c (fraction of light speed)
    pub mission_lifetime: f64,      // years
    pub communication_delay: f64,    // years (one-way)
    pub power_system: String,
    pub propulsion_type: String,
    pub scientific_instruments: Vec<String>,
    pub data_storage: f64,           // TB
    pub transmission_rate: f64,     // bps
}

impl InterstellarProbe {
    pub fn new(probe_name: String, target_system: String) -> Self {
        Self {
            probe_name,
            target_system,
            distance_ly: 0.0,
            velocity: 0.0,
            mission_lifetime: 0.0,
            communication_delay: 0.0,
            power_system: "Radioisotope".into(),
            propulsion_type: "Fusion".into(),
            scientific_instruments: Vec::new(),
            data_storage: 0.0,
            transmission_rate: 0.0,
        }
    }

    pub fn travel_time(&self) -> f64 {
        self.distance_ly / self.velocity
    }

    pub fn arrival_date(&self, launch_year: f64) -> f64 {
        launch_year + self.travel_time()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interstellar_probe() {
        let probe = InterstellarProbe::new("Breakthrough Starshot".into(), "Proxima Centauri".into());
        assert_eq!(probe.probe_name, "Breakthrough Starshot");
    }
}
