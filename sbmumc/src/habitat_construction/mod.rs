//! Habitat Construction Module (653)
//!
//! Space habitat architecture, construction techniques, and life support integration.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HabitatLocation {
    EarthSurface,
    Lunar,
    Martian,
    Asteroid,
    SpaceStation,
    DeepSpace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitatStructure {
    pub name: String,
    pub location: HabitatLocation,
    pub floor_area: f64,          // m^2
    pub volume: f64,               // m^3
    pub wall_thickness: f64,       // m
    pub material: String,
    pub radiation_shielding: f64,  // g/cm^2
    pub thermal_mass: f64,          // J/K
    pub airlock_count: u32,
    pub window_area: f64,          // m^2
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructionRobot {
    pub robot_type: String,
    pub reach: f64,               // m
    pub payload: f64,             // kg
    pub precision: f64,           // mm
    pub autonomy_level: f64,     // percent
    pub power_consumption: f64,   // kW
    pub construction_rate: f64,   // kg/hour
}

impl HabitatStructure {
    pub fn new(name: String, location: HabitatLocation) -> Self {
        Self {
            name,
            location,
            floor_area: 0.0,
            volume: 0.0,
            wall_thickness: 0.0,
            material: "Aluminum-Lithium".into(),
            radiation_shielding: 0.0,
            thermal_mass: 0.0,
            airlock_count: 0,
            window_area: 0.0,
        }
    }

    pub fn calculate_mass(&self) -> f64 {
        let density = match self.material.as_str() {
            "Aluminum-Lithium" => 2700.0,
            "Steel" => 7800.0,
            "Titanium" => 4500.0,
            "Regolith" => 1500.0,
            _ => 2500.0,
        };
        let surface_area = self.floor_area * 2.0 + (self.floor_area.sqrt() * 4.0 * 3.0);
        surface_area * self.wall_thickness * density
    }

    pub fn radiation_attenuation(&self) -> f64 {
        (-self.radiation_shielding / 20.0).exp()
    }
}

pub struct HabitatAnalysis;

impl HabitatAnalysis {
    pub fn internal_volume(floor_area: f64, height: f64) -> f64 {
        floor_area * height
    }

    pub fn thermal_time_constant(mass: f64, specific_heat: f64, surface_area: f64, insulation: f64) -> f64 {
        mass * specific_heat / (surface_area / insulation)
    }

    pub fn radiation_dose(debris_shields: f64, solar_activity: f64) -> f64 {
        debris_shields * solar_activity * 0.01
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_habitat_structure() {
        let habitat = HabitatStructure::new("Lunar Base Alpha".into(), HabitatLocation::Lunar);
        assert_eq!(habitat.name, "Lunar Base Alpha");
    }
}
