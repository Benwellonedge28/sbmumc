//! # SBMUMC Module 1287: Aquatic Invasive Species
//!
//! Systems for managing invasive species in aquatic environments.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvasiveSpeciesPathway {
    BallastWater,
    HullFouling,
    AquacultureEscape,
    CanalsChannels,
    AquariumTrade,
    LiveBait,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquaticInvasiveSpeciesSystem {
    pub system_id: String,
    pub pathway: InvasiveSpeciesPathway,
    pub invasion_risk: f64,
    pub detection_capability: f64,
    pub control_effectiveness: f64,
    pub ecosystem_impact: f64,
}

impl AquaticInvasiveSpeciesSystem {
    pub fn new(pathway: InvasiveSpeciesPathway) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            pathway,
            invasion_risk: 0.0,
            detection_capability: 0.0,
            control_effectiveness: 0.0,
            ecosystem_impact: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.pathway {
            InvasiveSpeciesPathway::BallastWater => {
                self.invasion_risk = 0.85 + rand_simple() * 0.14;
                self.control_effectiveness = 0.70 + rand_simple() * 0.25;
                self.detection_capability = 0.75 + rand_simple() * 0.22;
            },
            InvasiveSpeciesPathway::HullFouling => {
                self.invasion_risk = 0.75 + rand_simple() * 0.22;
                self.detection_capability = 0.70 + rand_simple() * 0.25;
                self.control_effectiveness = 0.65 + rand_simple() * 0.30;
            },
            InvasiveSpeciesPathway::AquacultureEscape => {
                self.invasion_risk = 0.70 + rand_simple() * 0.25;
                self.control_effectiveness = 0.60 + rand_simple() * 0.35;
                self.detection_capability = 0.80 + rand_simple() * 0.18;
            },
            InvasiveSpeciesPathway::CanalsChannels => {
                self.invasion_risk = 0.90 + rand_simple() * 0.10;
                self.control_effectiveness = 0.50 + rand_simple() * 0.40;
                self.detection_capability = 0.60 + rand_simple() * 0.35;
            },
            InvasiveSpeciesPathway::AquariumTrade => {
                self.invasion_risk = 0.65 + rand_simple() * 0.30;
                self.detection_capability = 0.55 + rand_simple() * 0.40;
                self.control_effectiveness = 0.45 + rand_simple() * 0.40;
            },
            InvasiveSpeciesPathway::LiveBait => {
                self.invasion_risk = 0.60 + rand_simple() * 0.35;
                self.control_effectiveness = 0.70 + rand_simple() * 0.25;
                self.detection_capability = 0.65 + rand_simple() * 0.30;
            },
        }

        if self.ecosystem_impact == 0.0 {
            self.ecosystem_impact = self.invasion_risk * (1.0 - self.control_effectiveness) * (0.5 + rand_simple() * 0.5);
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
    fn test_canals_pathway() {
        let mut system = AquaticInvasiveSpeciesSystem::new(InvasiveSpeciesPathway::CanalsChannels);
        system.analyze_system().unwrap();
        assert!(system.invasion_risk > 0.7);
    }
}
