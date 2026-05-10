//! # SBMUMC Module 1208: Urban Agriculture
//!
//! Food production in urban and peri-urban environments.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UrbanFarmingType {
    CommunityGardens,
    RooftopFarms,
    VerticalBuildings,
    ContainerFarms,
    Institutional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanAgricultureSystem {
    pub system_id: String,
    pub farming_type: UrbanFarmingType,
    pub food_production: f64,
    pub community_engagement: f64,
    pub environmental_benefit: f64,
    pub economic_viability: f64,
}

impl UrbanAgricultureSystem {
    pub fn new(farming_type: UrbanFarmingType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            farming_type,
            food_production: 0.0,
            community_engagement: 0.0,
            environmental_benefit: 0.0,
            economic_viability: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.farming_type {
            UrbanFarmingType::CommunityGardens => {
                self.community_engagement = 0.90 + rand_simple() * 0.10;
                self.food_production = 0.40 + rand_simple() * 0.35;
                self.environmental_benefit = 0.65 + rand_simple() * 0.30;
            },
            UrbanFarmingType::RooftopFarms => {
                self.food_production = 0.70 + rand_simple() * 0.25;
                self.environmental_benefit = 0.80 + rand_simple() * 0.18;
                self.economic_viability = 0.55 + rand_simple() * 0.40;
            },
            UrbanFarmingType::VerticalBuildings => {
                self.food_production = 0.85 + rand_simple() * 0.14;
                self.economic_viability = 0.50 + rand_simple() * 0.40;
            },
            UrbanFarmingType::ContainerFarms => {
                self.food_production = 0.60 + rand_simple() * 0.30;
                self.economic_viability = 0.60 + rand_simple() * 0.35;
                self.environmental_benefit = 0.50 + rand_simple() * 0.35;
            },
            UrbanFarmingType::Institutional => {
                self.food_production = 0.75 + rand_simple() * 0.22;
                self.community_engagement = 0.60 + rand_simple() * 0.35;
                self.economic_viability = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.economic_viability == 0.0 {
            self.economic_viability = (self.food_production + self.community_engagement) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_rooftop_farms() {
        let mut system = UrbanAgricultureSystem::new(UrbanFarmingType::RooftopFarms);
        system.analyze_system().unwrap();
        assert!(system.environmental_benefit > 0.6);
    }
}