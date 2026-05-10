//! # SBMUMC Module 1307: Biophilic Design
//!
//! Systems for integrating nature into architectural spaces.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiophilicElement {
    LivingWalls,
    WaterFeatures,
    NaturalLight,
    OrganicForms,
    NaturalMaterials,
    Planters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiophilicDesignSystem {
    pub system_id: String,
    pub biophilic_element: BiophilicElement,
    pub wellbeing_impact: f64,
    pub air_quality_improvement: f64,
    pub aesthetic_integration: f64,
    pub maintenance_requirement: f64,
}

impl BiophilicDesignSystem {
    pub fn new(biophilic_element: BiophilicElement) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            biophilic_element,
            wellbeing_impact: 0.0,
            air_quality_improvement: 0.0,
            aesthetic_integration: 0.0,
            maintenance_requirement: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.biophilic_element {
            BiophilicElement::LivingWalls => {
                self.wellbeing_impact = 0.90 + rand_simple() * 0.10;
                self.air_quality_improvement = 0.85 + rand_simple() * 0.14;
                self.maintenance_requirement = 0.75 + rand_simple() * 0.22;
            },
            BiophilicElement::WaterFeatures => {
                self.wellbeing_impact = 0.95 + rand_simple() * 0.05;
                self.aesthetic_integration = 0.90 + rand_simple() * 0.10;
                self.maintenance_requirement = 0.60 + rand_simple() * 0.35;
            },
            BiophilicElement::NaturalLight => {
                self.wellbeing_impact = 0.95 + rand_simple() * 0.05;
                self.air_quality_improvement = 0.70 + rand_simple() * 0.25;
                self.aesthetic_integration = 0.85 + rand_simple() * 0.14;
            },
            BiophilicElement::OrganicForms => {
                self.aesthetic_integration = 0.90 + rand_simple() * 0.10;
                self.wellbeing_impact = 0.85 + rand_simple() * 0.14;
                self.maintenance_requirement = 0.45 + rand_simple() * 0.40;
            },
            BiophilicElement::NaturalMaterials => {
                self.aesthetic_integration = 0.85 + rand_simple() * 0.14;
                self.wellbeing_impact = 0.80 + rand_simple() * 0.18;
                self.maintenance_requirement = 0.55 + rand_simple() * 0.40;
            },
            BiophilicElement::Planters => {
                self.air_quality_improvement = 0.80 + rand_simple() * 0.18;
                self.wellbeing_impact = 0.85 + rand_simple() * 0.14;
                self.maintenance_requirement = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.maintenance_requirement == 0.0 {
            self.maintenance_requirement = (1.0 - self.wellbeing_impact) * (0.5 + rand_simple() * 0.5);
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
    fn test_water_features() {
        let mut system = BiophilicDesignSystem::new(BiophilicElement::WaterFeatures);
        system.analyze_system().unwrap();
        assert!(system.wellbeing_impact > 0.8);
    }
}
