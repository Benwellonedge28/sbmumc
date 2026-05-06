//! Space Tether Module (672)
//!
//! Space tether dynamics, momentum exchange, and electrodynamic tether systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TetherType {
    MomentumExchange,
    Electrodynamic,
    Skyhook,
    Rotovator,
    StationTether,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceTether {
    pub tether_name: String,
    pub tether_type: TetherType,
    pub length: f64,                 // km
    pub tether_material: String,
    pub tensile_strength: f64,      // GPa
    pub mass: f64,                   // kg
    pub end_mass: f64,              // kg
    pub orbital_altitude: f64,       // km
    pub deployment_status: String,
    pub applications: Vec<String>,
}

impl SpaceTether {
    pub fn new(tether_name: String, tether_type: TetherType, length: f64) -> Self {
        Self {
            tether_name,
            tether_type,
            length,
            tether_material: "Kevlar".into(),
            tensile_strength: 0.0,
            mass: 0.0,
            end_mass: 0.0,
            orbital_altitude: 0.0,
            deployment_status: "Conceptual".into(),
            applications: Vec::new(),
        }
    }

    pub fn orbital_velocity(&self) -> f64 {
        let mu = 398600.4418;
        let r = 6371.0 + self.orbital_altitude;
        (mu / r).sqrt()
    }

    pub fn tether_tension(&self) -> f64 {
        (self.mass + self.end_mass) * self.orbital_velocity().powi(2) / self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_tether() {
        let tether = SpaceTether::new("Tether 1".into(), TetherType::MomentumExchange, 20.0);
        assert!(tether.length > 0.0);
    }
}
