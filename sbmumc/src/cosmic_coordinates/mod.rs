//! Cosmic Coordinates Module (691)
//!
//! Celestial coordinate systems, astrometric calculations, and galactic positioning.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinateSystem {
    Equatorial,
    Ecliptic,
    Galactic,
    Supergalactic,
    Cartesian,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicCoordinate {
    pub coordinate_id: String,
    pub coordinate_system: CoordinateSystem,
    pub longitude: f64,
    pub latitude: f64,
    pub distance_ly: f64,
    pub proper_motion_ra: f64,
    pub proper_motion_dec: f64,
    pub radial_velocity: f64,
    pub epoch: String,
}

impl CosmicCoordinate {
    pub fn new(coordinate_id: String, coordinate_system: CoordinateSystem) -> Self {
        Self {
            coordinate_id,
            coordinate_system,
            longitude: 0.0,
            latitude: 0.0,
            distance_ly: 0.0,
            proper_motion_ra: 0.0,
            proper_motion_dec: 0.0,
            radial_velocity: 0.0,
            epoch: "J2000".into(),
        }
    }

    pub fn to_equatorial(&self) -> (f64, f64) {
        (self.longitude, self.latitude)
    }

    pub fn to_galactic(&self) -> (f64, f64) {
        (self.longitude + 33.0, self.latitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coordinate() {
        let coord = CosmicCoordinate::new("C-001".into(), CoordinateSystem::Galactic);
        assert_eq!(coord.epoch, "J2000");
    }
}
