//! # SBMUMC Module 1355: Esports
//!
//! Systems for competitive gaming and esports.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EsportsGameType {
    MOBA,
    FPS,
    Fighting,
    Strategy,
    BattleRoyale,
    Sports,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EsportsSystem {
    pub system_id: String,
    pub game_type: EsportsGameType,
    pub competitive_balance: f64,
    pub spectator_appeal: f64,
    pub skill_celebration: f64,
    pub community_growth: f64,
}

impl EsportsSystem {
    pub fn new(game_type: EsportsGameType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            game_type,
            competitive_balance: 0.0,
            spectator_appeal: 0.0,
            skill_celebration: 0.0,
            community_growth: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.game_type {
            EsportsGameType::MOBA => {
                self.competitive_balance = 0.95 + rand_simple() * 0.05;
                self.spectator_appeal = 0.90 + rand_simple() * 0.10;
                self.community_growth = 0.85 + rand_simple() * 0.14;
            },
            EsportsGameType::FPS => {
                self.skill_celebration = 0.95 + rand_simple() * 0.05;
                self.spectator_appeal = 0.90 + rand_simple() * 0.10;
                self.competitive_balance = 0.85 + rand_simple() * 0.14;
            },
            EsportsGameType::Fighting => {
                self.skill_celebration = 0.95 + rand_simple() * 0.05;
                self.spectator_appeal = 0.90 + rand_simple() * 0.10;
                self.competitive_balance = 0.85 + rand_simple() * 0.14;
            },
            EsportsGameType::Strategy => {
                self.competitive_balance = 0.95 + rand_simple() * 0.05;
                self.skill_celebration = 0.90 + rand_simple() * 0.10;
                self.spectator_appeal = 0.80 + rand_simple() * 0.18;
            },
            EsportsGameType::BattleRoyale => {
                self.spectator_appeal = 0.95 + rand_simple() * 0.05;
                self.community_growth = 0.90 + rand_simple() * 0.10;
                self.competitive_balance = 0.80 + rand_simple() * 0.18;
            },
            EsportsGameType::Sports => {
                self.spectator_appeal = 0.95 + rand_simple() * 0.05;
                self.competitive_balance = 0.90 + rand_simple() * 0.10;
                self.skill_celebration = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.community_growth == 0.0 {
            self.community_growth = (self.spectator_appeal + self.competitive_balance) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_moba() {
        let mut system = EsportsSystem::new(EsportsGameType::MOBA);
        system.analyze_system().unwrap();
        assert!(system.competitive_balance > 0.8);
    }
}