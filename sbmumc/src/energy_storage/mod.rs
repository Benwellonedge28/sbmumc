//! Energy Storage Module (758)
//!
//! Grid storage, battery systems, and energy buffering technologies.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    LithiumIon,
    Flow,
    CompressedAir,
    PumpedHydro,
    Gravity,
    Thermal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyStorageSystem {
    pub storage_id: String,
    pub storage_type: StorageType,
    pub capacity_mwh: f64,
    pub power_rating_mw: f64,
    pub discharge_hours: f64,
    pub round_trip_efficiency: f64,
    pub cycles: u32,
    pub response_time_ms: f64,
}

impl EnergyStorageSystem {
    pub fn new(storage_id: String) -> Self {
        Self {
            storage_id,
            storage_type: StorageType::LithiumIon,
            capacity_mwh: 0.0,
            power_rating_mw: 0.0,
            discharge_hours: 0.0,
            round_trip_efficiency: 0.0,
            cycles: 0,
            response_time_ms: 100.0,
        }
    }

    pub fn energy_to_power_ratio(&self) -> f64 {
        self.capacity_mwh / self.power_rating_mw.max(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_storage() {
        let storage = EnergyStorageSystem::new("ES-001".into());
        assert_eq!(storage.storage_id, "ES-001");
    }
}
