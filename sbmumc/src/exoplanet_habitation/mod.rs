//! Exoplanet Habitation Module (657)
//!
//! Habitation engineering for exoplanets and habitable zone planetary bodies.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExoplanetClass {
    EarthAnalog,
    SuperEarth,
    SubEarth,
    OceanWorld,
    DesertWorld,
    TidallyLocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExoplanetHabitation {
    pub system_name: String,
    pub planet_name: String,
    pub star_type: String,
    pub distance_ly: f64,
    pub habitable_zone: f64,         // AU
    pub surface_gravity: f64,        // g
    pub atmosphere_composition: Vec<(String, f64)>,
    pub surface_temperature: f64,    // K
    pub colonization_difficulty: String,
    pub terraforming_potential: f64, // percent
    pub protection_needs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereModification {
    pub target_composition: Vec<(String, f64)>,
    pub current_composition: Vec<(String, f64)>,
    pub modification_technique: String,
    pub energy_required: f64,         // GW
    pub timeline_years: f64,
}

impl ExoplanetHabitation {
    pub fn new(system_name: String, planet_name: String) -> Self {
        Self {
            system_name,
            planet_name,
            star_type: "G".into(),
            distance_ly: 0.0,
            habitable_zone: 0.0,
            surface_gravity: 1.0,
            atmosphere_composition: Vec::new(),
            surface_temperature: 0.0,
            colonization_difficulty: "Unknown".into(),
            terraforming_potential: 0.0,
            protection_needs: Vec::new(),
        }
    }

    pub fn calculate_travel_time(&self, velocity: f64) -> f64 {
        let c = 299792458.0;
        self.distance_ly * 9.461e15 / (velocity * c)
    }

    pub fn assess_habitability(&self) -> f64 {
        let mut score = 50.0;
        if self.surface_temperature > 260.0 && self.surface_temperature < 320.0 {
            score += 30.0;
        }
        if self.surface_gravity > 0.5 && self.surface_gravity < 1.5 {
            score += 20.0;
        }
        score
    }
}

pub struct ExoplanetAnalysis;

impl ExoplanetAnalysis {
    pub fn goldilocks_zone(star_luminosity: f64) -> f64 {
        (star_luminosity.sqrt() - 0.2).max(0.1)
    }

    pub fn escape_velocity(radius: f64, mass: f64) -> f64 {
        let g = 6.67430e-11;
        (2.0 * g * mass / radius).sqrt() / 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exoplanet_habitation() {
        let hab = ExoplanetHabitation::new("Trappist".into(), "Trappist-1e".into());
        assert_eq!(hab.planet_name, "Trappist-1e");
    }
}
