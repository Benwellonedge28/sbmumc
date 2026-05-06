//! Orbital Mechanics Module (643)
//!
//! Comprehensive orbital dynamics simulation and analysis for space exploration.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrbitType {
    Circular,
    Elliptical,
    Parabolic,
    Hyperbolic,
    Geostationary,
    Molniya,
    LEO,
    MEO,
    HEO,
    GEO,
    Polar,
    SunSynchronous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalElements {
    pub semi_major_axis: f64,       // km
    pub eccentricity: f64,
    pub inclination: f64,           // degrees
    pub right_ascension: f64,      // degrees
    pub argument_of_periapsis: f64, // degrees
    pub true_anomaly: f64,         // degrees
    pub mean_anomaly: f64,         // degrees
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalState {
    pub position: [f64; 3],        // km
    pub velocity: [f64; 3],        // km/s
    pub elements: OrbitalElements,
    pub central_body_mass: f64,    // kg
    pub gravitational_parameter: f64, // km^3/s^2
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoBodyProblem {
    pub mu: f64,
    pub r1: [f64; 3],
    pub r2: [f64; 3],
    pub v1: [f64; 3],
    pub v2: [f64; 3],
    pub time_of_flight: f64,
}

impl OrbitalElements {
    pub fn new(semi_major_axis: f64, eccentricity: f64, inclination: f64,
               right_ascension: f64, argument_of_periapsis: f64, true_anomaly: f64) -> Self {
        Self {
            semi_major_axis,
            eccentricity,
            inclination,
            right_ascension,
            argument_of_periapsis,
            true_anomaly,
            mean_anomaly: 0.0,
        }
    }

    pub fn period(&self) -> f64 {
        let a = self.semi_major_axis;
        let mu = 398600.4418; // Earth's gravitational parameter
        2.0 * std::f64::consts::PI * (a * a * a / mu).sqrt()
    }

    pub fn velocity_at_periapsis(&self) -> f64 {
        let a = self.semi_major_axis;
        let e = self.eccentricity;
        let mu = 398600.4418;
        ((1.0 + e) / (1.0 - e)).sqrt() * (mu / a).sqrt()
    }

    pub fn velocity_at_apoapsis(&self) -> f64 {
        let a = self.semi_major_axis;
        let e = self.eccentricity;
        let mu = 398600.4418;
        ((1.0 - e) / (1.0 + e)).sqrt() * (mu / a).sqrt()
    }
}

impl OrbitalState {
    pub fn from_elements(elements: OrbitalElements, central_body_mass: f64) -> Self {
        let mu = 6.67430e-20 * central_body_mass; // G * M in km^3/s^2
        Self {
            position: [0.0; 3],
            velocity: [0.0; 3],
            elements,
            central_body_mass,
            gravitational_parameter: mu,
        }
    }

    pub fn calculate_orbital_period(&self) -> f64 {
        self.elements.period()
    }

    pub fn escape_velocity(&self) -> f64 {
        let r = (self.position[0].powi(2) + self.position[1].powi(2) + self.position[2].powi(2)).sqrt();
        (2.0 * self.gravitational_parameter / r).sqrt()
    }
}

impl TwoBodyProblem {
    pub fn new(mu: f64, r1: [f64; 3], r2: [f64; 3], v1: [f64; 3]) -> Self {
        Self {
            mu,
            r1,
            r2,
            v1,
            v2: [0.0; 3],
            time_of_flight: 0.0,
        }
    }

    pub fn solve_hohmann_transfer(&self, r_start: f64, r_end: f64) -> Result<(f64, f64)> {
        if r_start <= 0.0 || r_end <= 0.0 {
            return Err(SbmumcError::InvalidInput("Invalid orbital radii".into()));
        }
        let a_transfer = (r_start + r_end) / 2.0;
        let dv1 = ((2.0 * self.mu / r_start) - (self.mu / a_transfer)).sqrt() - 
                  (self.mu / r_start).sqrt();
        let dv2 = (self.mu / r_end).sqrt() - 
                  ((2.0 * self.mu / r_end) - (self.mu / a_transfer)).sqrt();
        Ok((dv1, dv2))
    }
}

pub struct OrbitalMechanics;

impl OrbitalMechanics {
    pub fn calculate_vis_viva_velocity(r: f64, a: f64, mu: f64) -> f64 {
        (mu * (2.0/r - 1.0/a)).sqrt()
    }

    pub fn calculate_specific_angular_momentum(r: [f64; 3], v: [f64; 3]) -> [f64; 3] {
        [
            r[1]*v[2] - r[2]*v[1],
            r[2]*v[0] - r[0]*v[2],
            r[0]*v[1] - r[1]*v[0],
        ]
    }

    pub fn calculate_orbital_energy(r: f64, v: f64, mu: f64) -> f64 {
        v*v/2.0 - mu/r
    }

    pub fn lambert_solver(mu: f64, r1: [f64; 3], r2: [f64; 3], tof: f64, N: i32, M: i32) -> Result<f64> {
        let r1_mag = (r1[0].powi(2) + r1[1].powi(2) + r1[2].powi(2)).sqrt();
        let r2_mag = (r2[0].powi(2) + r2[1].powi(2) + r2[2].powi(2)).sqrt();
        if r1_mag <= 0.0 || r2_mag <= 0.0 || tof <= 0.0 {
            return Err(SbmumcError::InvalidInput("Invalid Lambert problem parameters".into()));
        }
        Ok(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbital_elements() {
        let elements = OrbitalElements::new(6778.0, 0.001, 28.5, 0.0, 0.0, 0.0);
        assert!(elements.semi_major_axis > 0.0);
        assert!(elements.eccentricity < 1.0);
    }

    #[test]
    fn test_vis_viva_velocity() {
        let v = OrbitalMechanics::calculate_vis_viva_velocity(6778.0, 6778.0, 398600.4418);
        assert!(v > 0.0);
    }
}
