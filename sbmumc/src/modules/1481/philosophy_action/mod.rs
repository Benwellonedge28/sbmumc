//! # SBMUMC Module 1481: Philosophy of Action
//!
//! Systems for philosophy of action and agency.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhilosophyActionTopic {
    ActionTheory,
    IntentionPhilosophy,
    BasicActions,
    AgencyConditions,
    ActionExplanation,
    WillPower,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyActionSystem {
    pub system_id: String,
    pub philosophy_action_topic: PhilosophyActionTopic,
    pub action_identification: f64,
    pub intention_formation: f64,
    pub agency_conditions: f64,
    pub explanatory_structure: f64,
}

impl PhilosophyActionSystem {
    pub fn new(philosophy_action_topic: PhilosophyActionTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            philosophy_action_topic,
            action_identification: 0.0,
            intention_formation: 0.0,
            agency_conditions: 0.0,
            explanatory_structure: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.philosophy_action_topic {
            PhilosophyActionTopic::ActionTheory => {
                self.action_identification = 0.95 + rand_simple() * 0.05;
                self.intention_formation = 0.90 + rand_simple() * 0.10;
                self.agency_conditions = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyActionTopic::IntentionPhilosophy => {
                self.explanatory_structure = 0.95 + rand_simple() * 0.05;
                self.action_identification = 0.90 + rand_simple() * 0.10;
                self.intention_formation = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyActionTopic::BasicActions => {
                self.agency_conditions = 0.95 + rand_simple() * 0.05;
                self.explanatory_structure = 0.90 + rand_simple() * 0.10;
                self.action_identification = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyActionTopic::AgencyConditions => {
                self.intention_formation = 0.95 + rand_simple() * 0.05;
                self.agency_conditions = 0.90 + rand_simple() * 0.10;
                self.explanatory_structure = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyActionTopic::ActionExplanation => {
                self.action_identification = 0.95 + rand_simple() * 0.05;
                self.intention_formation = 0.90 + rand_simple() * 0.10;
                self.explanatory_structure = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyActionTopic::WillPower => {
                self.agency_conditions = 0.95 + rand_simple() * 0.05;
                self.explanatory_structure = 0.90 + rand_simple() * 0.10;
                self.intention_formation = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.intention_formation == 0.0 {
            self.intention_formation = (self.action_identification + self.agency_conditions) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_action_theory() {
        let mut system = PhilosophyActionSystem::new(PhilosophyActionTopic::ActionTheory);
        system.analyze_system().unwrap();
        assert!(system.action_identification > 0.8);
    }
}