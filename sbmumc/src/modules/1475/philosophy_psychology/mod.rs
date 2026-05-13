//! # SBMUMC Module 1475: Philosophy of Psychology
//!
//! Systems for philosophy of psychology and cognitive science.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhilosophyPsychologyTopic {
    FolkPsychology,
    TheoryTheory,
    SimulationTheory,
    ReductionismPsych,
    MentalRepresentation,
    ComputationalTheory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyPsychologySystem {
    pub system_id: String,
    pub philosophy_psychology_topic: PhilosophyPsychologyTopic,
    pub folk_psychology_legitimacy: f64,
    pub mind_representation: f64,
    pub psychological_explanation: f64,
    pub cognitive_architecture: f64,
}

impl PhilosophyPsychologySystem {
    pub fn new(philosophy_psychology_topic: PhilosophyPsychologyTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            philosophy_psychology_topic,
            folk_psychology_legitimacy: 0.0,
            mind_representation: 0.0,
            psychological_explanation: 0.0,
            cognitive_architecture: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.philosophy_psychology_topic {
            PhilosophyPsychologyTopic::FolkPsychology => {
                self.folk_psychology_legitimacy = 0.95 + rand_simple() * 0.05;
                self.mind_representation = 0.90 + rand_simple() * 0.10;
                self.psychological_explanation = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyPsychologyTopic::TheoryTheory => {
                self.cognitive_architecture = 0.95 + rand_simple() * 0.05;
                self.folk_psychology_legitimacy = 0.90 + rand_simple() * 0.10;
                self.mind_representation = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyPsychologyTopic::SimulationTheory => {
                self.psychological_explanation = 0.95 + rand_simple() * 0.05;
                self.cognitive_architecture = 0.90 + rand_simple() * 0.10;
                self.folk_psychology_legitimacy = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyPsychologyTopic::ReductionismPsych => {
                self.mind_representation = 0.95 + rand_simple() * 0.05;
                self.psychological_explanation = 0.90 + rand_simple() * 0.10;
                self.cognitive_architecture = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyPsychologyTopic::MentalRepresentation => {
                self.folk_psychology_legitimacy = 0.95 + rand_simple() * 0.05;
                self.cognitive_architecture = 0.90 + rand_simple() * 0.10;
                self.psychological_explanation = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyPsychologyTopic::ComputationalTheory => {
                self.mind_representation = 0.95 + rand_simple() * 0.05;
                self.folk_psychology_legitimacy = 0.90 + rand_simple() * 0.10;
                self.cognitive_architecture = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.cognitive_architecture == 0.0 {
            self.cognitive_architecture = (self.folk_psychology_legitimacy + self.mind_representation) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_folk_psychology() {
        let mut system = PhilosophyPsychologySystem::new(PhilosophyPsychologyTopic::FolkPsychology);
        system.analyze_system().unwrap();
        assert!(system.folk_psychology_legitimacy > 0.8);
    }
}