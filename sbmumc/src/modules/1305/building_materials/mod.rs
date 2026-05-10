//! # SBMUMC Module 1305: Building Materials
//!
//! Systems for building material selection and evaluation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildingMaterial {
    Steel,
    Concrete,
    Timber,
    Glass,
    Bamboo,
    RammedEarth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingMaterialsSystem {
    pub system_id: String,
    pub building_material: BuildingMaterial,
    pub durability_rating: f64,
    pub environmental_impact: f64,
    pub cost_effectiveness: f64,
    pub performance_characteristics: f64,
}

impl BuildingMaterialsSystem {
    pub fn new(building_material: BuildingMaterial) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            building_material,
            durability_rating: 0.0,
            environmental_impact: 0.0,
            cost_effectiveness: 0.0,
            performance_characteristics: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.building_material {
            BuildingMaterial::Steel => {
                self.durability_rating = 0.95 + rand_simple() * 0.05;
                self.performance_characteristics = 0.90 + rand_simple() * 0.10;
                self.environmental_impact = 0.55 + rand_simple() * 0.40;
            },
            BuildingMaterial::Concrete => {
                self.durability_rating = 0.90 + rand_simple() * 0.10;
                self.cost_effectiveness = 0.85 + rand_simple() * 0.14;
                self.environmental_impact = 0.45 + rand_simple() * 0.40;
            },
            BuildingMaterial::Timber => {
                self.environmental_impact = 0.90 + rand_simple() * 0.10;
                self.performance_characteristics = 0.80 + rand_simple() * 0.18;
                self.cost_effectiveness = 0.75 + rand_simple() * 0.22;
            },
            BuildingMaterial::Glass => {
                self.performance_characteristics = 0.85 + rand_simple() * 0.14;
                self.durability_rating = 0.80 + rand_simple() * 0.18;
                self.environmental_impact = 0.60 + rand_simple() * 0.35;
            },
            BuildingMaterial::Bamboo => {
                self.environmental_impact = 0.95 + rand_simple() * 0.05;
                self.cost_effectiveness = 0.90 + rand_simple() * 0.10;
                self.performance_characteristics = 0.75 + rand_simple() * 0.22;
            },
            BuildingMaterial::RammedEarth => {
                self.environmental_impact = 0.95 + rand_simple() * 0.05;
                self.cost_effectiveness = 0.85 + rand_simple() * 0.14;
                self.durability_rating = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.cost_effectiveness == 0.0 {
            self.cost_effectiveness = (self.durability_rating + self.performance_characteristics) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_bamboo() {
        let mut system = BuildingMaterialsSystem::new(BuildingMaterial::Bamboo);
        system.analyze_system().unwrap();
        assert!(system.environmental_impact > 0.8);
    }
}
