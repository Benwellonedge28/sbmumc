//! # SBMUMC Module 1217: Fisheries
//!
//! Management and cultivation of fish and aquatic species.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FisheryType {
    Capture,
    Aquaculture,
    Mariculture,
    Freshwater,
    StockEnhancement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FisheriesSystem {
    pub system_id: String,
    pub fishery_type: FisheryType,
    pub harvest_efficiency: f64,
    pub sustainability_index: f64,
    pub resource_dependence: f64,
    pub product_quality: f64,
}

impl FisheriesSystem {
    pub fn new(fishery_type: FisheryType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            fishery_type,
            harvest_efficiency: 0.0,
            sustainability_index: 0.0,
            resource_dependence: 0.0,
            product_quality: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.fishery_type {
            FisheryType::Capture => {
                self.harvest_efficiency = 0.75 + rand_simple() * 0.22;
                self.sustainability_index = 0.50 + rand_simple() * 0.35;
            },
            FisheryType::Aquaculture => {
                self.harvest_efficiency = 0.85 + rand_simple() * 0.14;
                self.sustainability_index = 0.65 + rand_simple() * 0.30;
                self.resource_dependence = 0.70 + rand_simple() * 0.25;
            },
            FisheryType::Mariculture => {
                self.product_quality = 0.85 + rand_simple() * 0.14;
                self.sustainability_index = 0.60 + rand_simple() * 0.35;
                self.resource_dependence = 0.65 + rand_simple() * 0.30;
            },
            FisheryType::Freshwater => {
                self.harvest_efficiency = 0.70 + rand_simple() * 0.25;
                self.resource_dependence = 0.60 + rand_simple() * 0.35;
            },
            FisheryType::StockEnhancement => {
                self.sustainability_index = 0.70 + rand_simple() * 0.25;
                self.harvest_efficiency = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.product_quality == 0.0 {
            self.product_quality = (self.harvest_efficiency + self.sustainability_index) / 2.0 * (0.7 + rand_simple() * 0.3);
        }
        if self.resource_dependence == 0.0 {
            self.resource_dependence = 0.45 + rand_simple() * 0.45;
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
    fn test_aquaculture() {
        let mut system = FisheriesSystem::new(FisheryType::Aquaculture);
        system.analyze_system().unwrap();
        assert!(system.harvest_efficiency > 0.6);
    }
}