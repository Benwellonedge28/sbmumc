//! # SBMUMC Module 1227: Crop Rotation
//!
//! Sequential planting of different crops to improve soil health.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RotationPattern {
    TwoYear,
    ThreeYear,
    FourYear,
    Diversified,
    Permaculture,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropRotationSystem {
    pub system_id: String,
    pub rotation_pattern: RotationPattern,
    pub soil_health_improvement: f64,
    pub pest_suppression: f64,
    pub nutrient_efficiency: f64,
    pub yield_stability: f64,
}

impl CropRotationSystem {
    pub fn new(rotation_pattern: RotationPattern) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            rotation_pattern,
            soil_health_improvement: 0.0,
            pest_suppression: 0.0,
            nutrient_efficiency: 0.0,
            yield_stability: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.rotation_pattern {
            RotationPattern::TwoYear => {
                self.soil_health_improvement = 0.60 + rand_simple() * 0.30;
                self.nutrient_efficiency = 0.65 + rand_simple() * 0.30;
            },
            RotationPattern::ThreeYear => {
                self.soil_health_improvement = 0.75 + rand_simple() * 0.22;
                self.pest_suppression = 0.70 + rand_simple() * 0.25;
                self.yield_stability = 0.75 + rand_simple() * 0.22;
            },
            RotationPattern::FourYear => {
                self.soil_health_improvement = 0.85 + rand_simple() * 0.14;
                self.pest_suppression = 0.80 + rand_simple() * 0.18;
                self.nutrient_efficiency = 0.80 + rand_simple() * 0.18;
            },
            RotationPattern::Diversified => {
                self.soil_health_improvement = 0.85 + rand_simple() * 0.14;
                self.pest_suppression = 0.85 + rand_simple() * 0.14;
                self.yield_stability = 0.80 + rand_simple() * 0.18;
            },
            RotationPattern::Permaculture => {
                self.soil_health_improvement = 0.90 + rand_simple() * 0.10;
                self.nutrient_efficiency = 0.85 + rand_simple() * 0.14;
                self.yield_stability = 0.75 + rand_simple() * 0.22;
            },
        }

        if self.nutrient_efficiency == 0.0 {
            self.nutrient_efficiency = (self.soil_health_improvement + self.yield_stability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_diversified_rotation() {
        let mut system = CropRotationSystem::new(RotationPattern::Diversified);
        system.analyze_system().unwrap();
        assert!(system.pest_suppression > 0.6);
    }
}