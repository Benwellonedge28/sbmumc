//! # SBMUMC Module 1198: Soil Management
//!
//! Practices for maintaining and improving soil health.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SoilManagementPractice {
    Composting,
    CoverCropping,
    ReducedTillage,
    Mulching,
    CropRotation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilManagementSystem {
    pub system_id: String,
    pub management_practice: SoilManagementPractice,
    pub organic_matter: f64,
    pub moisture_retention: f64,
    pub nutrient_cycling: f64,
    pub soil_structure: f64,
}

impl SoilManagementSystem {
    pub fn new(management_practice: SoilManagementPractice) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            management_practice,
            organic_matter: 0.0,
            moisture_retention: 0.0,
            nutrient_cycling: 0.0,
            soil_structure: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.management_practice {
            SoilManagementPractice::Composting => {
                self.organic_matter = 0.85 + rand_simple() * 0.14;
                self.nutrient_cycling = 0.80 + rand_simple() * 0.18;
            },
            SoilManagementPractice::CoverCropping => {
                self.organic_matter = 0.75 + rand_simple() * 0.22;
                self.moisture_retention = 0.80 + rand_simple() * 0.18;
                self.soil_structure = 0.75 + rand_simple() * 0.22;
            },
            SoilManagementPractice::ReducedTillage => {
                self.soil_structure = 0.85 + rand_simple() * 0.14;
                self.moisture_retention = 0.80 + rand_simple() * 0.18;
                self.organic_matter = 0.70 + rand_simple() * 0.25;
            },
            SoilManagementPractice::Mulching => {
                self.moisture_retention = 0.90 + rand_simple() * 0.10;
                self.soil_structure = 0.70 + rand_simple() * 0.25;
            },
            SoilManagementPractice::CropRotation => {
                self.nutrient_cycling = 0.85 + rand_simple() * 0.14;
                self.organic_matter = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.soil_structure == 0.0 {
            self.soil_structure = (self.organic_matter + self.moisture_retention) / 2.0 * (0.7 + rand_simple() * 0.3);
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
    fn test_cover_cropping() {
        let mut system = SoilManagementSystem::new(SoilManagementPractice::CoverCropping);
        system.analyze_system().unwrap();
        assert!(system.moisture_retention > 0.6);
    }
}