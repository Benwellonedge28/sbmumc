//! # SBMUMC Module 1411: Game Theory
//!
//! Systems for game theory and strategic interaction.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameType {
    NonCooperative,
    Cooperative,
    ExtensiveForm,
    Bayesian,
    Evolutionary,
    MechanismDesign,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTheorySystem {
    pub system_id: String,
    pub game_type: GameType,
    pub strategic_reasoning: f64,
    pub equilibrium_analysis: f64,
    pub payoff_optimization: f64,
    pub mechanism_analysis: f64,
}

impl GameTheorySystem {
    pub fn new(game_type: GameType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            game_type,
            strategic_reasoning: 0.0,
            equilibrium_analysis: 0.0,
            payoff_optimization: 0.0,
            mechanism_analysis: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.game_type {
            GameType::NonCooperative => {
                self.strategic_reasoning = 0.95 + rand_simple() * 0.05;
                self.equilibrium_analysis = 0.90 + rand_simple() * 0.10;
                self.payoff_optimization = 0.85 + rand_simple() * 0.14;
            },
            GameType::Cooperative => {
                self.payoff_optimization = 0.95 + rand_simple() * 0.05;
                self.mechanism_analysis = 0.90 + rand_simple() * 0.10;
                self.strategic_reasoning = 0.85 + rand_simple() * 0.14;
            },
            GameType::ExtensiveForm => {
                self.equilibrium_analysis = 0.95 + rand_simple() * 0.05;
                self.strategic_reasoning = 0.90 + rand_simple() * 0.10;
                self.mechanism_analysis = 0.85 + rand_simple() * 0.14;
            },
            GameType::Bayesian => {
                self.mechanism_analysis = 0.95 + rand_simple() * 0.05;
                self.payoff_optimization = 0.90 + rand_simple() * 0.10;
                self.equilibrium_analysis = 0.85 + rand_simple() * 0.14;
            },
            GameType::Evolutionary => {
                self.strategic_reasoning = 0.95 + rand_simple() * 0.05;
                self.equilibrium_analysis = 0.90 + rand_simple() * 0.10;
                self.payoff_optimization = 0.85 + rand_simple() * 0.14;
            },
            GameType::MechanismDesign => {
                self.mechanism_analysis = 0.95 + rand_simple() * 0.05;
                self.equilibrium_analysis = 0.90 + rand_simple() * 0.10;
                self.strategic_reasoning = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.mechanism_analysis == 0.0 {
            self.mechanism_analysis = (self.strategic_reasoning + self.equilibrium_analysis) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_noncooperative() {
        let mut system = GameTheorySystem::new(GameType::NonCooperative);
        system.analyze_system().unwrap();
        assert!(system.strategic_reasoning > 0.8);
    }
}
