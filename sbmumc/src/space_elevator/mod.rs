//! Space Elevator Module (670)
//!
//! Space elevator engineering, cable materials, and tether systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElevatorLocation {
    Earth,
    Moon,
    Mars,
    Ceres,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceElevator {
    pub location: ElevatorLocation,
    pub elevator_name: String,
    pub cable_length: f64,            // km
    pub cable_material: String,
    pub cable_tensile_strength: f64, // GPa
    pub climbers: u32,
    pub climber_speed: f64,          // km/h
    pub payload_capacity: f64,         // kg
    pub construction_status: String,
    pub cost_estimate: f64,           // billion USD
}

impl SpaceElevator {
    pub fn new(location: ElevatorLocation, elevator_name: String) -> Self {
        Self {
            location,
            elevator_name,
            cable_length: 0.0,
            cable_material: "Carbon Nanotube".into(),
            cable_tensile_strength: 0.0,
            climbers: 0,
            climber_speed: 0.0,
            payload_capacity: 0.0,
            construction_status: "Conceptual".into(),
            cost_estimate: 0.0,
        }
    }

    pub fn orbital_velocity(&self) -> f64 {
        let mu = match self.location {
            ElevatorLocation::Earth => 398600.4418,
            ElevatorLocation::Moon => 4904.8695,
            ElevatorLocation::Mars => 42828.3,
            ElevatorLocation::Ceres => 63.1,
        };
        let r = 6371.0 + self.cable_length;
        (mu / r).sqrt()
    }

    pub fn travel_time(&self) -> f64 {
        self.cable_length / self.climber_speed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_elevator() {
        let elevator = SpaceElevator::new(ElevatorLocation::Earth, "Clarke Elevator".into());
        assert_eq!(elevator.elevator_name, "Clarke Elevator");
    }
}
