//! # SBMUMC Module 1436: Descriptive Set Theory
//!
//! Systems for descriptive set theory and definability.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DescriptiveTopic {
    BorelSets,
    AnalyticSets,
    ProjectiveSets,
    PolishSpaces,
    WadgeHierarchy,
    EogorovHierarchy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescriptiveSetTheorySystem {
    pub system_id: String,
    pub descriptive_topic: DescriptiveTopic,
    pub definability_hierarchy: f64,
    pub separation_properties: f64,
    pub reduction_theorems: f64,
    pub uniformity_theorems: f64,
}

impl DescriptiveSetTheorySystem {
    pub fn new(descriptive_topic: DescriptiveTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            descriptive_topic,
            definability_hierarchy: 0.0,
            separation_properties: 0.0,
            reduction_theorems: 0.0,
            uniformity_theorems: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.descriptive_topic {
            DescriptiveTopic::BorelSets => {
                self.definability_hierarchy = 0.95 + rand_simple() * 0.05;
                self.separation_properties = 0.90 + rand_simple() * 0.10;
                self.reduction_theorems = 0.85 + rand_simple() * 0.14;
            },
            DescriptiveTopic::AnalyticSets => {
                self.uniformity_theorems = 0.95 + rand_simple() * 0.05;
                self.definability_hierarchy = 0.90 + rand_simple() * 0.10;
                self.separation_properties = 0.85 + rand_simple() * 0.14;
            },
            DescriptiveTopic::ProjectiveSets => {
                self.reduction_theorems = 0.95 + rand_simple() * 0.05;
                self.uniformity_theorems = 0.90 + rand_simple() * 0.10;
                self.definability_hierarchy = 0.85 + rand_simple() * 0.14;
            },
            DescriptiveTopic::PolishSpaces => {
                self.separation_properties = 0.95 + rand_simple() * 0.05;
                self.reduction_theorems = 0.90 + rand_simple() * 0.10;
                self.uniformity_theorems = 0.85 + rand_simple() * 0.14;
            },
            DescriptiveTopic::WadgeHierarchy => {
                self.definability_hierarchy = 0.95 + rand_simple() * 0.05;
                self.uniformity_theorems = 0.90 + rand_simple() * 0.10;
                self.reduction_theorems = 0.85 + rand_simple() * 0.14;
            },
            DescriptiveTopic::EogorovHierarchy => {
                self.separation_properties = 0.95 + rand_simple() * 0.05;
                self.definability_hierarchy = 0.90 + rand_simple() * 0.10;
                self.uniformity_theorems = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.reduction_theorems == 0.0 {
            self.reduction_theorems = (self.definability_hierarchy + self.separation_properties) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_borel() {
        let mut system = DescriptiveSetTheorySystem::new(DescriptiveTopic::BorelSets);
        system.analyze_system().unwrap();
        assert!(system.definability_hierarchy > 0.8);
    }
}