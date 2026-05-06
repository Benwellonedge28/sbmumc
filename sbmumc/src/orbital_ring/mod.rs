//! Orbital Ring Module (671)
//!
//! Orbital ring systems, space infrastructure, and megastructure construction.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalRing {
    pub ring_name: String,
    pub orbital_radius: f64,         // km
    pub ring_circumference: f64,     // km
    pub ring_width: f64,            // m
    pub construction_material: String,
    pub mass: f64,                   // tonnes
    pub population_capacity: u32,
    pub power_generation: f64,      // GW
    pub status: String,
    pub connected_stations: u32,
}

impl OrbitalRing {
    pub fn new(ring_name: String, orbital_radius: f64) -> Self {
        Self {
            ring_name,
            orbital_radius,
            ring_circumference: 2.0 * std::f64::consts::PI * orbital_radius,
            ring_width: 0.0,
            construction_material: "Steel".into(),
            mass: 0.0,
            population_capacity: 0,
            power_generation: 0.0,
            status: "Planned".into(),
            connected_stations: 0,
        }
    }

    pub fn centrifugal_force(&self, rotation_rate: f64) -> f64 {
        let omega = rotation_rate / 86400.0; // rad/s from per day
        self.orbital_radius * 1000.0 * omega.powi(2)
    }

    pub fn artificial_gravity(&self, rotation_rate: f64) -> f64 {
        self.centrifugal_force(rotation_rate) / 9.80665
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbital_ring() {
        let ring = OrbitalRing::new("Ring 1".into(), 42164.0);
        assert!(ring.ring_circumference > 0.0);
    }
}
