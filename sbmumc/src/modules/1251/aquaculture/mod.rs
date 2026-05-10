//! # SBMUMC Module 1251: Aquaculture
//!
//! Cultivation of aquatic organisms for food and other products.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AquacultureSystem {
    Finfish,
    Shellfish,
    Seaweed,
    Crustaceans,
    Integrated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquacultureFramework {
    pub framework_id: String,
    pub aquaculture_system: AquacultureSystem,
    pub production_efficiency: f64,
    pub environmental_footprint: f64,
    pub disease_management: f64,
    pub market_quality: f64,
}

impl AquacultureFramework {
    pub fn new(aquaculture_system: AquacultureSystem) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            aquaculture_system,
            production_efficiency: 0.0,
            environmental_footprint: 0.0,
            disease_management: 0.0,
            market_quality: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.aquaculture_system {
            AquacultureSystem::Finfish => {
                self.production_efficiency = 0.80 + rand_simple() * 0.18;
                self.disease_management = 0.70 + rand_simple() * 0.25;
                self.environmental_footprint = 0.40 + rand_simple() * 0.35;
            },
            AquacultureSystem::Shellfish => {
                self.production_efficiency = 0.70 + rand_simple() * 0.25;
                self.environmental_footprint = 0.80 + rand_simple() * 0.18;
                self.market_quality = 0.85 + rand_simple() * 0.14;
            },
            AquacultureSystem::Seaweed => {
                self.environmental_footprint = 0.90 + rand_simple() * 0.10;
                self.production_efficiency = 0.65 + rand_simple() * 0.30;
            },
            AquacultureSystem::Crustaceans => {
                self.production_efficiency = 0.75 + rand_simple() * 0.22;
                self.disease_management = 0.65 + rand_simple() * 0.30;
                self.market_quality = 0.80 + rand_simple() * 0.18;
            },
            AquacultureSystem::Integrated => {
                self.production_efficiency = 0.75 + rand_simple() * 0.22;
                self.environmental_footprint = 0.70 + rand_simple() * 0.25;
                self.disease_management = 0.60 + rand_simple() * 0.35;
            },
        }

        if self.market_quality == 0.0 {
            self.market_quality = (self.production_efficiency + self.disease_management) / 2.0 * (0.7 + rand_simple() * 0.3);
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
    fn test_shellfish_aquaculture() {
        let mut framework = AquacultureFramework::new(AquacultureSystem::Shellfish);
        framework.analyze_framework().unwrap();
        assert!(framework.environmental_footprint > 0.6);
    }
}