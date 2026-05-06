//! Starship Design Module (669)
//!
//! Interstellar starship engineering, crew systems, and mission architecture.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StarshipClass {
    Interstellar,
    Intergalactic,
    GenerationShip,
    Worldship,
    SeedShip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Starship {
    pub ship_name: String,
    pub starship_class: StarshipClass,
    pub length: f64,                 // m
    pub beam: f64,                   // m
    pub displacement: f64,          // tonnes
    pub crew_capacity: u32,
    pub velocity_max: f64,           // c
    pub propulsion_system: String,
    pub power_system: String,
    pub radiation_shielding: f64,    // g/cm^2
    pub artificial_gravity: f64,      // g
    pub mission_duration_max: f64,   // years
}

impl Starship {
    pub fn new(ship_name: String, starship_class: StarshipClass) -> Self {
        Self {
            ship_name,
            starship_class,
            length: 0.0,
            beam: 0.0,
            displacement: 0.0,
            crew_capacity: 0,
            velocity_max: 0.0,
            propulsion_system: "Fusion Drive".into(),
            power_system: "Antimatter".into(),
            radiation_shielding: 0.0,
            artificial_gravity: 1.0,
            mission_duration_max: 0.0,
        }
    }

    pub fn volume_capacity(&self) -> f64 {
        self.length * self.beam * self.beam * 0.8
    }

    pub fn mass_ratio(&self) -> f64 {
        self.displacement / 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starship() {
        let ship = Starship::new("ISV Venture".into(), StarshipClass::Interstellar);
        assert_eq!(ship.ship_name, "ISV Venture");
    }
}
