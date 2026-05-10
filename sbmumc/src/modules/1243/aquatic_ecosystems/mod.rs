//! # SBMUMC Module 1243: Aquatic Ecosystems
//!
//! Study of water-based ecological systems and their functions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AquaticEcosystemType {
    Marine,
    Freshwater,
    Brackish,
    Transitional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquaticEcosystemsFramework {
    pub framework_id: String,
    pub ecosystem_type: AquaticEcosystemType,
    pub biodiversity_index: f64,
    pub ecosystem_resilience: f64,
    pub service_capacity: f64,
    pub habitat_integrity: f64,
}

impl AquaticEcosystemsFramework {
    pub fn new(ecosystem_type: AquaticEcosystemType) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            ecosystem_type,
            biodiversity_index: 0.0,
            ecosystem_resilience: 0.0,
            service_capacity: 0.0,
            habitat_integrity: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.ecosystem_type {
            AquaticEcosystemType::Marine => {
                self.biodiversity_index = 0.85 + rand_simple() * 0.14;
                self.service_capacity = 0.80 + rand_simple() * 0.18;
                self.habitat_integrity = 0.70 + rand_simple() * 0.25;
            },
            AquaticEcosystemType::Freshwater => {
                self.biodiversity_index = 0.75 + rand_simple() * 0.22;
                self.ecosystem_resilience = 0.70 + rand_simple() * 0.25;
                self.habitat_integrity = 0.65 + rand_simple() * 0.30;
            },
            AquaticEcosystemType::Brackish => {
                self.biodiversity_index = 0.80 + rand_simple() * 0.18;
                self.ecosystem_resilience = 0.75 + rand_simple() * 0.22;
                self.service_capacity = 0.70 + rand_simple() * 0.25;
            },
            AquaticEcosystemType::Transitional => {
                self.biodiversity_index = 0.85 + rand_simple() * 0.14;
                self.service_capacity = 0.85 + rand_simple() * 0.14;
                self.ecosystem_resilience = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.ecosystem_resilience == 0.0 {
            self.ecosystem_resilience = (self.biodiversity_index + self.habitat_integrity) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_marine_ecosystem() {
        let mut framework = AquaticEcosystemsFramework::new(AquaticEcosystemType::Marine);
        framework.analyze_framework().unwrap();
        assert!(framework.biodiversity_index > 0.6);
    }
}