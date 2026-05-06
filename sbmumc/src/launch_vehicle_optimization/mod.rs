//! Launch Vehicle Optimization Module (684)
//!
//! Launch vehicle performance optimization, trajectory design, and cost reduction.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VehicleClass {
    SmallLift,     // < 500 kg to LEO
    MediumLift,    // 500-2000 kg to LEO
    HeavyLift,     // 2-20 tonnes to LEO
    SuperHeavyLift, // > 20 tonnes to LEO
    Orbital,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchVehicle {
    pub vehicle_name: String,
    pub vehicle_class: VehicleClass,
    pub payload_capacity_leo: f64,     // kg
    pub payload_capacity_gto: f64,     // kg
    pub liftoff_thrust: f64,            // kN
    pub mass_to_orbit: f64,            // tonnes
    pub cost_per_kg: f64,              // USD/kg
    pub reusability: String,
    pub turnaround_days: f64,
    pub success_rate: f64,             // percent
}

impl LaunchVehicle {
    pub fn new(vehicle_name: String, vehicle_class: VehicleClass) -> Self {
        Self {
            vehicle_name,
            vehicle_class,
            payload_capacity_leo: 0.0,
            payload_capacity_gto: 0.0,
            liftoff_thrust: 0.0,
            mass_to_orbit: 0.0,
            cost_per_kg: 0.0,
            reusability: "Expendable".into(),
            turnaround_days: 0.0,
            success_rate: 95.0,
        }
    }

    pub fn launch_cost(&self, payload_mass: f64) -> f64 {
        payload_mass * self.cost_per_kg
    }

    pub fn cost_performance(&self) -> f64 {
        1000.0 / self.cost_per_kg.max(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launch_vehicle() {
        let vehicle = LaunchVehicle::new("Falcon 9".into(), VehicleClass::HeavyLift);
        assert_eq!(vehicle.vehicle_name, "Falcon 9");
    }
}
