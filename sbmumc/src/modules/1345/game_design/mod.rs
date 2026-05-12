//! # SBMUMC Module 1345: Game Design
//!
//! Systems for video game design and development.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameGenre {
    Action,
    Adventure,
    Puzzle,
    RolePlaying,
    Strategy,
    Simulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDesignSystem {
    pub system_id: String,
    pub game_genre: GameGenre,
    pub gameplay_mechanics: f64,
    pub narrative_depth: f64,
    pub visual_design: f64,
    pub player_engagement: f64,
}

impl GameDesignSystem {
    pub fn new(game_genre: GameGenre) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            game_genre,
            gameplay_mechanics: 0.0,
            narrative_depth: 0.0,
            visual_design: 0.0,
            player_engagement: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.game_genre {
            GameGenre::Action => {
                self.gameplay_mechanics = 0.95 + rand_simple() * 0.05;
                self.player_engagement = 0.90 + rand_simple() * 0.10;
                self.visual_design = 0.85 + rand_simple() * 0.14;
            },
            GameGenre::Adventure => {
                self.narrative_depth = 0.95 + rand_simple() * 0.05;
                self.player_engagement = 0.90 + rand_simple() * 0.10;
                self.visual_design = 0.85 + rand_simple() * 0.14;
            },
            GameGenre::Puzzle => {
                self.gameplay_mechanics = 0.95 + rand_simple() * 0.05;
                self.narrative_depth = 0.80 + rand_simple() * 0.18;
                self.player_engagement = 0.85 + rand_simple() * 0.14;
            },
            GameGenre::RolePlaying => {
                self.narrative_depth = 0.95 + rand_simple() * 0.05;
                self.gameplay_mechanics = 0.90 + rand_simple() * 0.10;
                self.player_engagement = 0.85 + rand_simple() * 0.14;
            },
            GameGenre::Strategy => {
                self.gameplay_mechanics = 0.95 + rand_simple() * 0.05;
                self.narrative_depth = 0.80 + rand_simple() * 0.18;
                self.player_engagement = 0.85 + rand_simple() * 0.14;
            },
            GameGenre::Simulation => {
                self.gameplay_mechanics = 0.90 + rand_simple() * 0.10;
                self.player_engagement = 0.85 + rand_simple() * 0.14;
                self.visual_design = 0.80 + rand_simple() * 0.18;
            },
        }

        if self.visual_design == 0.0 {
            self.visual_design = (self.gameplay_mechanics + self.narrative_depth) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_rpg() {
        let mut system = GameDesignSystem::new(GameGenre::RolePlaying);
        system.analyze_system().unwrap();
        assert!(system.narrative_depth > 0.8);
    }
}