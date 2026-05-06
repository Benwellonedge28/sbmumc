//! Life Support Systems Module (654)
//!
//! Advanced life support system design, regeneration, and recycling technologies.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LifeSupportType {
    OpenLoop,
    ClosedLoop,
    Hybrid,
    BioRegenerative,
    FullyClosed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeSupportSystem {
    pub system_type: LifeSupportType,
    pub crew_size: u32,
    pub mission_duration: f64,       // days
    pub oxygen_generation_rate: f64, // kg/day
    pub co2_removal_rate: f64,       // kg/day
    pub water_reclamation_rate: f64, // percent
    pub waste_processing_rate: f64,  // kg/day
    pub power_consumption: f64,      // kW
    pub mass: f64,                   // kg
    pub volume: f64,                  // m^3
    pub reliability: f64,           // MTBF hours
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereControl {
    pub oxygen_fraction: f64,      // percent
    pub nitrogen_fraction: f64,     // percent
    pub co2_fraction: f64,          // percent
    pub pressure: f64,              // kPa
    pub humidity: f64,              // percent
    pub temperature_setpoint: f64,  // C
    pub trace_contaminants: Vec<String>,
}

impl LifeSupportSystem {
    pub fn new(system_type: LifeSupportType, crew_size: u32) -> Self {
        Self {
            system_type,
            crew_size,
            mission_duration: 0.0,
            oxygen_generation_rate: 0.0,
            co2_removal_rate: 0.0,
            water_reclamation_rate: 0.0,
            waste_processing_rate: 0.0,
            power_consumption: 0.0,
            mass: 0.0,
            volume: 0.0,
            reliability: 0.0,
        }
    }

    pub fn calculate_oxygen_need(&self) -> f64 {
        0.84 * self.crew_size as f64 * self.mission_duration
    }

    pub fn calculate_water_need(&self) -> f64 {
        3.5 * self.crew_size as f64 * self.mission_duration
    }

    pub fn calculate_food_need(&self) -> f64 {
        1.8 * self.crew_size as f64 * self.mission_duration
    }

    pub fn total_consumables(&self) -> f64 {
        self.calculate_oxygen_need() + self.calculate_water_need() + self.calculate_food_need()
    }
}

impl AtmosphereControl {
    pub fn new() -> Self {
        Self {
            oxygen_fraction: 21.0,
            nitrogen_fraction: 78.0,
            co2_fraction: 0.04,
            pressure: 101.3,
            humidity: 50.0,
            temperature_setpoint: 22.0,
            trace_contaminants: Vec::new(),
        }
    }

    pub fn total_pressure(&self) -> f64 {
        self.pressure
    }

    pub fn validate_atmosphere(&self) -> bool {
        let sum = self.oxygen_fraction + self.nitrogen_fraction + self.co2_fraction;
        (sum - 100.0).abs() < 0.1 && self.pressure > 90.0 && self.pressure < 110.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_life_support() {
        let system = LifeSupportSystem::new(LifeSupportType::ClosedLoop, 6);
        assert!(matches!(system.system_type, LifeSupportType::ClosedLoop));
    }
}
