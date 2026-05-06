//! Oneill Colony Module (674)
//!
//! O'Neill cylinder space colony design, construction, and habitation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CylinderClass {
    IslandOne,      // 1.6 km radius, 3.2 km length
    IslandTwo,      // 3.2 km radius, 6.4 km length
    IslandThree,    // 5 km radius, 10 km length
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneillColony {
    pub colony_name: String,
    pub cylinder_class: CylinderClass,
    pub radius: f64,                 // km
    pub length: f64,                  // km
    pub rotational_speed: f64,        // rpm
    pub surface_area: f64,            // km^2
    pub population_capacity: u32,
    pub interior_terrain: String,
    pub agricultural_area: f64,       // km^2
    pub industrial_area: f64,           // km^2
    pub water_reserves: f64,           // km^3
    pub construction_cost: f64,       // billion USD
}

impl OneillColony {
    pub fn new(colony_name: String, cylinder_class: CylinderClass) -> Self {
        let (radius, length) = match cylinder_class {
            CylinderClass::IslandOne => (1.6, 3.2),
            CylinderClass::IslandTwo => (3.2, 6.4),
            CylinderClass::IslandThree => (5.0, 10.0),
            CylinderClass::Custom => (4.0, 8.0),
        };
        Self {
            colony_name,
            cylinder_class,
            radius,
            length,
            rotational_speed: 0.0,
            surface_area: 0.0,
            population_capacity: 0,
            interior_terrain: "Mountainous".into(),
            agricultural_area: 0.0,
            industrial_area: 0.0,
            water_reserves: 0.0,
            construction_cost: 0.0,
        }
    }

    pub fn calculate_artificial_gravity(&self) -> f64 {
        let radius_m = self.radius * 1000.0;
        let omega = self.rotational_speed * 2.0 * std::f64::consts::PI / 60.0;
        radius_m * omega.powi(2) / 9.80665
    }

    pub fn total_surface_area(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius * self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oneill_colony() {
        let colony = OneillColony::new("Colony Alpha".into(), CylinderClass::IslandTwo);
        assert_eq!(colony.colony_name, "Colony Alpha");
    }
}
