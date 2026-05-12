//! # SBMUMC Module 1357: Casino Design
//!
//! Systems for casino and gaming facility design.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CasinoGameType {
    SlotMachines,
    TableGames,
    Poker,
    SportsBetting,
    HighRoller,
    Entertainment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasinoDesignSystem {
    pub system_id: String,
    pub casino_game_type: CasinoGameType,
    pub player_engagement: f64,
    pub atmosphere_creation: f64,
    pub revenue_generation: f64,
    pub responsible_gaming: f64,
}

impl CasinoDesignSystem {
    pub fn new(casino_game_type: CasinoGameType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            casino_game_type,
            player_engagement: 0.0,
            atmosphere_creation: 0.0,
            revenue_generation: 0.0,
            responsible_gaming: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.casino_game_type {
            CasinoGameType::SlotMachines => {
                self.player_engagement = 0.95 + rand_simple() * 0.05;
                self.revenue_generation = 0.90 + rand_simple() * 0.10;
                self.responsible_gaming = 0.75 + rand_simple() * 0.22;
            },
            CasinoGameType::TableGames => {
                self.player_engagement = 0.90 + rand_simple() * 0.10;
                self.atmosphere_creation = 0.85 + rand_simple() * 0.14;
                self.responsible_gaming = 0.85 + rand_simple() * 0.14;
            },
            CasinoGameType::Poker => {
                self.player_engagement = 0.95 + rand_simple() * 0.05;
                self.responsible_gaming = 0.90 + rand_simple() * 0.10;
                self.atmosphere_creation = 0.85 + rand_simple() * 0.14;
            },
            CasinoGameType::SportsBetting => {
                self.revenue_generation = 0.95 + rand_simple() * 0.05;
                self.player_engagement = 0.85 + rand_simple() * 0.14;
                self.responsible_gaming = 0.80 + rand_simple() * 0.18;
            },
            CasinoGameType::HighRoller => {
                self.revenue_generation = 0.95 + rand_simple() * 0.05;
                self.atmosphere_creation = 0.90 + rand_simple() * 0.10;
                self.player_engagement = 0.80 + rand_simple() * 0.18;
            },
            CasinoGameType::Entertainment => {
                self.atmosphere_creation = 0.95 + rand_simple() * 0.05;
                self.player_engagement = 0.90 + rand_simple() * 0.10;
                self.responsible_gaming = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.responsible_gaming == 0.0 {
            self.responsible_gaming = (self.player_engagement + self.atmosphere_creation) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_poker() {
        let mut system = CasinoDesignSystem::new(CasinoGameType::Poker);
        system.analyze_system().unwrap();
        assert!(system.player_engagement > 0.8);
    }
}