//! Fission Reactor Module (744)
//!
//! Nuclear fission power plants, fuel cycles, and reactor operations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReactorType {
    PWR,
    BWR,
    PHWR,
    CANDU,
    HTGR,
    MSR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FissionReactor {
    pub reactor_id: String,
    pub reactor_type: ReactorType,
    pub thermal_power_mwt: f64,
    pub electrical_power_mwe: f64,
    pub burnup_mwd_t: f64,
    pub enrichment_percent: f64,
    pub capacity_factor: f64,
    pub operational_status: String,
}

impl FissionReactor {
    pub fn new(reactor_id: String, reactor_type: ReactorType) -> Self {
        Self {
            reactor_id,
            reactor_type,
            thermal_power_mwt: 0.0,
            electrical_power_mwe: 0.0,
            burnup_mwd_t: 0.0,
            enrichment_percent: 0.0,
            capacity_factor: 0.0,
            operational_status: "Operating".into(),
        }
    }

    pub fn thermal_efficiency(&self) -> f64 {
        (self.electrical_power_mwe / self.thermal_power_mwt.max(1.0)) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fission() {
        let reactor = FissionReactor::new("FR-001".into(), ReactorType::PWR);
        assert!(matches!(reactor.reactor_type, ReactorType::PWR));
    }
}
