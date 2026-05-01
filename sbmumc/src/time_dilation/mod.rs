//! Time Dilation Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeDilation {
    pub td_id: String,
    pub relativistic_effects: Vec<RelativisticEffect>,
    pub gravitational_dilation: GravitationalDilation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelativisticEffect {
    SpecialRelativity { velocity: f64 },
    GeneralRelativity { gravitational_potential: f64 },
    KinematicDilation { gamma: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravitationalDilation {
    pub mass: f64,
    pub radius: f64,
    pub time_slowdown_factor: f64,
}

impl TimeDilation {
    pub fn new() -> Self {
        Self {
            td_id: String::from("time_dilation_v1"),
            relativistic_effects: vec![],
            gravitational_dilation: GravitationalDilation {
                mass: 0.0,
                radius: 0.0,
                time_slowdown_factor: 1.0,
            },
        }
    }

    pub fn calculate_time_factor(&self, velocity: f64) -> f64 {
        let c = 299792458.0;
        let beta = velocity / c;
        let gamma = 1.0 / (1.0 - beta * beta).sqrt();
        1.0 / gamma
    }
}

impl Default for TimeDilation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_dilation_calculation() {
        let dilation = TimeDilation::new();
        let factor = dilation.calculate_time_factor(0.5);
        assert!(factor > 0.0 && factor < 1.0);
    }
}
