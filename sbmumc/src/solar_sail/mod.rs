//! Solar Sail Module (650)
//!
//! Solar photon propulsion system design and orbital dynamics analysis.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarSail {
    pub sail_material: String,
    pub sail_area: f64,           // m^2
    pub sail_mass: f64,          // kg
    pub payload_mass: f64,       // kg
    pub total_mass: f64,         // kg
    pub reflectivity: f64,
    pub emissivity: f64,
    pub temperature_limit: f64,   // K
    pub deployment_status: String,
    pub sail_shape: String,
    pub aspect_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarPressure {
    pub solar_constant: f64,      // W/m^2 at 1 AU
    pub distance_au: f64,          // AU from Sun
    pub radiation_pressure: f64,  // Pa
    pub particle_pressure: f64,   // Pa
}

impl SolarSail {
    pub fn new(sail_material: String, sail_area: f64, sail_mass: f64) -> Self {
        Self {
            sail_material,
            sail_area,
            sail_mass,
            payload_mass: 0.0,
            total_mass: sail_mass,
            reflectivity: 0.88,
            emissivity: 0.05,
            temperature_limit: 600.0,
            deployment_status: "Stowed".into(),
            sail_shape: "Square".into(),
            aspect_ratio: 1.0,
        }
    }

    pub fn calculate_solar_pressure(&self, distance_au: f64) -> f64 {
        let solar_constant = 1367.0; // W/m^2 at 1 AU
        let c = 299792458.0;
        solar_constant / (distance_au * distance_au) / c
    }

    pub fn calculate_thrust(&self, distance_au: f64) -> f64 {
        let pressure = self.calculate_solar_pressure(distance_au);
        let thrust_coefficient = 2.0 * self.reflectivity / (1.0 + self.emissivity);
        thrust_coefficient * pressure * self.sail_area * 1e6 // mN
    }

    pub fn acceleration(&self, distance_au: f64) -> f64 {
        let thrust = self.calculate_thrust(distance_au) * 1e-3; // N
        thrust / self.total_mass.max(1.0) // m/s^2
    }

    pub fn characteristic_acceleration(&self) -> f64 {
        let solar_constant = 1367.0;
        let c = 299792458.0;
        let pressure = solar_constant / c;
        let thrust_coefficient = 2.0 * self.reflectivity / (1.0 + self.emissivity);
        thrust_coefficient * pressure * self.sail_area * 1e6 / self.total_mass.max(1.0)
    }
}

pub struct SolarSailAnalysis;

impl SolarSailAnalysis {
    pub fn heliocentric_velocity(sail_area: f64, mass: f64, distance: f64) -> f64 {
        let solar_constant = 1367.0;
        let c = 299792458.0;
        let pressure = solar_constant / (distance * distance) / c;
        let thrust = 2.0 * 0.88 * pressure * sail_area * 1e6;
        let accel = thrust / mass;
        accel * 86400.0 * 365.25 // km/s per year
    }

    pub fn escape_velocity(mass: f64, distance: f64) -> f64 {
        let mu = 1.32712440018e20; // Sun's gravitational parameter
        (2.0 * mu / (distance * 1.495978707e11)).sqrt() / 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_sail() {
        let sail = SolarSail::new("CP-1".into(), 10000.0, 5.0);
        assert!(sail.sail_area > 0.0);
    }
}
