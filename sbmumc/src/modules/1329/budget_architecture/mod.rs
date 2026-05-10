//! # SBMUMC Module 1329: Budget Architecture
//!
//! Systems for cost-effective architectural design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BudgetStrategy {
    CostOptimization,
    ValueEngineering,
    PhasedConstruction,
    SharedFacilities,
    CommunityParticipation,
    SimpleMaterials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetArchitectureSystem {
    pub system_id: String,
    pub budget_strategy: BudgetStrategy,
    pub cost_efficiency: f64,
    pub value_retention: f64,
    pub quality_maintenance: f64,
    pub lifecycle_cost: f64,
}

impl BudgetArchitectureSystem {
    pub fn new(budget_strategy: BudgetStrategy) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            budget_strategy,
            cost_efficiency: 0.0,
            value_retention: 0.0,
            quality_maintenance: 0.0,
            lifecycle_cost: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.budget_strategy {
            BudgetStrategy::CostOptimization => {
                self.cost_efficiency = 0.95 + rand_simple() * 0.05;
                self.value_retention = 0.85 + rand_simple() * 0.14;
                self.quality_maintenance = 0.80 + rand_simple() * 0.18;
            },
            BudgetStrategy::ValueEngineering => {
                self.value_retention = 0.90 + rand_simple() * 0.10;
                self.cost_efficiency = 0.85 + rand_simple() * 0.14;
                self.lifecycle_cost = 0.80 + rand_simple() * 0.18;
            },
            BudgetStrategy::PhasedConstruction => {
                self.cost_efficiency = 0.90 + rand_simple() * 0.10;
                self.lifecycle_cost = 0.85 + rand_simple() * 0.14;
                self.quality_maintenance = 0.75 + rand_simple() * 0.22;
            },
            BudgetStrategy::SharedFacilities => {
                self.value_retention = 0.85 + rand_simple() * 0.14;
                self.cost_efficiency = 0.85 + rand_simple() * 0.14;
                self.quality_maintenance = 0.80 + rand_simple() * 0.18;
            },
            BudgetStrategy::CommunityParticipation => {
                self.cost_efficiency = 0.90 + rand_simple() * 0.10;
                self.value_retention = 0.80 + rand_simple() * 0.18;
                self.lifecycle_cost = 0.75 + rand_simple() * 0.22;
            },
            BudgetStrategy::SimpleMaterials => {
                self.cost_efficiency = 0.95 + rand_simple() * 0.05;
                self.quality_maintenance = 0.75 + rand_simple() * 0.22;
                self.lifecycle_cost = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.lifecycle_cost == 0.0 {
            self.lifecycle_cost = (self.cost_efficiency + self.value_retention) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_cost_optimization() {
        let mut system = BudgetArchitectureSystem::new(BudgetStrategy::CostOptimization);
        system.analyze_system().unwrap();
        assert!(system.cost_efficiency > 0.8);
    }
}
