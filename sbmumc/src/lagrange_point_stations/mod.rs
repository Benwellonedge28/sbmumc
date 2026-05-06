//! Lagrange Point Stations Module (673)
//!
//! Space stations positioned at gravitational equilibrium points.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LagrangePoint {
    L1,
    L2,
    L3,
    L4,
    L5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LagrangeStation {
    pub station_name: String,
    pub lagrange_point: LagrangePoint,
    pub primary_body: String,
    pub secondary_body: String,
    pub distance_from_primary: f64,   // km
    pub station_type: String,
    pub purpose: String,
    pub mass: f64,                    // tonnes
    pub power_generation: f64,        // kW
    pub crew_capacity: u32,
    pub construction_status: String,
}

impl LagrangeStation {
    pub fn new(station_name: String, lagrange_point: LagrangePoint) -> Self {
        Self {
            station_name,
            lagrange_point,
            primary_body: "Sun".into(),
            secondary_body: "Earth".into(),
            distance_from_primary: 0.0,
            station_type: "Observation".into(),
            purpose: "Science".into(),
            mass: 0.0,
            power_generation: 0.0,
            crew_capacity: 0,
            construction_status: "Conceptual".into(),
        }
    }

    pub fn stability_factor(&self) -> f64 {
        match self.lagrange_point {
            LagrangePoint::L4 | LagrangePoint::L5 => 100.0,
            _ => 50.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lagrange_station() {
        let station = LagrangeStation::new("L1 Gateway".into(), LagrangePoint::L1);
        assert_eq!(station.lagrange_point, LagrangePoint::L1);
    }
}
