//! Hubble Space Telescope Module (692)
//!
//! Space telescope observation planning and astronomical data processing.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubbleObservation {
    pub observation_id: String,
    pub target_name: String,
    pub instrument: String,
    pub exposure_time: f64,
    pub limiting_magnitude: f64,
}

impl HubbleObservation {
    pub fn new(observation_id: String, target: String) -> Self {
        Self {
            observation_id,
            target_name: target,
            instrument: "WFC3".into(),
            exposure_time: 0.0,
            limiting_magnitude: 31.0,
        }
    }

    pub fn calculate_depth(&self) -> f64 {
        31.0 - 2.5 * (self.exposure_time / 3600.0).log10()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hubble() {
        let obs = HubbleObservation::new("HST-001".into(), "M31".into());
        assert_eq!(obs.target_name, "M31");
    }
}
