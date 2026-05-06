//! Gravitational Assist Module (644)
//!
//! Gravity assist trajectory planning and optimization for interplanetary missions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravityAssistBody {
    pub name: String,
    pub mass: f64,              // kg
    pub radius: f64,            // km
    pub gravitational_parameter: f64, // km^3/s^2
    pub orbital_radius: f64,    // km
    pub orbital_velocity: f64, // km/s
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravityAssistTrajectory {
    pub flyby_body: GravityAssistBody,
    pub entry_altitude: f64,   // km
    pub exit_altitude: f64,    // km
    pub entry_velocity: [f64; 3], // km/s
    pub exit_velocity: [f64; 3],  // km/s
    pub delta_v_gain: f64,     // km/s
    pub bending_angle: f64,     // degrees
    pub periapse_altitude: f64, // km
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwingBySequence {
    pub bodies: Vec<GravityAssistBody>,
    pub departure_date: f64,   // MJD
    pub arrival_date: f64,     // MJD
    pub total_delta_v: f64,    // km/s
    pub sequence_length: usize,
}

impl GravityAssistTrajectory {
    pub fn new(flyby_body: GravityAssistBody) -> Self {
        Self {
            flyby_body,
            entry_altitude: 0.0,
            exit_altitude: 0.0,
            entry_velocity: [0.0; 3],
            exit_velocity: [0.0; 3],
            delta_v_gain: 0.0,
            bending_angle: 0.0,
            periapse_altitude: 0.0,
        }
    }

    pub fn calculate_turn_angle(&self) -> f64 {
        let rp = self.flyby_body.radius + self.periapse_altitude;
        let vinf = self.entry_velocity.iter().map(|v| v.powi(2)).sum::<f64>().sqrt();
        let mu = self.flyby_body.gravitational_parameter;
        let delta = (std::f64::consts::PI / 180.0 * (90.0 + (2.0 * rp * vinf*vinf / mu).asin())).abs();
        delta * 180.0 / std::f64::consts::PI
    }

    pub fn calculate_delta_v(&self) -> f64 {
        let vin_entry = self.entry_velocity.iter().map(|v| v.powi(2)).sum::<f64>().sqrt();
        let vin_exit = self.exit_velocity.iter().map(|v| v.powi(2)).sum::<f64>().sqrt();
        2.0 * vin_entry * (1.0 - (self.bending_angle * std::f64::consts::PI / 180.0).cos().abs().sqrt() * (vin_exit/vin_entry).signum().abs())
    }

    pub fn optimize_flyby_altitude(&self) -> Result<f64> {
        if self.flyby_body.radius <= 0.0 {
            return Err(SbmumcError::InvalidInput("Invalid body radius".into()));
        }
        Ok(self.flyby_body.radius * 3.0)
    }
}

impl SwingBySequence {
    pub fn new(departure_date: f64) -> Self {
        Self {
            bodies: Vec::new(),
            departure_date,
            arrival_date: 0.0,
            total_delta_v: 0.0,
            sequence_length: 0,
        }
    }

    pub fn add_flyby(&mut self, body: GravityAssistBody) {
        self.bodies.push(body);
        self.sequence_length += 1;
    }

    pub fn calculate_total_delta_v(&mut self) {
        self.total_delta_v = self.bodies.len() as f64 * 2.5;
    }
}

pub struct GravitationalAssist;

impl GravitationalAssist {
    pub fn calculate_sphere_of_influence(primary_mass: f64, secondary_mass: f64, orbital_radius: f64) -> f64 {
        orbital_radius * (secondary_mass / primary_mass).powf(2.0/3.0)
    }

    pub fn calculate_patch_point(primary: &GravityAssistBody, secondary: &GravityAssistBody, position: f64) -> f64 {
        let r_soi = Self::calculate_sphere_of_influence(
            primary.mass, secondary.mass, secondary.orbital_radius
        );
        (primary.radius + secondary.radius + r_soi) / 2.0
    }

    pub fn interplanetary_transfer_delta_v(mu: f64, r1: f64, r2: f64) -> f64 {
        let a = (r1 + r2) / 2.0;
        let dv1 = ((2.0 * mu / r1) - (mu / a)).sqrt() - (mu / r1).sqrt();
        let dv2 = (mu / r2).sqrt() - ((2.0 * mu / r2) - (mu / a)).sqrt();
        dv1.abs() + dv2.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity_assist_creation() {
        let jupiter = GravityAssistBody {
            name: "Jupiter".into(),
            mass: 1.898e27,
            radius: 69911.0,
            gravitational_parameter: 126686534.9,
            orbital_radius: 778500000.0,
            orbital_velocity: 13.07,
        };
        let trajectory = GravityAssistTrajectory::new(jupiter);
        assert_eq!(trajectory.flyby_body.name, "Jupiter");
    }
}
