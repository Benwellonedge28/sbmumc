//! # SBMUMC Module 1342: Heritage Architecture
//!
//! Systems for managing heritage buildings and sites.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeritageCategory {
    WorldHeritage,
    NationalHeritage,
    LocalHeritage,
    IndustrialHeritage,
    CulturalLandscape,
    Archaeological,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeritageArchitectureSystem {
    pub system_id: String,
    pub heritage_category: HeritageCategory,
    pub heritage_significance: f64,
    pub preservation_state: f64,
    pub visitor_management: f64,
    pub conservation_compliance: f64,
}

impl HeritageArchitectureSystem {
    pub fn new(heritage_category: HeritageCategory) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            heritage_category,
            heritage_significance: 0.0,
            preservation_state: 0.0,
            visitor_management: 0.0,
            conservation_compliance: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.heritage_category {
            HeritageCategory::WorldHeritage => {
                self.heritage_significance = 0.95 + rand_simple() * 0.05;
                self.conservation_compliance = 0.95 + rand_simple() * 0.05;
                self.preservation_state = 0.90 + rand_simple() * 0.10;
            },
            HeritageCategory::NationalHeritage => {
                self.heritage_significance = 0.90 + rand_simple() * 0.10;
                self.conservation_compliance = 0.90 + rand_simple() * 0.10;
                self.visitor_management = 0.85 + rand_simple() * 0.14;
            },
            HeritageCategory::LocalHeritage => {
                self.heritage_significance = 0.85 + rand_simple() * 0.14;
                self.preservation_state = 0.80 + rand_simple() * 0.18;
                self.visitor_management = 0.80 + rand_simple() * 0.18;
            },
            HeritageCategory::IndustrialHeritage => {
                self.heritage_significance = 0.85 + rand_simple() * 0.14;
                self.preservation_state = 0.75 + rand_simple() * 0.22;
                self.conservation_compliance = 0.80 + rand_simple() * 0.18;
            },
            HeritageCategory::CulturalLandscape => {
                self.visitor_management = 0.90 + rand_simple() * 0.10;
                self.heritage_significance = 0.85 + rand_simple() * 0.14;
                self.preservation_state = 0.85 + rand_simple() * 0.14;
            },
            HeritageCategory::Archaeological => {
                self.heritage_significance = 0.90 + rand_simple() * 0.10;
                self.preservation_state = 0.85 + rand_simple() * 0.14;
                self.conservation_compliance = 0.90 + rand_simple() * 0.10;
            },
        }

        if self.visitor_management == 0.0 {
            self.visitor_management = (self.heritage_significance + self.preservation_state) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_world_heritage() {
        let mut system = HeritageArchitectureSystem::new(HeritageCategory::WorldHeritage);
        system.analyze_system().unwrap();
        assert!(system.heritage_significance > 0.8);
    }
}