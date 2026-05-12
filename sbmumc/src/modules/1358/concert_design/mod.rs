//! # SBMUMC Module 1358: Concert Design
//!
//! Systems for concert and live music event design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConcertType {
    Arena,
    Festival,
    Club,
    Classical,
    Jazz,
    Electronic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcertDesignSystem {
    pub system_id: String,
    pub concert_type: ConcertType,
    pub sound_quality: f64,
    pub visual_experience: f64,
    pub crowd_management: f64,
    pub artist_connection: f64,
}

impl ConcertDesignSystem {
    pub fn new(concert_type: ConcertType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            concert_type,
            sound_quality: 0.0,
            visual_experience: 0.0,
            crowd_management: 0.0,
            artist_connection: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.concert_type {
            ConcertType::Arena => {
                self.sound_quality = 0.95 + rand_simple() * 0.05;
                self.visual_experience = 0.90 + rand_simple() * 0.10;
                self.crowd_management = 0.85 + rand_simple() * 0.14;
            },
            ConcertType::Festival => {
                self.visual_experience = 0.95 + rand_simple() * 0.05;
                self.crowd_management = 0.90 + rand_simple() * 0.10;
                self.artist_connection = 0.85 + rand_simple() * 0.14;
            },
            ConcertType::Club => {
                self.sound_quality = 0.90 + rand_simple() * 0.10;
                self.artist_connection = 0.95 + rand_simple() * 0.05;
                self.visual_experience = 0.85 + rand_simple() * 0.14;
            },
            ConcertType::Classical => {
                self.sound_quality = 0.95 + rand_simple() * 0.05;
                self.artist_connection = 0.90 + rand_simple() * 0.10;
                self.crowd_management = 0.85 + rand_simple() * 0.14;
            },
            ConcertType::Jazz => {
                self.sound_quality = 0.95 + rand_simple() * 0.05;
                self.artist_connection = 0.95 + rand_simple() * 0.05;
                self.crowd_management = 0.80 + rand_simple() * 0.18;
            },
            ConcertType::Electronic => {
                self.visual_experience = 0.95 + rand_simple() * 0.05;
                self.sound_quality = 0.90 + rand_simple() * 0.10;
                self.crowd_management = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.artist_connection == 0.0 {
            self.artist_connection = (self.sound_quality + self.visual_experience) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_electronic() {
        let mut system = ConcertDesignSystem::new(ConcertType::Electronic);
        system.analyze_system().unwrap();
        assert!(system.visual_experience > 0.8);
    }
}