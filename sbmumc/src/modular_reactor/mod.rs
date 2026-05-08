//! Small Modular Reactor Module (747)
//!
//! SMR design, factory construction, and distributed nuclear power.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModularReactor {
    pub reactor_id: String,
    pub capacity_mwe: f64,
    pub modules_per_site: u8,
    pub factory_built: bool,
    pub construction_time_months: u16,
    pub cost_per_kwh: f64,
    pub passive_safety: bool,
    pub deployment_status: String,
}

impl ModularReactor {
    pub fn new(reactor_id: String) -> Self {
        Self {
            reactor_id,
            capacity_mwe: 50.0,
            modules_per_site: 1,
            factory_built: true,
            construction_time_months: 36,
            cost_per_kwh: 0.0,
            passive_safety: true,
            deployment_status: "Under Development".into(),
        }
    }

    pub fn lcoe(&self) -> f64 {
        self.cost_per_kwh * 1.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_modular() {
        let reactor = ModularReactor::new("SMR-001".into());
        assert_eq!(reactor.capacity_mwe, 50.0);
    }
}
