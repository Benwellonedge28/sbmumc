//! # SBMUMC Module 1207: Vertical Farming
//!
//! Multi-layer controlled environment agriculture.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerticalFarmingTechnology {
    Hydroponics,
    Aeroponics,
    Aquaponics,
    LEDLighting,
    ClimateControl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerticalFarmingSystem {
    pub system_id: String,
    pub farming_technology: VerticalFarmingTechnology,
    pub yield_per_area: f64,
    pub resource_efficiency: f64,
    pub energy_consumption: f64,
    pub product_quality: f64,
}

impl VerticalFarmingSystem {
    pub fn new(farming_technology: VerticalFarmingTechnology) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            farming_technology,
            yield_per_area: 0.0,
            resource_efficiency: 0.0,
            energy_consumption: 0.0,
            product_quality: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.farming_technology {
            VerticalFarmingTechnology::Hydroponics => {
                self.yield_per_area = 0.90 + rand_simple() * 0.10;
                self.resource_efficiency = 0.85 + rand_simple() * 0.14;
            },
            VerticalFarmingTechnology::Aeroponics => {
                self.yield_per_area = 0.85 + rand_simple() * 0.14;
                self.resource_efficiency = 0.90 + rand_simple() * 0.10;
                self.energy_consumption = 0.70 + rand_simple() * 0.25;
            },
            VerticalFarmingTechnology::Aquaponics => {
                self.resource_efficiency = 0.90 + rand_simple() * 0.10;
                self.product_quality = 0.85 + rand_simple() * 0.14;
            },
            VerticalFarmingTechnology::LEDLighting => {
                self.energy_consumption = 0.60 + rand_simple() * 0.30;
                self.product_quality = 0.80 + rand_simple() * 0.18;
            },
            VerticalFarmingTechnology::ClimateControl => {
                self.yield_per_area = 0.80 + rand_simple() * 0.18;
                self.product_quality = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.energy_consumption == 0.0 {
            self.energy_consumption = 0.40 + rand_simple() * 0.40;
        }
        if self.product_quality == 0.0 {
            self.product_quality = (self.yield_per_area + self.resource_efficiency) / 2.0 * (0.7 + rand_simple() * 0.3);
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
    fn test_aeroponics_farming() {
        let mut system = VerticalFarmingSystem::new(VerticalFarmingTechnology::Aeroponics);
        system.analyze_system().unwrap();
        assert!(system.resource_efficiency > 0.7);
    }
}