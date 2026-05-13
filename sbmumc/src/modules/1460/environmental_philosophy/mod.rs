//! # SBMUMC Module 1460: Environmental Philosophy
//!
//! Systems for environmental philosophy and ecological ethics.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentalFramework {
    Ecocentrism,
    Biocentrism,
    Anthropocentrism,
    DeepEcology,
    Ecofeminism,
    EnvironmentalVirtueEthics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalPhilosophySystem {
    pub system_id: String,
    pub environmental_framework: EnvironmentalFramework,
    pub intrinsic_value_nature: f64,
    pub ecological_justice: f64,
    pub sustainability_ethics: f64,
    pub nature_worth: f64,
}

impl EnvironmentalPhilosophySystem {
    pub fn new(environmental_framework: EnvironmentalFramework) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            environmental_framework,
            intrinsic_value_nature: 0.0,
            ecological_justice: 0.0,
            sustainability_ethics: 0.0,
            nature_worth: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.environmental_framework {
            EnvironmentalFramework::Ecocentrism => {
                self.intrinsic_value_nature = 0.95 + rand_simple() * 0.05;
                self.ecological_justice = 0.90 + rand_simple() * 0.10;
                self.sustainability_ethics = 0.85 + rand_simple() * 0.14;
            },
            EnvironmentalFramework::Biocentrism => {
                self.nature_worth = 0.95 + rand_simple() * 0.05;
                self.intrinsic_value_nature = 0.90 + rand_simple() * 0.10;
                self.ecological_justice = 0.85 + rand_simple() * 0.14;
            },
            EnvironmentalFramework::Anthropocentrism => {
                self.sustainability_ethics = 0.95 + rand_simple() * 0.05;
                self.nature_worth = 0.90 + rand_simple() * 0.10;
                self.intrinsic_value_nature = 0.85 + rand_simple() * 0.14;
            },
            EnvironmentalFramework::DeepEcology => {
                self.ecological_justice = 0.95 + rand_simple() * 0.05;
                self.sustainability_ethics = 0.90 + rand_simple() * 0.10;
                self.nature_worth = 0.85 + rand_simple() * 0.14;
            },
            EnvironmentalFramework::Ecofeminism => {
                self.intrinsic_value_nature = 0.95 + rand_simple() * 0.05;
                self.ecological_justice = 0.90 + rand_simple() * 0.10;
                self.nature_worth = 0.85 + rand_simple() * 0.14;
            },
            EnvironmentalFramework::EnvironmentalVirtueEthics => {
                self.sustainability_ethics = 0.95 + rand_simple() * 0.05;
                self.intrinsic_value_nature = 0.90 + rand_simple() * 0.10;
                self.ecological_justice = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.ecological_justice == 0.0 {
            self.ecological_justice = (self.intrinsic_value_nature + self.sustainability_ethics) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_ecocentrism() {
        let mut system = EnvironmentalPhilosophySystem::new(EnvironmentalFramework::Ecocentrism);
        system.analyze_system().unwrap();
        assert!(system.intrinsic_value_nature > 0.8);
    }
}