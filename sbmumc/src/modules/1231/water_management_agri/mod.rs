//! # SBMUMC Module 1231: Water Management Agriculture
//!
//! Sustainable use of water resources in farming.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WaterManagementStrategy {
    RainwaterHarvesting,
    RecycledWater,
    DeficitIrrigation,
    WaterTrading,
    WatershedManagement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterManagementAgricultureSystem {
    pub system_id: String,
    pub management_strategy: WaterManagementStrategy,
    pub water_use_efficiency: f64,
    pub allocation_equity: f64,
    pub environmental_flow: f64,
    pub supply_reliability: f64,
}

impl WaterManagementAgricultureSystem {
    pub fn new(management_strategy: WaterManagementStrategy) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            management_strategy,
            water_use_efficiency: 0.0,
            allocation_equity: 0.0,
            environmental_flow: 0.0,
            supply_reliability: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.management_strategy {
            WaterManagementStrategy::RainwaterHarvesting => {
                self.water_use_efficiency = 0.75 + rand_simple() * 0.22;
                self.supply_reliability = 0.70 + rand_simple() * 0.25;
            },
            WaterManagementStrategy::RecycledWater => {
                self.water_use_efficiency = 0.85 + rand_simple() * 0.14;
                self.allocation_equity = 0.65 + rand_simple() * 0.30;
            },
            WaterManagementStrategy::DeficitIrrigation => {
                self.water_use_efficiency = 0.90 + rand_simple() * 0.10;
                self.supply_reliability = 0.80 + rand_simple() * 0.18;
            },
            WaterManagementStrategy::WaterTrading => {
                self.allocation_equity = 0.80 + rand_simple() * 0.18;
                self.supply_reliability = 0.75 + rand_simple() * 0.22;
            },
            WaterManagementStrategy::WatershedManagement => {
                self.environmental_flow = 0.85 + rand_simple() * 0.14;
                self.supply_reliability = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.environmental_flow == 0.0 {
            self.environmental_flow = (self.water_use_efficiency + self.supply_reliability) / 2.0 * (0.5 + rand_simple() * 0.4);
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
    fn test_deficit_irrigation() {
        let mut system = WaterManagementAgricultureSystem::new(WaterManagementStrategy::DeficitIrrigation);
        system.analyze_system().unwrap();
        assert!(system.water_use_efficiency > 0.7);
    }
}