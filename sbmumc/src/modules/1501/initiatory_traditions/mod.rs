//! # SBMUMC Module 1501: Initiatory Traditions
//!
//! Systems for initiatory traditions and secret societies.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InitiatoryTraditionsTopic {
    MasonicLodge,
    RosicrucianOrder,
    HermeticOrder,
    KnightsTemplar,
    MysterySchools,
    SufiOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiatoryTraditionsSystem {
    pub system_id: String,
    pub initiatory_traditions_topic: InitiatoryTraditionsTopic,
    pub esoteric_knowledge: f64,
    pub ritual_initiation: f64,
    pub hidden_wisdom: f64,
    pub secret_transmission: f64,
}

impl InitiatoryTraditionsSystem {
    pub fn new(initiatory_traditions_topic: InitiatoryTraditionsTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            initiatory_traditions_topic,
            esoteric_knowledge: 0.0,
            ritual_initiation: 0.0,
            hidden_wisdom: 0.0,
            secret_transmission: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.initiatory_traditions_topic {
            InitiatoryTraditionsTopic::MasonicLodge => {
                self.esoteric_knowledge = 0.95 + rand_simple() * 0.05;
                self.ritual_initiation = 0.90 + rand_simple() * 0.10;
                self.hidden_wisdom = 0.85 + rand_simple() * 0.14;
            },
            InitiatoryTraditionsTopic::RosicrucianOrder => {
                self.secret_transmission = 0.95 + rand_simple() * 0.05;
                self.hidden_wisdom = 0.90 + rand_simple() * 0.10;
                self.ritual_initiation = 0.85 + rand_simple() * 0.14;
            },
            InitiatoryTraditionsTopic::HermeticOrder => {
                self.ritual_initiation = 0.95 + rand_simple() * 0.05;
                self.esoteric_knowledge = 0.90 + rand_simple() * 0.10;
                self.secret_transmission = 0.85 + rand_simple() * 0.14;
            },
            InitiatoryTraditionsTopic::KnightsTemplar => {
                self.hidden_wisdom = 0.95 + rand_simple() * 0.05;
                self.secret_transmission = 0.90 + rand_simple() * 0.10;
                self.esoteric_knowledge = 0.85 + rand_simple() * 0.14;
            },
            InitiatoryTraditionsTopic::MysterySchools => {
                self.esoteric_knowledge = 0.95 + rand_simple() * 0.05;
                self.ritual_initiation = 0.90 + rand_simple() * 0.10;
                self.secret_transmission = 0.85 + rand_simple() * 0.14;
            },
            InitiatoryTraditionsTopic::SufiOrder => {
                self.secret_transmission = 0.95 + rand_simple() * 0.05;
                self.hidden_wisdom = 0.90 + rand_simple() * 0.10;
                self.ritual_initiation = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.hidden_wisdom == 0.0 {
            self.hidden_wisdom = (self.esoteric_knowledge + self.ritual_initiation) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_masonic() {
        let mut system = InitiatoryTraditionsSystem::new(InitiatoryTraditionsTopic::MasonicLodge);
        system.analyze_system().unwrap();
        assert!(system.esoteric_knowledge > 0.8);
    }
}