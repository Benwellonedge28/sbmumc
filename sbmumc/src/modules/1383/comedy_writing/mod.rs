//! # SBMUMC Module 1383: Comedy Writing
//!
//! Systems for comedy writing and humor development.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComedyStyle {
    StandUp,
    Sitcom,
    Sketch,
    Improv,
    Satire,
    DarkComedy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComedyWritingSystem {
    pub system_id: String,
    pub comedy_style: ComedyStyle,
    pub comedic_timing: f64,
    pub wordplay_mastery: f64,
    pub audience_connection: f64,
    pub social_commentary: f64,
}

impl ComedyWritingSystem {
    pub fn new(comedy_style: ComedyStyle) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            comedy_style,
            comedic_timing: 0.0,
            wordplay_mastery: 0.0,
            audience_connection: 0.0,
            social_commentary: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.comedy_style {
            ComedyStyle::StandUp => {
                self.comedic_timing = 0.95 + rand_simple() * 0.05;
                self.audience_connection = 0.90 + rand_simple() * 0.10;
                self.wordplay_mastery = 0.85 + rand_simple() * 0.14;
            },
            ComedyStyle::Sitcom => {
                self.audience_connection = 0.95 + rand_simple() * 0.05;
                self.comedic_timing = 0.90 + rand_simple() * 0.10;
                self.social_commentary = 0.85 + rand_simple() * 0.14;
            },
            ComedyStyle::Sketch => {
                self.wordplay_mastery = 0.95 + rand_simple() * 0.05;
                self.social_commentary = 0.90 + rand_simple() * 0.10;
                self.comedic_timing = 0.85 + rand_simple() * 0.14;
            },
            ComedyStyle::Improv => {
                self.comedic_timing = 0.95 + rand_simple() * 0.05;
                self.audience_connection = 0.90 + rand_simple() * 0.10;
                self.wordplay_mastery = 0.85 + rand_simple() * 0.14;
            },
            ComedyStyle::Satire => {
                self.social_commentary = 0.95 + rand_simple() * 0.05;
                self.wordplay_mastery = 0.90 + rand_simple() * 0.10;
                self.audience_connection = 0.85 + rand_simple() * 0.14;
            },
            ComedyStyle::DarkComedy => {
                self.social_commentary = 0.95 + rand_simple() * 0.05;
                self.comedic_timing = 0.90 + rand_simple() * 0.10;
                self.wordplay_mastery = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.social_commentary == 0.0 {
            self.social_commentary = (self.comedic_timing + self.audience_connection) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_standup() {
        let mut system = ComedyWritingSystem::new(ComedyStyle::StandUp);
        system.analyze_system().unwrap();
        assert!(system.comedic_timing > 0.8);
    }
}
