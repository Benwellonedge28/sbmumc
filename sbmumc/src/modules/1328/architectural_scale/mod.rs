//! # SBMUMC Module 1328: Architectural Scale
//!
//! Systems for understanding architectural proportions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScaleApplication {
    HumanScale,
    UrbanScale,
    MonumentalScale,
    NeighborhoodScale,
    BuildingScale,
    DetailScale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitecturalScaleSystem {
    pub system_id: String,
    pub scale_application: ScaleApplication,
    pub proportional_alignment: f64,
    pub spatial_perception: f64,
    pub functional_appropriateness: f64,
    pub contextual_fit: f64,
}

impl ArchitecturalScaleSystem {
    pub fn new(scale_application: ScaleApplication) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            scale_application,
            proportional_alignment: 0.0,
            spatial_perception: 0.0,
            functional_appropriateness: 0.0,
            contextual_fit: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.scale_application {
            ScaleApplication::HumanScale => {
                self.spatial_perception = 0.95 + rand_simple() * 0.05;
                self.functional_appropriateness = 0.90 + rand_simple() * 0.10;
                self.proportional_alignment = 0.85 + rand_simple() * 0.14;
            },
            ScaleApplication::UrbanScale => {
                self.contextual_fit = 0.95 + rand_simple() * 0.05;
                self.proportional_alignment = 0.90 + rand_simple() * 0.10;
                self.functional_appropriateness = 0.85 + rand_simple() * 0.14;
            },
            ScaleApplication::MonumentalScale => {
                self.proportional_alignment = 0.95 + rand_simple() * 0.05;
                self.contextual_fit = 0.90 + rand_simple() * 0.10;
                self.spatial_perception = 0.85 + rand_simple() * 0.14;
            },
            ScaleApplication::NeighborhoodScale => {
                self.contextual_fit = 0.90 + rand_simple() * 0.10;
                self.functional_appropriateness = 0.85 + rand_simple() * 0.14;
                self.spatial_perception = 0.80 + rand_simple() * 0.18;
            },
            ScaleApplication::BuildingScale => {
                self.proportional_alignment = 0.90 + rand_simple() * 0.10;
                self.functional_appropriateness = 0.90 + rand_simple() * 0.10;
                self.spatial_perception = 0.85 + rand_simple() * 0.14;
            },
            ScaleApplication::DetailScale => {
                self.spatial_perception = 0.90 + rand_simple() * 0.10;
                self.functional_appropriateness = 0.85 + rand_simple() * 0.14;
                self.proportional_alignment = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.contextual_fit == 0.0 {
            self.contextual_fit = (self.proportional_alignment + self.spatial_perception) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_human_scale() {
        let mut system = ArchitecturalScaleSystem::new(ScaleApplication::HumanScale);
        system.analyze_system().unwrap();
        assert!(system.spatial_perception > 0.8);
    }
}
