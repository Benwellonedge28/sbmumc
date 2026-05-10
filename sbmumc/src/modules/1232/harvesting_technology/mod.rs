//! # SBMUMC Module 1232: Harvesting Technology
//!
//! Equipment and methods for crop harvesting operations.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HarvestingTechnology {
    Manual,
    Mechanical,
    Automated,
    Selective,
    Precision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestingTechnologySystem {
    pub system_id: String,
    pub harvesting_technology: HarvestingTechnology,
    pub harvest_efficiency: f64,
    pub crop_quality: f64,
    pub labor_requirement: f64,
    pub loss_reduction: f64,
}

impl HarvestingTechnologySystem {
    pub fn new(harvesting_technology: HarvestingTechnology) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            harvesting_technology,
            harvest_efficiency: 0.0,
            crop_quality: 0.0,
            labor_requirement: 0.0,
            loss_reduction: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.harvesting_technology {
            HarvestingTechnology::Manual => {
                self.crop_quality = 0.90 + rand_simple() * 0.10;
                self.labor_requirement = 0.80 + rand_simple() * 0.18;
                self.loss_reduction = 0.50 + rand_simple() * 0.35;
            },
            HarvestingTechnology::Mechanical => {
                self.harvest_efficiency = 0.85 + rand_simple() * 0.14;
                self.labor_requirement = 0.30 + rand_simple() * 0.30;
            },
            HarvestingTechnology::Automated => {
                self.harvest_efficiency = 0.90 + rand_simple() * 0.10;
                self.labor_requirement = 0.10 + rand_simple() * 0.15;
                self.loss_reduction = 0.70 + rand_simple() * 0.25;
            },
            HarvestingTechnology::Selective => {
                self.crop_quality = 0.85 + rand_simple() * 0.14;
                self.loss_reduction = 0.80 + rand_simple() * 0.18;
            },
            HarvestingTechnology::Precision => {
                self.crop_quality = 0.90 + rand_simple() * 0.10;
                self.harvest_efficiency = 0.85 + rand_simple() * 0.14;
                self.loss_reduction = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.harvest_efficiency == 0.0 {
            self.harvest_efficiency = (self.crop_quality + self.loss_reduction) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_precision_harvesting() {
        let mut system = HarvestingTechnologySystem::new(HarvestingTechnology::Precision);
        system.analyze_system().unwrap();
        assert!(system.loss_reduction > 0.6);
    }
}