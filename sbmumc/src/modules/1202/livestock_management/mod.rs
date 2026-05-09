//! # SBMUMC Module 1202: Livestock Management
//!
//! Rearing and care of farm animals for production.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LivestockSystem {
    Intensive,
    FreeRange,
    Organic,
    GrassFed,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivestockManagementSystem {
    pub system_id: String,
    pub livestock_system: LivestockSystem,
    pub animal_welfare: f64,
    pub production_efficiency: f64,
    pub environmental_impact: f64,
    pub product_quality: f64,
}

impl LivestockManagementSystem {
    pub fn new(livestock_system: LivestockSystem) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            livestock_system,
            animal_welfare: 0.0,
            production_efficiency: 0.0,
            environmental_impact: 0.0,
            product_quality: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.livestock_system {
            LivestockSystem::Intensive => {
                self.production_efficiency = 0.90 + rand_simple() * 0.10;
                self.animal_welfare = 0.40 + rand_simple() * 0.35;
                self.environmental_impact = 0.30 + rand_simple() * 0.30;
            },
            LivestockSystem::FreeRange => {
                self.animal_welfare = 0.85 + rand_simple() * 0.14;
                self.product_quality = 0.80 + rand_simple() * 0.18;
                self.production_efficiency = 0.50 + rand_simple() * 0.35;
            },
            LivestockSystem::Organic => {
                self.animal_welfare = 0.90 + rand_simple() * 0.10;
                self.product_quality = 0.85 + rand_simple() * 0.14;
                self.environmental_impact = 0.70 + rand_simple() * 0.25;
            },
            LivestockSystem::GrassFed => {
                self.product_quality = 0.90 + rand_simple() * 0.10;
                self.environmental_impact = 0.80 + rand_simple() * 0.18;
                self.animal_welfare = 0.85 + rand_simple() * 0.14;
            },
            LivestockSystem::Mixed => {
                self.production_efficiency = 0.70 + rand_simple() * 0.25;
                self.animal_welfare = 0.60 + rand_simple() * 0.35;
                self.environmental_impact = 0.50 + rand_simple() * 0.35;
            },
        }

        if self.product_quality == 0.0 {
            self.product_quality = (self.animal_welfare + self.production_efficiency) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_grass_fed_system() {
        let mut system = LivestockManagementSystem::new(LivestockSystem::GrassFed);
        system.analyze_system().unwrap();
        assert!(system.product_quality > 0.7);
    }
}