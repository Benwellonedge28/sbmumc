//! # SBMUMC Module 1222: Food Waste Reduction
//!
//! Strategies to minimize food loss throughout the supply chain.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WasteReductionStrategy {
    ImprovedStorage,
    SupplyChainOptimization,
    ConsumerEducation,
    SurplusRedistribution,
    ProcessingOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodWasteReductionSystem {
    pub system_id: String,
    pub waste_strategy: WasteReductionStrategy,
    pub waste_reduction_potential: f64,
    pub economic_value: f64,
    pub environmental_impact: f64,
    pub implementation_cost: f64,
}

impl FoodWasteReductionSystem {
    pub fn new(waste_strategy: WasteReductionStrategy) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            waste_strategy,
            waste_reduction_potential: 0.0,
            economic_value: 0.0,
            environmental_impact: 0.0,
            implementation_cost: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.waste_strategy {
            WasteReductionStrategy::ImprovedStorage => {
                self.waste_reduction_potential = 0.80 + rand_simple() * 0.18;
                self.implementation_cost = 0.60 + rand_simple() * 0.30;
            },
            WasteReductionStrategy::SupplyChainOptimization => {
                self.waste_reduction_potential = 0.70 + rand_simple() * 0.25;
                self.economic_value = 0.80 + rand_simple() * 0.18;
            },
            WasteReductionStrategy::ConsumerEducation => {
                self.waste_reduction_potential = 0.50 + rand_simple() * 0.35;
                self.implementation_cost = 0.20 + rand_simple() * 0.25;
                self.economic_value = 0.60 + rand_simple() * 0.35;
            },
            WasteReductionStrategy::SurplusRedistribution => {
                self.waste_reduction_potential = 0.40 + rand_simple() * 0.35;
                self.economic_value = 0.65 + rand_simple() * 0.30;
                self.environmental_impact = 0.70 + rand_simple() * 0.25;
            },
            WasteReductionStrategy::ProcessingOptimization => {
                self.waste_reduction_potential = 0.75 + rand_simple() * 0.22;
                self.economic_value = 0.70 + rand_simple() * 0.25;
                self.implementation_cost = 0.50 + rand_simple() * 0.35;
            },
        }

        if self.environmental_impact == 0.0 {
            self.environmental_impact = self.waste_reduction_potential * (0.6 + rand_simple() * 0.3);
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
    fn test_storage_strategy() {
        let mut system = FoodWasteReductionSystem::new(WasteReductionStrategy::ImprovedStorage);
        system.analyze_system().unwrap();
        assert!(system.waste_reduction_potential > 0.6);
    }
}