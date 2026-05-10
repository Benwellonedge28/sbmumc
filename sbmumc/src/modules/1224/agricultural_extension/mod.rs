//! # SBMUMC Module 1224: Agricultural Extension
//!
//! Transfer of agricultural knowledge to farmers and rural communities.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtensionApproach {
    DirectAdvisory,
    FarmerFieldSchools,
    TechnologyTransfer,
    Participatory,
    Digital,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalExtensionSystem {
    pub system_id: String,
    pub extension_approach: ExtensionApproach,
    pub reach_effectiveness: f64,
    pub adoption_impact: f64,
    pub knowledge_retention: f64,
    pub scalability_index: f64,
}

impl AgriculturalExtensionSystem {
    pub fn new(extension_approach: ExtensionApproach) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            extension_approach,
            reach_effectiveness: 0.0,
            adoption_impact: 0.0,
            knowledge_retention: 0.0,
            scalability_index: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.extension_approach {
            ExtensionApproach::DirectAdvisory => {
                self.reach_effectiveness = 0.70 + rand_simple() * 0.25;
                self.adoption_impact = 0.80 + rand_simple() * 0.18;
                self.knowledge_retention = 0.75 + rand_simple() * 0.22;
            },
            ExtensionApproach::FarmerFieldSchools => {
                self.knowledge_retention = 0.90 + rand_simple() * 0.10;
                self.adoption_impact = 0.85 + rand_simple() * 0.14;
                self.reach_effectiveness = 0.50 + rand_simple() * 0.35;
            },
            ExtensionApproach::TechnologyTransfer => {
                self.reach_effectiveness = 0.80 + rand_simple() * 0.18;
                self.adoption_impact = 0.70 + rand_simple() * 0.25;
                self.scalability_index = 0.80 + rand_simple() * 0.18;
            },
            ExtensionApproach::Participatory => {
                self.knowledge_retention = 0.85 + rand_simple() * 0.14;
                self.adoption_impact = 0.80 + rand_simple() * 0.18;
            },
            ExtensionApproach::Digital => {
                self.reach_effectiveness = 0.90 + rand_simple() * 0.10;
                self.scalability_index = 0.85 + rand_simple() * 0.14;
                self.adoption_impact = 0.60 + rand_simple() * 0.35;
            },
        }

        if self.scalability_index == 0.0 {
            self.scalability_index = (self.reach_effectiveness + self.adoption_impact) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_farmer_field_schools() {
        let mut system = AgriculturalExtensionSystem::new(ExtensionApproach::FarmerFieldSchools);
        system.analyze_system().unwrap();
        assert!(system.knowledge_retention > 0.7);
    }
}