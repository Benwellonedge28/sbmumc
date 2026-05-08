//! Offshore Wind Module (755)
//!
//! Offshore wind farms, floating platforms, and marine wind energy.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FoundationType {
    Monopile,
    Jacket,
    Gravity,
    Floating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffshoreWindFarm {
    pub farm_id: String,
    pub foundation_type: FoundationType,
    pub turbines: u32,
    pub capacity_mw: f64,
    pub distance_shore_km: f64,
    pub water_depth_m: f64,
    pub capacity_factor: f64,
    pub cable_length_km: f64,
}

impl OffshoreWindFarm {
    pub fn new(farm_id: String) -> Self {
        Self {
            farm_id,
            foundation_type: FoundationType::Monopile,
            turbines: 0,
            capacity_mw: 0.0,
            distance_shore_km: 0.0,
            water_depth_m: 0.0,
            capacity_factor: 0.0,
            cable_length_km: 0.0,
        }
    }

    pub fn avg_turbine_spacing_diameter(&self) -> f64 {
        7.0 * self.water_depth_m.max(10.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_offshore() {
        let farm = OffshoreWindFarm::new("OWF-001".into());
        assert_eq!(farm.farm_id, "OWF-001");
    }
}
