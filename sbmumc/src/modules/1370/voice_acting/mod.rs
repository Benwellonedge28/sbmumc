//! # SBMUMC Module 1370: Voice Acting
//!
//! Systems for voice acting and voice-over work.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoiceActingType {
    Animation,
    VideoGames,
    Dubbing,
    Commercial,
    Narration,
    Audiobooks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceActingSystem {
    pub system_id: String,
    pub voice_type: VoiceActingType,
    pub vocal_expressiveness: f64,
    pub character_alignment: f64,
    pub technical_clarity: f64,
    pub emotional_range: f64,
}

impl VoiceActingSystem {
    pub fn new(voice_type: VoiceActingType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            voice_type,
            vocal_expressiveness: 0.0,
            character_alignment: 0.0,
            technical_clarity: 0.0,
            emotional_range: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.voice_type {
            VoiceActingType::Animation => {
                self.character_alignment = 0.95 + rand_simple() * 0.05;
                self.vocal_expressiveness = 0.90 + rand_simple() * 0.10;
                self.emotional_range = 0.85 + rand_simple() * 0.14;
            },
            VoiceActingType::VideoGames => {
                self.emotional_range = 0.95 + rand_simple() * 0.05;
                self.character_alignment = 0.90 + rand_simple() * 0.10;
                self.technical_clarity = 0.85 + rand_simple() * 0.14;
            },
            VoiceActingType::Dubbing => {
                self.technical_clarity = 0.95 + rand_simple() * 0.05;
                self.character_alignment = 0.90 + rand_simple() * 0.10;
                self.vocal_expressiveness = 0.85 + rand_simple() * 0.14;
            },
            VoiceActingType::Commercial => {
                self.vocal_expressiveness = 0.95 + rand_simple() * 0.05;
                self.technical_clarity = 0.90 + rand_simple() * 0.10;
                self.character_alignment = 0.85 + rand_simple() * 0.14;
            },
            VoiceActingType::Narration => {
                self.technical_clarity = 0.95 + rand_simple() * 0.05;
                self.vocal_expressiveness = 0.90 + rand_simple() * 0.10;
                self.emotional_range = 0.85 + rand_simple() * 0.14;
            },
            VoiceActingType::Audiobooks => {
                self.character_alignment = 0.95 + rand_simple() * 0.05;
                self.technical_clarity = 0.90 + rand_simple() * 0.10;
                self.emotional_range = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.emotional_range == 0.0 {
            self.emotional_range = (self.vocal_expressiveness + self.character_alignment) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_animation() {
        let mut system = VoiceActingSystem::new(VoiceActingType::Animation);
        system.analyze_system().unwrap();
        assert!(system.character_alignment > 0.8);
    }
}