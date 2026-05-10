//! # SBMUMC Module 1244: Marine Biology
//!
//! Scientific study of marine organisms and their environments.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarineResearchFocus {
    PlanktonEcology,
    CoralReefs,
    DeepSea,
    Coastal,
    Pelagic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarineBiologySystem {
    pub system_id: String,
    pub research_focus: MarineResearchFocus,
    pub species_discovery_rate: f64,
    pub ecological_insight: f64,
    pub conservation_application: f64,
    pub climate_relevance: f64,
}

impl MarineBiologySystem {
    pub fn new(research_focus: MarineResearchFocus) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            research_focus,
            species_discovery_rate: 0.0,
            ecological_insight: 0.0,
            conservation_application: 0.0,
            climate_relevance: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.research_focus {
            MarineResearchFocus::PlanktonEcology => {
                self.species_discovery_rate = 0.80 + rand_simple() * 0.18;
                self.climate_relevance = 0.90 + rand_simple() * 0.10;
            },
            MarineResearchFocus::CoralReefs => {
                self.species_discovery_rate = 0.75 + rand_simple() * 0.22;
                self.conservation_application = 0.85 + rand_simple() * 0.14;
            },
            MarineResearchFocus::DeepSea => {
                self.species_discovery_rate = 0.90 + rand_simple() * 0.10;
                self.ecological_insight = 0.80 + rand_simple() * 0.18;
            },
            MarineResearchFocus::Coastal => {
                self.ecological_insight = 0.85 + rand_simple() * 0.14;
                self.conservation_application = 0.80 + rand_simple() * 0.18;
            },
            MarineResearchFocus::Pelagic => {
                self.climate_relevance = 0.85 + rand_simple() * 0.14;
                self.ecological_insight = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.conservation_application == 0.0 {
            self.conservation_application = (self.ecological_insight + self.species_discovery_rate) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_deep_sea_research() {
        let mut system = MarineBiologySystem::new(MarineResearchFocus::DeepSea);
        system.analyze_system().unwrap();
        assert!(system.species_discovery_rate > 0.7);
    }
}