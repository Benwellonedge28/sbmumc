//! # SBMUMC Module 1302: Vernacular Architecture
//!
//! Systems for traditional and regional building practices.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VernacularStyle {
    Mediterranean,
    EastAsian,
    African,
    LatinAmerican,
    MiddleEastern,
    NorthernEuropean,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VernacularArchitectureSystem {
    pub system_id: String,
    pub vernacular_style: VernacularStyle,
    pub cultural_authenticity: f64,
    pub climate_adaptation: f64,
    pub material_availability: f64,
    pub craft_preservation: f64,
}

impl VernacularArchitectureSystem {
    pub fn new(vernacular_style: VernacularStyle) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            vernacular_style,
            cultural_authenticity: 0.0,
            climate_adaptation: 0.0,
            material_availability: 0.0,
            craft_preservation: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.vernacular_style {
            VernacularStyle::Mediterranean => {
                self.cultural_authenticity = 0.90 + rand_simple() * 0.10;
                self.climate_adaptation = 0.85 + rand_simple() * 0.14;
                self.material_availability = 0.80 + rand_simple() * 0.18;
            },
            VernacularStyle::EastAsian => {
                self.climate_adaptation = 0.95 + rand_simple() * 0.05;
                self.cultural_authenticity = 0.90 + rand_simple() * 0.10;
                self.craft_preservation = 0.85 + rand_simple() * 0.14;
            },
            VernacularStyle::African => {
                self.material_availability = 0.90 + rand_simple() * 0.10;
                self.cultural_authenticity = 0.85 + rand_simple() * 0.14;
                self.climate_adaptation = 0.80 + rand_simple() * 0.18;
            },
            VernacularStyle::LatinAmerican => {
                self.cultural_authenticity = 0.85 + rand_simple() * 0.14;
                self.craft_preservation = 0.90 + rand_simple() * 0.10;
                self.material_availability = 0.75 + rand_simple() * 0.22;
            },
            VernacularStyle::MiddleEastern => {
                self.climate_adaptation = 0.90 + rand_simple() * 0.10;
                self.cultural_authenticity = 0.85 + rand_simple() * 0.14;
                self.material_availability = 0.70 + rand_simple() * 0.25;
            },
            VernacularStyle::NorthernEuropean => {
                self.climate_adaptation = 0.85 + rand_simple() * 0.14;
                self.craft_preservation = 0.80 + rand_simple() * 0.18;
                self.material_availability = 0.90 + rand_simple() * 0.10;
            },
        }

        if self.craft_preservation == 0.0 {
            self.craft_preservation = (self.cultural_authenticity + self.material_availability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_east_asian() {
        let mut system = VernacularArchitectureSystem::new(VernacularStyle::EastAsian);
        system.analyze_system().unwrap();
        assert!(system.climate_adaptation > 0.8);
    }
}
