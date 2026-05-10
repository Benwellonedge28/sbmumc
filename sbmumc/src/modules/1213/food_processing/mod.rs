//! # SBMUMC Module 1213: Food Processing
//!
//! Transformation of raw agricultural products into consumer goods.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingTechnology {
    Thermal,
    Mechanical,
    Chemical,
    Fermentation,
    FreezeDrying,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodProcessingSystem {
    pub system_id: String,
    pub processing_technology: ProcessingTechnology,
    pub shelf_life_extension: f64,
    pub nutritional_preservation: f64,
    pub safety_enhancement: f64,
    pub sensory_quality: f64,
}

impl FoodProcessingSystem {
    pub fn new(processing_technology: ProcessingTechnology) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            processing_technology,
            shelf_life_extension: 0.0,
            nutritional_preservation: 0.0,
            safety_enhancement: 0.0,
            sensory_quality: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.processing_technology {
            ProcessingTechnology::Thermal => {
                self.shelf_life_extension = 0.90 + rand_simple() * 0.10;
                self.safety_enhancement = 0.85 + rand_simple() * 0.14;
                self.nutritional_preservation = 0.60 + rand_simple() * 0.30;
            },
            ProcessingTechnology::Mechanical => {
                self.sensory_quality = 0.80 + rand_simple() * 0.18;
                self.nutritional_preservation = 0.85 + rand_simple() * 0.14;
            },
            ProcessingTechnology::Chemical => {
                self.shelf_life_extension = 0.85 + rand_simple() * 0.14;
                self.safety_enhancement = 0.75 + rand_simple() * 0.22;
            },
            ProcessingTechnology::Fermentation => {
                self.nutritional_preservation = 0.90 + rand_simple() * 0.10;
                self.sensory_quality = 0.85 + rand_simple() * 0.14;
                self.safety_enhancement = 0.80 + rand_simple() * 0.18;
            },
            ProcessingTechnology::FreezeDrying => {
                self.nutritional_preservation = 0.95 + rand_simple() * 0.05;
                self.shelf_life_extension = 0.90 + rand_simple() * 0.10;
                self.sensory_quality = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.sensory_quality == 0.0 {
            self.sensory_quality = (self.shelf_life_extension + self.nutritional_preservation) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_freeze_drying() {
        let mut system = FoodProcessingSystem::new(ProcessingTechnology::FreezeDrying);
        system.analyze_system().unwrap();
        assert!(system.nutritional_preservation > 0.8);
    }
}