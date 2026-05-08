//! Plasma Physics Module (751)
//!
//! Plasma behavior, instabilities, and transport in fusion devices.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlasmaInstability {
    TAE,
    RTM,
    ELM,
    LMP,
    NTMSawtooth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlasmaState {
    pub state_id: String,
    pub temperature_kev: f64,
    pub density_m3: f64,
    pub plasma_current_ma: f64,
    pub confinement_time_s: f64,
    pub instabilities: Vec<PlasmaInstability>,
    pub stability_score: f64,
}

impl PlasmaState {
    pub fn new(state_id: String) -> Self {
        Self {
            state_id,
            temperature_kev: 0.0,
            density_m3: 0.0,
            plasma_current_ma: 0.0,
            confinement_time_s: 0.0,
            instabilities: Vec::new(),
            stability_score: 0.0,
        }
    }

    pub fn beta_percent(&self) -> f64 {
        let p_psi = 2.0 * self.density_m3 * self.temperature_kev * 1.16e16;
        (p_psi / (self.plasma_current_ma / (std::f64::consts::PI * 0.8))).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plasma() {
        let plasma = PlasmaState::new("PL-001".into());
        assert_eq!(plasma.state_id, "PL-001");
    }
}
