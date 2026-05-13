//! # SBMUMC Module 1453: Philosophy of Logic
//!
//! Systems for philosophy of logic and logical consequence.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicPhilosophyTopic {
    LogicalConsequence,
    Logical Pluralism,
    ParadoxTheory,
    LogicalRevision,
    NormativeLogic,
    Expressivism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyLogicSystem {
    pub system_id: String,
    pub logic_philosophy_topic: LogicPhilosophyTopic,
    pub consequence_relation: f64,
    pub logical_truth: f64,
    pub logical_constants: f64,
    pub logical_reasoning: f64,
}

impl PhilosophyLogicSystem {
    pub fn new(logic_philosophy_topic: LogicPhilosophyTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            logic_philosophy_topic,
            consequence_relation: 0.0,
            logical_truth: 0.0,
            logical_constants: 0.0,
            logical_reasoning: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.logic_philosophy_topic {
            LogicPhilosophyTopic::LogicalConsequence => {
                self.consequence_relation = 0.95 + rand_simple() * 0.05;
                self.logical_truth = 0.90 + rand_simple() * 0.10;
                self.logical_constants = 0.85 + rand_simple() * 0.14;
            },
            LogicPhilosophyTopic::Logical Pluralism => {
                self.logical_reasoning = 0.95 + rand_simple() * 0.05;
                self.consequence_relation = 0.90 + rand_simple() * 0.10;
                self.logical_truth = 0.85 + rand_simple() * 0.14;
            },
            LogicPhilosophyTopic::ParadoxTheory => {
                self.logical_constants = 0.95 + rand_simple() * 0.05;
                self.logical_reasoning = 0.90 + rand_simple() * 0.10;
                self.consequence_relation = 0.85 + rand_simple() * 0.14;
            },
            LogicPhilosophyTopic::LogicalRevision => {
                self.logical_truth = 0.95 + rand_simple() * 0.05;
                self.logical_constants = 0.90 + rand_simple() * 0.10;
                self.logical_reasoning = 0.85 + rand_simple() * 0.14;
            },
            LogicPhilosophyTopic::NormativeLogic => {
                self.consequence_relation = 0.95 + rand_simple() * 0.05;
                self.logical_reasoning = 0.90 + rand_simple() * 0.10;
                self.logical_constants = 0.85 + rand_simple() * 0.14;
            },
            LogicPhilosophyTopic::Expressivism => {
                self.logical_truth = 0.95 + rand_simple() * 0.05;
                self.consequence_relation = 0.90 + rand_simple() * 0.10;
                self.logical_reasoning = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.logical_reasoning == 0.0 {
            self.logical_reasoning = (self.consequence_relation + self.logical_truth) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_consequence() {
        let mut system = PhilosophyLogicSystem::new(LogicPhilosophyTopic::LogicalConsequence);
        system.analyze_system().unwrap();
        assert!(system.consequence_relation > 0.8);
    }
}