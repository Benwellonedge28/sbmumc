//! # SBMUMC Module 1233: Post Harvest Technology
//!
//! Processing and storage technologies after crop harvest.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostHarvestTechnology {
    Drying,
    Cooling,
    Packaging,
    QualityGrading,
    ValueAddition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostHarvestTechnologySystem {
    pub system_id: String,
    pub post_harvest_technology: PostHarvestTechnology,
    pub quality_preservation: f64,
    pub shelf_life_extension: f64,
    pub loss_minimization: f64,
    pub processing_efficiency: f64,
}

impl PostHarvestTechnologySystem {
    pub fn new(post_harvest_technology: PostHarvestTechnology) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            post_harvest_technology,
            quality_preservation: 0.0,
            shelf_life_extension: 0.0,
            loss_minimization: 0.0,
            processing_efficiency: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.post_harvest_technology {
            PostHarvestTechnology::Drying => {
                self.quality_preservation = 0.80 + rand_simple() * 0.18;
                self.shelf_life_extension = 0.85 + rand_simple() * 0.14;
            },
            PostHarvestTechnology::Cooling => {
                self.quality_preservation = 0.85 + rand_simple() * 0.14;
                self.loss_minimization = 0.80 + rand_simple() * 0.18;
            },
            PostHarvestTechnology::Packaging => {
                self.shelf_life_extension = 0.80 + rand_simple() * 0.18;
                self.processing_efficiency = 0.75 + rand_simple() * 0.22;
            },
            PostHarvestTechnology::QualityGrading => {
                self.quality_preservation = 0.75 + rand_simple() * 0.22;
                self.processing_efficiency = 0.80 + rand_simple() * 0.18;
            },
            PostHarvestTechnology::ValueAddition => {
                self.processing_efficiency = 0.85 + rand_simple() * 0.14;
                self.quality_preservation = 0.80 + rand_simple() * 0.18;
                self.loss_minimization = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.loss_minimization == 0.0 {
            self.loss_minimization = (self.quality_preservation + self.shelf_life_extension) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_cooling_technology() {
        let mut system = PostHarvestTechnologySystem::new(PostHarvestTechnology::Cooling);
        system.analyze_system().unwrap();
        assert!(system.quality_preservation > 0.6);
    }
}