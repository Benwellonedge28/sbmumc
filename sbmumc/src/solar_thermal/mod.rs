//! Solar Thermal Module (752)
//!
//! Concentrated solar power, thermal storage, and solar heating systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SolarThermalType {
    ParabolicTrough,
    SolarTower,
    ParabolicDish,
    LinearFresnel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarThermalSystem {
    pub system_id: String,
    pub thermal_type: SolarThermalType,
    pub capacity_mwe: f64,
    pub collector_area_m2: f64,
    pub thermal_efficiency_percent: f64,
    pub thermal_storage_mwht: f64,
    pub annual_output_mwh: f64,
}

impl SolarThermalSystem {
    pub fn new(system_id: String) -> Self {
        Self {
            system_id,
            thermal_type: SolarThermalType::ParabolicTrough,
            capacity_mwe: 0.0,
            collector_area_m2: 0.0,
            thermal_efficiency_percent: 0.0,
            thermal_storage_mwht: 0.0,
            annual_output_mwh: 0.0,
        }
    }

    pub fn capacity_factor(&self, dni_kwh_m2_day: f64) -> f64 {
        (self.annual_output_mwh / (self.capacity_mwe * 8760.0) * 100.0).min(100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solar_thermal() {
        let system = SolarThermalSystem::new("ST-001".into());
        assert_eq!(system.system_id, "ST-001");
    }
}
