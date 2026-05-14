//! # SBMUMC Module 1531: Astral Travel
//!
//! Systems for astral travel and conscious projection.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AstralTravelTopic {
    ConsciousProjection,
    AstralBodyTravel,
    AstralRealms,
    LucidDreaming,
    OutOfBodyAdventure,
    AstralExploration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstralTravelSystem {
    pub system_id: String,
    pub astral_travel_topic: AstralTravelTopic,
    pub astral_journey: f64,
    pub dimensional_excursion: f64,
    pub soul_projection: f64,
    pub conscious_astral: f64,
}

impl AstralTravelSystem {
    pub fn new(astral_travel_topic: AstralTravelTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            astral_travel_topic,
            astral_journey: 0.0,
            dimensional_excursion: 0.0,
            soul_projection: 0.0,
            conscious_astral: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.astral_travel_topic {
            AstralTravelTopic::ConsciousProjection => {
                self.astral_journey = 0.95 + rand_simple() * 0.05;
                self.dimensional_excursion = 0.90 + rand_simple() * 0.10;
                self.soul_projection = 0.85 + rand_simple() * 0.14;
            },
            AstralTravelTopic::AstralBodyTravel => {
                self.conscious_astral = 0.95 + rand_simple() * 0.05;
                self.soul_projection = 0.90 + rand_simple() * 0.10;
                self.dimensional_excursion = 0.85 + rand_simple() * 0.14;
            },
            AstralTravelTopic::AstralRealms => {
                self.dimensional_excursion = 0.95 + rand_simple() * 0.05;
                self.astral_journey = 0.90 + rand_simple() * 0.10;
                self.conscious_astral = 0.85 + rand_simple() * 0.14;
            },
            AstralTravelTopic::LucidDreaming => {
                self.soul_projection = 0.95 + rand_simple() * 0.05;
                self.conscious_astral = 0.90 + rand_simple() * 0.10;
                self.astral_journey = 0.85 + rand_simple() * 0.14;
            },
            AstralTravelTopic::OutOfBodyAdventure => {
                self.astral_journey = 0.95 + rand_simple() * 0.05;
                self.dimensional_excursion = 0.90 + rand_simple() * 0.10;
                self.conscious_astral = 0.85 + rand_simple() * 0.14;
            },
            AstralTravelTopic::AstralExploration => {
                self.conscious_astral = 0.95 + rand_simple() * 0.05;
                self.soul_projection = 0.90 + rand_simple() * 0.10;
                self.dimensional_excursion = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.soul_projection == 0.0 {
            self.soul_projection = (self.astral_journey + self.dimensional_excursion) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_conscious_projection() {
        let mut system = AstralTravelSystem::new(AstralTravelTopic::ConsciousProjection);
        system.analyze_system().unwrap();
        assert!(system.astral_journey > 0.8);
    }
}