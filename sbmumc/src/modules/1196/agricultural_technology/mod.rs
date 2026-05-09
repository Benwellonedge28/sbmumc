//! # SBMUMC Module 1196: Agricultural Technology
//!
//! Innovation in farming tools, equipment, and systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgriTechCategory {
    Mechanization,
    Biotechnology,
    InformationSystems,
    Automation,
    Nanotechnology,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculturalTechnologySystem {
    pub system_id: String,
    pub tech_category: AgriTechCategory,
    pub efficiency_gain: f64,
    pub productivity_boost: f64,
    pub skill_requirements: f64,
    pub adoption_barrier: f64,
}

impl AgriculturalTechnologySystem {
    pub fn new(tech_category: AgriTechCategory) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            tech_category,
            efficiency_gain: 0.0,
            productivity_boost: 0.0,
            skill_requirements: 0.0,
            adoption_barrier: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.tech_category {
            AgriTechCategory::Mechanization => {
                self.efficiency_gain = 0.85 + rand_simple() * 0.14;
                self.productivity_boost = 0.40 + rand_simple() * 0.25;
                self.adoption_barrier = 0.30 + rand_simple() * 0.25;
            },
            AgriTechCategory::Biotechnology => {
                self.productivity_boost = 0.50 + rand_simple() * 0.30;
                self.skill_requirements = 0.60 + rand_simple() * 0.30;
                self.adoption_barrier = 0.50 + rand_simple() * 0.30;
            },
            AgriTechCategory::InformationSystems => {
                self.efficiency_gain = 0.70 + rand_simple() * 0.25;
                self.skill_requirements = 0.75 + rand_simple() * 0.22;
            },
            AgriTechCategory::Automation => {
                self.efficiency_gain = 0.90 + rand_simple() * 0.10;
                self.productivity_boost = 0.45 + rand_simple() * 0.25;
                self.skill_requirements = 0.80 + rand_simple() * 0.18;
            },
            AgriTechCategory::Nanotechnology => {
                self.productivity_boost = 0.35 + rand_simple() * 0.30;
                self.skill_requirements = 0.85 + rand_simple() * 0.14;
                self.adoption_barrier = 0.70 + rand_simple() * 0.20;
            },
        }

        if self.skill_requirements == 0.0 {
            self.skill_requirements = 0.40 + rand_simple() * 0.40;
        }
        if self.adoption_barrier == 0.0 {
            self.adoption_barrier = 0.35 + rand_simple() * 0.35;
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
    fn test_automation_tech() {
        let mut system = AgriculturalTechnologySystem::new(AgriTechCategory::Automation);
        system.analyze_system().unwrap();
        assert!(system.efficiency_gain > 0.7);
    }
}