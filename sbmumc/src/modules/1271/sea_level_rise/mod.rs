//! # SBMUMC Module 1271: Sea Level Rise
//!
//! Systems for monitoring and adapting to sea level rise.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeaLevelRiseImpact {
    CoastalErosion,
    Flooding,
    SaltwaterIntrusion,
    HabitatLoss,
    InfrastructureDamage,
    PopulationDisplacement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeaLevelRiseSystem {
    pub system_id: String,
    pub sea_level_impact: SeaLevelRiseImpact,
    pub rise_rate: f64,
    pub vulnerability_index: f64,
    pub adaptation_capacity: f64,
    pub mitigation_potential: f64,
}

impl SeaLevelRiseSystem {
    pub fn new(sea_level_impact: SeaLevelRiseImpact) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            sea_level_impact,
            rise_rate: 0.0,
            vulnerability_index: 0.0,
            adaptation_capacity: 0.0,
            mitigation_potential: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.sea_level_impact {
            SeaLevelRiseImpact::CoastalErosion => {
                self.vulnerability_index = 0.80 + rand_simple() * 0.18;
                self.mitigation_potential = 0.60 + rand_simple() * 0.35;
            },
            SeaLevelRiseImpact::Flooding => {
                self.vulnerability_index = 0.85 + rand_simple() * 0.14;
                self.adaptation_capacity = 0.65 + rand_simple() * 0.30;
            },
            SeaLevelRiseImpact::SaltwaterIntrusion => {
                self.vulnerability_index = 0.75 + rand_simple() * 0.22;
                self.mitigation_potential = 0.55 + rand_simple() * 0.40;
            },
            SeaLevelRiseImpact::HabitatLoss => {
                self.vulnerability_index = 0.90 + rand_simple() * 0.10;
                self.adaptation_capacity = 0.40 + rand_simple() * 0.35;
            },
            SeaLevelRiseImpact::InfrastructureDamage => {
                self.mitigation_potential = 0.70 + rand_simple() * 0.25;
                self.adaptation_capacity = 0.60 + rand_simple() * 0.35;
            },
            SeaLevelRiseImpact::PopulationDisplacement => {
                self.vulnerability_index = 0.85 + rand_simple() * 0.14;
                self.adaptation_capacity = 0.45 + rand_simple() * 0.40;
            },
        }

        if self.rise_rate == 0.0 {
            self.rise_rate = 0.3 + rand_simple() * 0.4;
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
    fn test_habitat_loss() {
        let mut system = SeaLevelRiseSystem::new(SeaLevelRiseImpact::HabitatLoss);
        system.analyze_system().unwrap();
        assert!(system.vulnerability_index > 0.7);
    }
}
