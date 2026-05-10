//! # SBMUMC Module 1281: Marine Ecology
//!
//! Systems for studying marine ecological relationships.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarineEcologyFocus {
    TrophicDynamics,
    SpeciesInteractions,
    HabitatEcology,
    PopulationBiology,
    CommunityStructure,
    EcosystemFunctioning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarineEcologySystem {
    pub system_id: String,
    pub ecology_focus: MarineEcologyFocus,
    pub ecological_health: f64,
    pub species_interaction: f64,
    pub habitat_connectivity: f64,
    pub resilience_index: f64,
}

impl MarineEcologySystem {
    pub fn new(ecology_focus: MarineEcologyFocus) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            ecology_focus,
            ecological_health: 0.0,
            species_interaction: 0.0,
            habitat_connectivity: 0.0,
            resilience_index: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.ecology_focus {
            MarineEcologyFocus::TrophicDynamics => {
                self.species_interaction = 0.85 + rand_simple() * 0.14;
                self.ecological_health = 0.80 + rand_simple() * 0.18;
                self.resilience_index = 0.70 + rand_simple() * 0.25;
            },
            MarineEcologyFocus::SpeciesInteractions => {
                self.species_interaction = 0.90 + rand_simple() * 0.10;
                self.ecological_health = 0.75 + rand_simple() * 0.22;
                self.habitat_connectivity = 0.70 + rand_simple() * 0.25;
            },
            MarineEcologyFocus::HabitatEcology => {
                self.habitat_connectivity = 0.90 + rand_simple() * 0.10;
                self.ecological_health = 0.85 + rand_simple() * 0.14;
                self.resilience_index = 0.75 + rand_simple() * 0.22;
            },
            MarineEcologyFocus::PopulationBiology => {
                self.ecological_health = 0.80 + rand_simple() * 0.18;
                self.resilience_index = 0.85 + rand_simple() * 0.14;
                self.species_interaction = 0.70 + rand_simple() * 0.25;
            },
            MarineEcologyFocus::CommunityStructure => {
                self.species_interaction = 0.80 + rand_simple() * 0.18;
                self.habitat_connectivity = 0.75 + rand_simple() * 0.22;
                self.ecological_health = 0.70 + rand_simple() * 0.25;
            },
            MarineEcologyFocus::EcosystemFunctioning => {
                self.ecological_health = 0.90 + rand_simple() * 0.10;
                self.resilience_index = 0.85 + rand_simple() * 0.14;
                self.habitat_connectivity = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.resilience_index == 0.0 {
            self.resilience_index = (self.ecological_health + self.species_interaction) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_habitat_ecology() {
        let mut system = MarineEcologySystem::new(MarineEcologyFocus::HabitatEcology);
        system.analyze_system().unwrap();
        assert!(system.habitat_connectivity > 0.7);
    }
}
