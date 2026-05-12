//! # SBMUMC Module 1343: Music Composition
//!
//! Systems for musical composition and arrangement.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MusicalGenre {
    Classical,
    Jazz,
    Electronic,
    Rock,
    Pop,
    Ambient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicCompositionSystem {
    pub system_id: String,
    pub musical_genre: MusicalGenre,
    pub melodic_complexity: f64,
    pub harmonic_richness: f64,
    pub rhythmic_innovation: f64,
    pub emotional_impact: f64,
}

impl MusicCompositionSystem {
    pub fn new(musical_genre: MusicalGenre) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            musical_genre,
            melodic_complexity: 0.0,
            harmonic_richness: 0.0,
            rhythmic_innovation: 0.0,
            emotional_impact: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.musical_genre {
            MusicalGenre::Classical => {
                self.melodic_complexity = 0.95 + rand_simple() * 0.05;
                self.harmonic_richness = 0.95 + rand_simple() * 0.05;
                self.emotional_impact = 0.90 + rand_simple() * 0.10;
            },
            MusicalGenre::Jazz => {
                self.harmonic_richness = 0.95 + rand_simple() * 0.05;
                self.rhythmic_innovation = 0.90 + rand_simple() * 0.10;
                self.melodic_complexity = 0.85 + rand_simple() * 0.14;
            },
            MusicalGenre::Electronic => {
                self.rhythmic_innovation = 0.95 + rand_simple() * 0.05;
                self.emotional_impact = 0.85 + rand_simple() * 0.14;
                self.harmonic_richness = 0.80 + rand_simple() * 0.18;
            },
            MusicalGenre::Rock => {
                self.emotional_impact = 0.90 + rand_simple() * 0.10;
                self.rhythmic_innovation = 0.85 + rand_simple() * 0.14;
                self.melodic_complexity = 0.75 + rand_simple() * 0.22;
            },
            MusicalGenre::Pop => {
                self.melodic_complexity = 0.90 + rand_simple() * 0.10;
                self.emotional_impact = 0.90 + rand_simple() * 0.10;
                self.harmonic_richness = 0.75 + rand_simple() * 0.22;
            },
            MusicalGenre::Ambient => {
                self.harmonic_richness = 0.90 + rand_simple() * 0.10;
                self.emotional_impact = 0.90 + rand_simple() * 0.10;
                self.melodic_complexity = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.rhythmic_innovation == 0.0 {
            self.rhythmic_innovation = (self.melodic_complexity + self.harmonic_richness) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_jazz() {
        let mut system = MusicCompositionSystem::new(MusicalGenre::Jazz);
        system.analyze_system().unwrap();
        assert!(system.harmonic_richness > 0.8);
    }
}