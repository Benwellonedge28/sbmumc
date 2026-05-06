//! Space Suit Design Module (658)
//!
//! Advanced space suit engineering for EVA, planetary surface, and deep space operations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuitType {
    IVA,
    EVA,
    Planetary,
    DeepSpace,
    Underwater,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceSuit {
    pub suit_type: SuitType,
    pub model_name: String,
    pub total_mass: f64,            // kg
    pub pressurized_volume: f64,      // m^3
    pub operating_pressure: f64,      // kPa
    pub temperature_range: (f64, f64), // C
    pub autonomy_hours: f64,
    pub mobility_rating: f64,        // percent
    pub radiation_protection: f64,   // g/cm^2
    pub repairability: String,
    pub life_support_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeSupportUnit {
    pub oxygen_capacity: f64,         // kg
    pub co2_scrubber_capacity: f64,   // kg
    pub cooling_capacity: f64,        // W
    pub power_reserve: f64,           // Wh
    pub battery_lifetime: f64,        // hours
    pub backup_systems: bool,
}

impl SpaceSuit {
    pub fn new(suit_type: SuitType, model_name: String) -> Self {
        Self {
            suit_type,
            model_name,
            total_mass: 0.0,
            pressurized_volume: 0.0,
            operating_pressure: 0.0,
            temperature_range: (-50.0, 50.0),
            autonomy_hours: 0.0,
            mobility_rating: 0.0,
            radiation_protection: 0.0,
            repairability: "Field Repairable".into(),
            life_support_type: "Semi-Closed".into(),
        }
    }

    pub fn calculate_mobility_factor(&self) -> f64 {
        self.mobility_rating / 100.0
    }

    pub fn total_mass_with_payload(&self, payload: f64) -> f64 {
        self.total_mass + payload
    }
}

impl LifeSupportUnit {
    pub fn new() -> Self {
        Self {
            oxygen_capacity: 0.8,
            co2_scrubber_capacity: 2.0,
            cooling_capacity: 500.0,
            power_reserve: 2000.0,
            battery_lifetime: 8.0,
            backup_systems: true,
        }
    }

    pub fn remaining_oxygen_hours(&self, usage_rate: f64) -> f64 {
        self.oxygen_capacity / usage_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_suit() {
        let suit = SpaceSuit::new(SuitType::EVA, "EMU Mark III".into());
        assert_eq!(suit.model_name, "EMU Mark III");
    }
}
