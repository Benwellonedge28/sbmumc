//! Chronobiology Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chronobiology {
    pub cb_id: String,
    pub circadian_rhythms: Vec<CircadianRhythm>,
    pub biological_clocks: Vec<BiologicalClock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircadianRhythm {
    pub rhythm_id: String,
    pub period_hours: f64,
    pub amplitude: f64,
    pub phase_shift: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicalClock {
    pub clock_type: ClockType,
    pub molecular_mechanism: String,
    pub entrainment_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClockType {
    Suprachiasmatic,
    Peripheral,
    Cellular,
    Ultradian,
}

impl Chronobiology {
    pub fn new() -> Self {
        Self {
            cb_id: String::from("chronobiology_v1"),
            circadian_rhythms: vec![],
            biological_clocks: vec![],
        }
    }
}

impl Default for Chronobiology {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chronobiology_creation() {
        let chrono = Chronobiology::new();
        assert_eq!(chrono.cb_id, "chronobiology_v1");
    }
}
