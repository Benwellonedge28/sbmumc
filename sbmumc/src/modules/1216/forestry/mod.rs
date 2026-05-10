//! # SBMUMC Module 1216: Forestry
//!
//! Management and cultivation of forest ecosystems and timber.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForestryPractice {
    Silviculture,
    Agroforestry,
    Reforestation,
    SelectiveHarvesting,
    Conservation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForestrySystem {
    pub system_id: String,
    pub forestry_practice: ForestryPractice,
    pub timber_yield: f64,
    pub ecosystem_services: f64,
    pub carbon_sequestration: f64,
    pub biodiversity_value: f64,
}

impl ForestrySystem {
    pub fn new(forestry_practice: ForestryPractice) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            forestry_practice,
            timber_yield: 0.0,
            ecosystem_services: 0.0,
            carbon_sequestration: 0.0,
            biodiversity_value: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.forestry_practice {
            ForestryPractice::Silviculture => {
                self.timber_yield = 0.85 + rand_simple() * 0.14;
                self.ecosystem_services = 0.70 + rand_simple() * 0.25;
            },
            ForestryPractice::Agroforestry => {
                self.timber_yield = 0.65 + rand_simple() * 0.30;
                self.ecosystem_services = 0.85 + rand_simple() * 0.14;
                self.biodiversity_value = 0.80 + rand_simple() * 0.18;
            },
            ForestryPractice::Reforestation => {
                self.carbon_sequestration = 0.90 + rand_simple() * 0.10;
                self.ecosystem_services = 0.80 + rand_simple() * 0.18;
            },
            ForestryPractice::SelectiveHarvesting => {
                self.timber_yield = 0.70 + rand_simple() * 0.25;
                self.biodiversity_value = 0.80 + rand_simple() * 0.18;
                self.ecosystem_services = 0.75 + rand_simple() * 0.22;
            },
            ForestryPractice::Conservation => {
                self.carbon_sequestration = 0.85 + rand_simple() * 0.14;
                self.biodiversity_value = 0.90 + rand_simple() * 0.10;
                self.ecosystem_services = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.carbon_sequestration == 0.0 {
            self.carbon_sequestration = (self.ecosystem_services + self.biodiversity_value) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_reforestation() {
        let mut system = ForestrySystem::new(ForestryPractice::Reforestation);
        system.analyze_system().unwrap();
        assert!(system.carbon_sequestration > 0.7);
    }
}