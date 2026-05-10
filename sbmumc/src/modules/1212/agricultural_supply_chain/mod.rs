//! # SBMUMC Module 1212: Agricultural Supply Chain
//!
//! Movement of agricultural products from farm to consumer.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupplyChainStage {
    Production,
    Processing,
    Storage,
    Distribution,
    Retail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalSupplyChainSystem {
    pub system_id: String,
    pub chain_stage: SupplyChainStage,
    pub efficiency_score: f64,
    pub quality_maintenance: f64,
    pub waste_reduction: f64,
    pub traceability: f64,
}

impl AgriculturalSupplyChainSystem {
    pub fn new(chain_stage: SupplyChainStage) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            chain_stage,
            efficiency_score: 0.0,
            quality_maintenance: 0.0,
            waste_reduction: 0.0,
            traceability: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.chain_stage {
            SupplyChainStage::Production => {
                self.efficiency_score = 0.75 + rand_simple() * 0.22;
                self.quality_maintenance = 0.80 + rand_simple() * 0.18;
            },
            SupplyChainStage::Processing => {
                self.efficiency_score = 0.80 + rand_simple() * 0.18;
                self.quality_maintenance = 0.75 + rand_simple() * 0.22;
                self.waste_reduction = 0.70 + rand_simple() * 0.25;
            },
            SupplyChainStage::Storage => {
                self.quality_maintenance = 0.85 + rand_simple() * 0.14;
                self.waste_reduction = 0.75 + rand_simple() * 0.22;
            },
            SupplyChainStage::Distribution => {
                self.efficiency_score = 0.85 + rand_simple() * 0.14;
                self.traceability = 0.70 + rand_simple() * 0.25;
            },
            SupplyChainStage::Retail => {
                self.efficiency_score = 0.70 + rand_simple() * 0.25;
                self.quality_maintenance = 0.65 + rand_simple() * 0.30;
                self.traceability = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.waste_reduction == 0.0 {
            self.waste_reduction = (self.efficiency_score + self.quality_maintenance) / 2.0 * (0.6 + rand_simple() * 0.3);
        }
        if self.traceability == 0.0 {
            self.traceability = 0.55 + rand_simple() * 0.40;
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
    fn test_storage_chain() {
        let mut system = AgriculturalSupplyChainSystem::new(SupplyChainStage::Storage);
        system.analyze_system().unwrap();
        assert!(system.quality_maintenance > 0.6);
    }
}