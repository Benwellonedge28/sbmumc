//! # SBMUMC Module 1380: Creative Direction
//!
//! Systems for creative direction in media projects.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreativeDirectionContext {
    Advertising,
    MusicVideos,
    BrandIdentity,
    InteractiveMedia,
    VisualArt,
    CrossMedia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeDirectionSystem {
    pub system_id: String,
    pub direction_context: CreativeDirectionContext,
    pub artistic_vision: f64,
    pub brand_alignment: f64,
    pub team_leadership: f64,
    pub innovation_driving: f64,
}

impl CreativeDirectionSystem {
    pub fn new(direction_context: CreativeDirectionContext) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            direction_context,
            artistic_vision: 0.0,
            brand_alignment: 0.0,
            team_leadership: 0.0,
            innovation_driving: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.direction_context {
            CreativeDirectionContext::Advertising => {
                self.brand_alignment = 0.95 + rand_simple() * 0.05;
                self.artistic_vision = 0.90 + rand_simple() * 0.10;
                self.innovation_driving = 0.85 + rand_simple() * 0.14;
            },
            CreativeDirectionContext::MusicVideos => {
                self.artistic_vision = 0.95 + rand_simple() * 0.05;
                self.innovation_driving = 0.90 + rand_simple() * 0.10;
                self.team_leadership = 0.85 + rand_simple() * 0.14;
            },
            CreativeDirectionContext::BrandIdentity => {
                self.brand_alignment = 0.95 + rand_simple() * 0.05;
                self.team_leadership = 0.90 + rand_simple() * 0.10;
                self.artistic_vision = 0.85 + rand_simple() * 0.14;
            },
            CreativeDirectionContext::InteractiveMedia => {
                self.innovation_driving = 0.95 + rand_simple() * 0.05;
                self.team_leadership = 0.90 + rand_simple() * 0.10;
                self.artistic_vision = 0.85 + rand_simple() * 0.14;
            },
            CreativeDirectionContext::VisualArt => {
                self.artistic_vision = 0.95 + rand_simple() * 0.05;
                self.innovation_driving = 0.90 + rand_simple() * 0.10;
                self.brand_alignment = 0.85 + rand_simple() * 0.14;
            },
            CreativeDirectionContext::CrossMedia => {
                self.innovation_driving = 0.95 + rand_simple() * 0.05;
                self.brand_alignment = 0.90 + rand_simple() * 0.10;
                self.team_leadership = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.team_leadership == 0.0 {
            self.team_leadership = (self.artistic_vision + self.brand_alignment) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_music_videos() {
        let mut system = CreativeDirectionSystem::new(CreativeDirectionContext::MusicVideos);
        system.analyze_system().unwrap();
        assert!(system.artistic_vision > 0.8);
    }
}