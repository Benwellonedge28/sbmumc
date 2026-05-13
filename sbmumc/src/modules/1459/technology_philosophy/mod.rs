//! # SBMUMC Module 1459: Philosophy of Technology
//!
//! Systems for philosophy of technology and digital ethics.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechnologyPhilosophyTopic {
    TechnologicalDeterminism,
    PhilosophyOfAI,
    DigitalEthics,
    TechnoOptimism,
    TechnoPessimism,
    PostHumanism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnologyPhilosophySystem {
    pub system_id: String,
    pub technology_philosophy_topic: TechnologyPhilosophyTopic,
    pub technological_critical: f64,
    pub design_ethics: f64,
    pub innovation_philosophy: f64,
    pub human_technology_relation: f64,
}

impl TechnologyPhilosophySystem {
    pub fn new(technology_philosophy_topic: TechnologyPhilosophyTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            technology_philosophy_topic,
            technological_critical: 0.0,
            design_ethics: 0.0,
            innovation_philosophy: 0.0,
            human_technology_relation: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.technology_philosophy_topic {
            TechnologyPhilosophyTopic::TechnologicalDeterminism => {
                self.technological_critical = 0.95 + rand_simple() * 0.05;
                self.design_ethics = 0.90 + rand_simple() * 0.10;
                self.innovation_philosophy = 0.85 + rand_simple() * 0.14;
            },
            TechnologyPhilosophyTopic::PhilosophyOfAI => {
                self.human_technology_relation = 0.95 + rand_simple() * 0.05;
                self.technological_critical = 0.90 + rand_simple() * 0.10;
                self.design_ethics = 0.85 + rand_simple() * 0.14;
            },
            TechnologyPhilosophyTopic::DigitalEthics => {
                self.innovation_philosophy = 0.95 + rand_simple() * 0.05;
                self.human_technology_relation = 0.90 + rand_simple() * 0.10;
                self.technological_critical = 0.85 + rand_simple() * 0.14;
            },
            TechnologyPhilosophyTopic::TechnoOptimism => {
                self.design_ethics = 0.95 + rand_simple() * 0.05;
                self.innovation_philosophy = 0.90 + rand_simple() * 0.10;
                self.human_technology_relation = 0.85 + rand_simple() * 0.14;
            },
            TechnologyPhilosophyTopic::TechnoPessimism => {
                self.technological_critical = 0.95 + rand_simple() * 0.05;
                self.human_technology_relation = 0.90 + rand_simple() * 0.10;
                self.design_ethics = 0.85 + rand_simple() * 0.14;
            },
            TechnologyPhilosophyTopic::PostHumanism => {
                self.design_ethics = 0.95 + rand_simple() * 0.05;
                self.technological_critical = 0.90 + rand_simple() * 0.10;
                self.innovation_philosophy = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.innovation_philosophy == 0.0 {
            self.innovation_philosophy = (self.technological_critical + self.design_ethics) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_determinism() {
        let mut system = TechnologyPhilosophySystem::new(TechnologyPhilosophyTopic::TechnologicalDeterminism);
        system.analyze_system().unwrap();
        assert!(system.technological_critical > 0.8);
    }
}