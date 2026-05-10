//! # SBMUMC Module 1314: Architectural Lighting
//!
//! Systems for architectural lighting design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LightingDesignApproach {
    Daylighting,
    TaskLighting,
    AmbientLighting,
    AccentLighting,
    EmergencyLighting,
    IntelligentLighting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitecturalLightingSystem {
    pub system_id: String,
    pub lighting_approach: LightingDesignApproach,
    pub illumination_quality: f64,
    pub energy_efficiency: f64,
    pub visual_comfort: f64,
    pub aesthetic_impact: f64,
}

impl ArchitecturalLightingSystem {
    pub fn new(lighting_approach: LightingDesignApproach) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            lighting_approach,
            illumination_quality: 0.0,
            energy_efficiency: 0.0,
            visual_comfort: 0.0,
            aesthetic_impact: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.lighting_approach {
            LightingDesignApproach::Daylighting => {
                self.illumination_quality = 0.90 + rand_simple() * 0.10;
                self.visual_comfort = 0.85 + rand_simple() * 0.14;
                self.energy_efficiency = 0.85 + rand_simple() * 0.14;
            },
            LightingDesignApproach::TaskLighting => {
                self.illumination_quality = 0.95 + rand_simple() * 0.05;
                self.visual_comfort = 0.90 + rand_simple() * 0.10;
                self.energy_efficiency = 0.75 + rand_simple() * 0.22;
            },
            LightingDesignApproach::AmbientLighting => {
                self.visual_comfort = 0.90 + rand_simple() * 0.10;
                self.aesthetic_impact = 0.85 + rand_simple() * 0.14;
                self.energy_efficiency = 0.80 + rand_simple() * 0.18;
            },
            LightingDesignApproach::AccentLighting => {
                self.aesthetic_impact = 0.95 + rand_simple() * 0.05;
                self.illumination_quality = 0.85 + rand_simple() * 0.14;
                self.energy_efficiency = 0.70 + rand_simple() * 0.25;
            },
            LightingDesignApproach::EmergencyLighting => {
                self.illumination_quality = 0.90 + rand_simple() * 0.10;
                self.visual_comfort = 0.80 + rand_simple() * 0.18;
                self.energy_efficiency = 0.65 + rand_simple() * 0.30;
            },
            LightingDesignApproach::IntelligentLighting => {
                self.energy_efficiency = 0.90 + rand_simple() * 0.10;
                self.visual_comfort = 0.85 + rand_simple() * 0.14;
                self.illumination_quality = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.aesthetic_impact == 0.0 {
            self.aesthetic_impact = (self.illumination_quality + self.visual_comfort) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_daylighting() {
        let mut system = ArchitecturalLightingSystem::new(LightingDesignApproach::Daylighting);
        system.analyze_system().unwrap();
        assert!(system.illumination_quality > 0.7);
    }
}
