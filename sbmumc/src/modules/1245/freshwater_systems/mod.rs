//! # SBMUMC Module 1245: Freshwater Systems
//!
//! Ecosystems and resources in lakes, rivers, and streams.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FreshwaterSystemType {
    Lotic,
    Lentic,
    Wetland,
    Groundwater,
    Springs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreshwaterSystemsFramework {
    pub framework_id: String,
    pub system_type: FreshwaterSystemType,
    pub water_quality_index: f64,
    pub biodiversity_score: f64,
    pub ecosystem_health: f64,
    pub human_dependency: f64,
}

impl FreshwaterSystemsFramework {
    pub fn new(system_type: FreshwaterSystemType) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            system_type,
            water_quality_index: 0.0,
            biodiversity_score: 0.0,
            ecosystem_health: 0.0,
            human_dependency: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.system_type {
            FreshwaterSystemType::Lotic => {
                self.water_quality_index = 0.75 + rand_simple() * 0.22;
                self.ecosystem_health = 0.80 + rand_simple() * 0.18;
                self.human_dependency = 0.70 + rand_simple() * 0.25;
            },
            FreshwaterSystemType::Lentic => {
                self.water_quality_index = 0.70 + rand_simple() * 0.25;
                self.biodiversity_score = 0.75 + rand_simple() * 0.22;
            },
            FreshwaterSystemType::Wetland => {
                self.biodiversity_score = 0.85 + rand_simple() * 0.14;
                self.ecosystem_health = 0.80 + rand_simple() * 0.18;
                self.human_dependency = 0.65 + rand_simple() * 0.30;
            },
            FreshwaterSystemType::Groundwater => {
                self.water_quality_index = 0.80 + rand_simple() * 0.18;
                self.human_dependency = 0.85 + rand_simple() * 0.14;
            },
            FreshwaterSystemType::Springs => {
                self.water_quality_index = 0.85 + rand_simple() * 0.14;
                self.biodiversity_score = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.ecosystem_health == 0.0 {
            self.ecosystem_health = (self.water_quality_index + self.biodiversity_score) / 2.0 * (0.7 + rand_simple() * 0.3);
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
    fn test_wetland_system() {
        let mut framework = FreshwaterSystemsFramework::new(FreshwaterSystemType::Wetland);
        framework.analyze_framework().unwrap();
        assert!(framework.biodiversity_score > 0.6);
    }
}