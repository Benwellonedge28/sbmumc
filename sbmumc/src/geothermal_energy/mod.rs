//! Geothermal Energy Module (756)
//!
//! Earth heat utilization, geothermal power, and enhanced geothermal systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeothermalType {
    Hydrothermal,
    Enhanced,
    DeepEGS,
    Magmatic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeothermalPlant {
    pub plant_id: String,
    pub geo_type: GeothermalType,
    pub capacity_mwe: f64,
    pub reservoir_temperature_c: f64,
    pub flow_rate_kgs: f64,
    pub drilling_depth_m: f64,
    pub capacity_factor: f64,
}

impl GeothermalPlant {
    pub fn new(plant_id: String) -> Self {
        Self {
            plant_id,
            geo_type: GeothermalType::Hydrothermal,
            capacity_mwe: 0.0,
            reservoir_temperature_c: 0.0,
            flow_rate_kgs: 0.0,
            drilling_depth_m: 0.0,
            capacity_factor: 0.0,
        }
    }

    pub fn thermal_efficiency(&self) -> f64 {
        let t_k = self.reservoir_temperature_c + 273.15;
        let carnot = 1.0 - 288.15 / t_k;
        (carnot * 0.4).max(0.05)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_geothermal() {
        let plant = GeothermalPlant::new("GEO-001".into());
        assert_eq!(plant.plant_id, "GEO-001");
    }
}
