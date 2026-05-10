//! # SBMUMC Module 1215: Horticulture
//!
//! Cultivation of fruits, vegetables, flowers, and ornamental plants.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HorticulturalSector {
    FruitProduction,
    VegetableProduction,
    Floriculture,
    Nursery,
    Landscape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorticultureSystem {
    pub system_id: String,
    pub horticultural_sector: HorticulturalSector,
    pub yield_quality: f64,
    pub market_value: f64,
    pub resource_intensity: f64,
    pub seasonal_flexibility: f64,
}

impl HorticultureSystem {
    pub fn new(horticultural_sector: HorticulturalSector) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            horticultural_sector,
            yield_quality: 0.0,
            market_value: 0.0,
            resource_intensity: 0.0,
            seasonal_flexibility: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.horticultural_sector {
            HorticulturalSector::FruitProduction => {
                self.yield_quality = 0.85 + rand_simple() * 0.14;
                self.market_value = 0.80 + rand_simple() * 0.18;
                self.resource_intensity = 0.70 + rand_simple() * 0.25;
            },
            HorticulturalSector::VegetableProduction => {
                self.yield_quality = 0.80 + rand_simple() * 0.18;
                self.market_value = 0.70 + rand_simple() * 0.25;
                self.seasonal_flexibility = 0.65 + rand_simple() * 0.30;
            },
            HorticulturalSector::Floriculture => {
                self.market_value = 0.90 + rand_simple() * 0.10;
                self.yield_quality = 0.85 + rand_simple() * 0.14;
                self.resource_intensity = 0.80 + rand_simple() * 0.18;
            },
            HorticulturalSector::Nursery => {
                self.market_value = 0.75 + rand_simple() * 0.22;
                self.seasonal_flexibility = 0.80 + rand_simple() * 0.18;
            },
            HorticulturalSector::Landscape => {
                self.market_value = 0.80 + rand_simple() * 0.18;
                self.resource_intensity = 0.55 + rand_simple() * 0.35;
            },
        }

        if self.resource_intensity == 0.0 {
            self.resource_intensity = 0.50 + rand_simple() * 0.40;
        }
        if self.seasonal_flexibility == 0.0 {
            self.seasonal_flexibility = (self.yield_quality + self.market_value) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_floriculture() {
        let mut system = HorticultureSystem::new(HorticulturalSector::Floriculture);
        system.analyze_system().unwrap();
        assert!(system.market_value > 0.7);
    }
}