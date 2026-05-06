//! Lunar Base Design Module (656)
//!
//! Lunar surface habitation, resource extraction, and operational systems design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LunarBaseLocation {
    MareTranquillitatis,
    MareImbrium,
    PoleNorth,
    PoleSouth,
    FarSide,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LunarBase {
    pub base_name: String,
    pub location: LunarBaseLocation,
    pub construction_status: String,
    pub crew_capacity: u32,
    pub habitat_area: f64,         // m^2
    regolith_shielding_depth: f64,  // m
    power_system: String,
    power_output: f64,             // kW
    communication_link: String,
    landing_facilities: u32,
    rovers: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LunarResources {
    pub water_ice_reserves: f64,   // tonnes
    pub helium3_reserves: f64,      // tonnes
    pub rare_earth_elements: f64,  // tonnes
    pub iron_oxide: f64,           // percent
    pub silicon: f64,              // percent
    pub aluminum: f64,            // percent
    pub extraction_rate: f64,      // kg/day
}

impl LunarBase {
    pub fn new(base_name: String, location: LunarBaseLocation) -> Self {
        Self {
            base_name,
            location,
            construction_status: "Planned".into(),
            crew_capacity: 0,
            habitat_area: 0.0,
            regolith_shielding_depth: 2.5,
            power_system: "Solar + RTG".into(),
            power_output: 0.0,
            communication_link: "Earth Direct".into(),
            landing_facilities: 0,
            rovers: 0,
        }
    }

    pub fn calculate_shielding_mass(&self) -> f64 {
        let density = 1500.0; // kg/m^3 regolith
        self.habitat_area * self.regolith_shielding_depth * density
    }

    pub fn power_per_crewmember(&self) -> f64 {
        self.power_output / self.crew_capacity.max(1) as f64
    }
}

impl LunarResources {
    pub fn new() -> Self {
        Self {
            water_ice_reserves: 0.0,
            helium3_reserves: 0.0,
            rare_earth_elements: 0.0,
            iron_oxide: 15.0,
            silicon: 20.0,
            aluminum: 10.0,
            extraction_rate: 0.0,
        }
    }

    pub fn is_extractable(&self) -> bool {
        self.water_ice_reserves > 1000.0 || self.helium3_reserves > 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lunar_base() {
        let base = LunarBase::new("Artemis Base Alpha".into(), LunarBaseLocation::PoleSouth);
        assert_eq!(base.base_name, "Artemis Base Alpha");
    }
}
