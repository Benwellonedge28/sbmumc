//! Nuclear Fuel Module (750)
//!
//! Uranium fuel cycles, fuel fabrication, and fuel performance.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FuelType {
    UO2,
    MOX,
    UC,
    UN,
    Thorium,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearFuel {
    pub fuel_id: String,
    pub fuel_type: FuelType,
    pub enrichment_percent: f64,
    pub burnup_mwd_t: f64,
    pub fabrication_method: String,
    pub in_core_lifetime_days: u32,
    pub defect_rate_percent: f64,
}

impl NuclearFuel {
    pub fn new(fuel_id: String, fuel_type: FuelType) -> Self {
        Self {
            fuel_id,
            fuel_type,
            enrichment_percent: 0.0,
            burnup_mwd_t: 0.0,
            fabrication_method: "Pellet".into(),
            in_core_lifetime_days: 0,
            defect_rate_percent: 0.0,
        }
    }

    pub fn fuel_efficiency(&self) -> f64 {
        (self.burnup_mwd_t / self.enrichment_percent).min(1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nuclear_fuel() {
        let fuel = NuclearFuel::new("FUEL-001".into(), FuelType::UO2);
        assert!(matches!(fuel.fuel_type, FuelType::UO2));
    }
}
