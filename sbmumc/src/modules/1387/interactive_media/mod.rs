//! # SBMUMC Module 1387: Interactive Media
//!
//! Systems for interactive media design and development.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractivePlatform {
    VideoGames,
    InteractiveFilm,
    DigitalInstallations,
    TransmediaNarrative,
    LocationBased,
    BrowserBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveMediaSystem {
    pub system_id: String,
    pub interactive_platform: InteractivePlatform,
    pub user_experience_design: f64,
    pub branching_narrative: f64,
    pub mechanic_integration: f64,
    pub feedback_loops: f64,
}

impl InteractiveMediaSystem {
    pub fn new(interactive_platform: InteractivePlatform) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            interactive_platform,
            user_experience_design: 0.0,
            branching_narrative: 0.0,
            mechanic_integration: 0.0,
            feedback_loops: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.interactive_platform {
            InteractivePlatform::VideoGames => {
                self.mechanic_integration = 0.95 + rand_simple() * 0.05;
                self.user_experience_design = 0.90 + rand_simple() * 0.10;
                self.feedback_loops = 0.85 + rand_simple() * 0.14;
            },
            InteractivePlatform::InteractiveFilm => {
                self.branching_narrative = 0.95 + rand_simple() * 0.05;
                self.user_experience_design = 0.90 + rand_simple() * 0.10;
                self.mechanic_integration = 0.85 + rand_simple() * 0.14;
            },
            InteractivePlatform::DigitalInstallations => {
                self.feedback_loops = 0.95 + rand_simple() * 0.05;
                self.mechanic_integration = 0.90 + rand_simple() * 0.10;
                self.branching_narrative = 0.85 + rand_simple() * 0.14;
            },
            InteractivePlatform::TransmediaNarrative => {
                self.branching_narrative = 0.95 + rand_simple() * 0.05;
                self.feedback_loops = 0.90 + rand_simple() * 0.10;
                self.user_experience_design = 0.85 + rand_simple() * 0.14;
            },
            InteractivePlatform::LocationBased => {
                self.user_experience_design = 0.95 + rand_simple() * 0.05;
                self.mechanic_integration = 0.90 + rand_simple() * 0.10;
                self.feedback_loops = 0.85 + rand_simple() * 0.14;
            },
            InteractivePlatform::BrowserBased => {
                self.mechanic_integration = 0.95 + rand_simple() * 0.05;
                self.feedback_loops = 0.90 + rand_simple() * 0.10;
                self.branching_narrative = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.feedback_loops == 0.0 {
            self.feedback_loops = (self.user_experience_design + self.mechanic_integration) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_video_games() {
        let mut system = InteractiveMediaSystem::new(InteractivePlatform::VideoGames);
        system.analyze_system().unwrap();
        assert!(system.mechanic_integration > 0.8);
    }
}
