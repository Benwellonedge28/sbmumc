//! # SBMUMC Module 1327: Recyclable Design
//!
//! Systems for design using recyclable materials.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecyclableMaterial {
    RecycledSteel,
    RecycledConcrete,
    RecycledPlastics,
    ReclaimedWood,
    RecycledGlass,
    BiodegradableComposites,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecyclableDesignSystem {
    pub system_id: String,
    pub recyclable_material: RecyclableMaterial,
    pub material_quality: f64,
    pub environmental_impact: f64,
    pub recyclability_rate: f64,
    pub structural_performance: f64,
}

impl RecyclableDesignSystem {
    pub fn new(recyclable_material: RecyclableMaterial) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            recyclable_material,
            material_quality: 0.0,
            environmental_impact: 0.0,
            recyclability_rate: 0.0,
            structural_performance: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.recyclable_material {
            RecyclableMaterial::RecycledSteel => {
                self.structural_performance = 0.95 + rand_simple() * 0.05;
                self.recyclability_rate = 0.90 + rand_simple() * 0.10;
                self.material_quality = 0.85 + rand_simple() * 0.14;
            },
            RecyclableMaterial::RecycledConcrete => {
                self.material_quality = 0.85 + rand_simple() * 0.14;
                self.recyclability_rate = 0.80 + rand_simple() * 0.18;
                self.structural_performance = 0.75 + rand_simple() * 0.22;
            },
            RecyclableMaterial::RecycledPlastics => {
                self.recyclability_rate = 0.95 + rand_simple() * 0.05;
                self.environmental_impact = 0.90 + rand_simple() * 0.10;
                self.material_quality = 0.70 + rand_simple() * 0.25;
            },
            RecyclableMaterial::ReclaimedWood => {
                self.material_quality = 0.90 + rand_simple() * 0.10;
                self.environmental_impact = 0.85 + rand_simple() * 0.14;
                self.structural_performance = 0.80 + rand_simple() * 0.18;
            },
            RecyclableMaterial::RecycledGlass => {
                self.recyclability_rate = 0.90 + rand_simple() * 0.10;
                self.material_quality = 0.85 + rand_simple() * 0.14;
                self.environmental_impact = 0.85 + rand_simple() * 0.14;
            },
            RecyclableMaterial::BiodegradableComposites => {
                self.environmental_impact = 0.95 + rand_simple() * 0.05;
                self.recyclability_rate = 0.90 + rand_simple() * 0.10;
                self.material_quality = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.environmental_impact == 0.0 {
            self.environmental_impact = (self.recyclability_rate + self.material_quality) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_recycled_steel() {
        let mut system = RecyclableDesignSystem::new(RecyclableMaterial::RecycledSteel);
        system.analyze_system().unwrap();
        assert!(system.structural_performance > 0.8);
    }
}
