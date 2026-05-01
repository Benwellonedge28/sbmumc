//! Time Measurement Module
use crate::core::{SbmumcError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeMeasurement {
    pub tm_id: String,
    pub measurement_standards: Vec<TimeStandard>,
    pub precision_clocks: Vec<PrecisionClock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeStandard {
    pub standard_type: StandardType,
    pub definition: String,
    pub accuracy_pp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StandardType {
    Atomic,
    Celestial,
    Gravitational,
    Quantum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionClock {
    pub clock_id: String,
    pub clock_type: StandardType,
    pub stability_factor: f64,
}

impl TimeMeasurement {
    pub fn new() -> Self {
        Self {
            tm_id: String::from("time_measurement_v1"),
            measurement_standards: vec![],
            precision_clocks: vec![],
        }
    }
}

impl Default for TimeMeasurement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_measurement_creation() {
        let measurement = TimeMeasurement::new();
        assert_eq!(measurement.tm_id, "time_measurement_v1");
    }
}
