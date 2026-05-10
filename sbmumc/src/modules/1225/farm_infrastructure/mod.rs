//! # SBMUMC Module 1225: Farm Infrastructure
//!
//! Physical structures and facilities supporting agricultural production.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfrastructureType {
    Storage,
    Processing,
    Irrigation,
    Transportation,
    Energy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmInfrastructureSystem {
    pub system_id: String,
    pub infrastructure_type: InfrastructureType,
    pub capacity_utilization: f64,
    pub efficiency_gain: f64,
    pub maintenance_cost: f64,
    pub resilience_factor: f64,
}

impl FarmInfrastructureSystem {
    pub fn new(infrastructure_type: InfrastructureType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            infrastructure_type,
            capacity_utilization: 0.0,
            efficiency_gain: 0.0,
            maintenance_cost: 0.0,
            resilience_factor: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.infrastructure_type {
            InfrastructureType::Storage => {
                self.capacity_utilization = 0.80 + rand_simple() * 0.18;
                self.efficiency_gain = 0.70 + rand_simple() * 0.25;
                self.resilience_factor = 0.75 + rand_simple() * 0.22;
            },
            InfrastructureType::Processing => {
                self.efficiency_gain = 0.85 + rand_simple() * 0.14;
                self.maintenance_cost = 0.30 + rand_simple() * 0.25;
            },
            InfrastructureType::Irrigation => {
                self.efficiency_gain = 0.80 + rand_simple() * 0.18;
                self.resilience_factor = 0.70 + rand_simple() * 0.25;
            },
            InfrastructureType::Transportation => {
                self.capacity_utilization = 0.75 + rand_simple() * 0.22;
                self.efficiency_gain = 0.80 + rand_simple() * 0.18;
            },
            InfrastructureType::Energy => {
                self.efficiency_gain = 0.75 + rand_simple() * 0.22;
                self.maintenance_cost = 0.40 + rand_simple() * 0.30;
                self.resilience_factor = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.maintenance_cost == 0.0 {
            self.maintenance_cost = 0.35 + rand_simple() * 0.40;
        }
        Ok(())
    }
}

fn rand_simple() -> f64 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_infrastructure() {
        let mut system = FarmInfrastructureSystem::new(InfrastructureType::Storage);
        system.analyze_system().unwrap();
        assert!(system.capacity_utilization > 0.6);
    }
}