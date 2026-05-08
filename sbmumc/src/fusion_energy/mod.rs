//! Fusion Energy Module (743)
//!
//! Nuclear fusion power, plasma confinement, and fusion reactor design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionReactor {
    pub reactor_id: String,
    pub confinement_type: String,
    pub plasma_volume_m3: f64,
    pub temperature_kev: f64,
    pub confinement_time_s: f64,
    pub fusion_gain_q: f64,
    pub fusion_power_mw: f64,
    pub construction_status: String,
}

impl FusionReactor {
    pub fn new(reactor_id: String) -> Self {
        Self {
            reactor_id,
            confinement_type: "Tokamak".into(),
            plasma_volume_m3: 0.0,
            temperature_kev: 0.0,
            confinement_time_s: 0.0,
            fusion_gain_q: 0.0,
            fusion_power_mw: 0.0,
            construction_status: "Planned".into(),
        }
    }

    pub fn calculate_triple_product(&self) -> f64 {
        self.temperature_kev * self.confinement_time_s * self.plasma_volume_m3
    }

    pub fn is_ignition_ready(&self) -> bool {
        self.fusion_gain_q > 10.0 && self.temperature_kev > 10000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fusion_reactor() {
        let reactor = FusionReactor::new("FUS-001".into());
        assert_eq!(reactor.reactor_id, "FUS-001");
    }
}
