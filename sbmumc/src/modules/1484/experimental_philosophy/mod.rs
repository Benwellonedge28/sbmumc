//! # SBMUMC Module 1484: Experimental Philosophy
//!
//! Systems for experimental philosophy and empirical philosophy.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperimentalPhilosophyTopic {
    IntuitionsCrossCultural,
    ExperimentalEpistemology,
    ExperimentalEthics,
    MetaphysicsEmpirical,
    LinguisticExperiments,
    CognitiveHistoriography,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentalPhilosophySystem {
    pub system_id: String,
    pub experimental_philosophy_topic: ExperimentalPhilosophyTopic,
    pub empirical_methodology: f64,
    pub intuition_analysis: f64,
    pub conceptual_experiments: f64,
    pub cross_cultural_studies: f64,
}

impl ExperimentalPhilosophySystem {
    pub fn new(experimental_philosophy_topic: ExperimentalPhilosophyTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            experimental_philosophy_topic,
            empirical_methodology: 0.0,
            intuition_analysis: 0.0,
            conceptual_experiments: 0.0,
            cross_cultural_studies: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.experimental_philosophy_topic {
            ExperimentalPhilosophyTopic::IntuitionsCrossCultural => {
                self.empirical_methodology = 0.95 + rand_simple() * 0.05;
                self.intuition_analysis = 0.90 + rand_simple() * 0.10;
                self.conceptual_experiments = 0.85 + rand_simple() * 0.14;
            },
            ExperimentalPhilosophyTopic::ExperimentalEpistemology => {
                self.cross_cultural_studies = 0.95 + rand_simple() * 0.05;
                self.empirical_methodology = 0.90 + rand_simple() * 0.10;
                self.intuition_analysis = 0.85 + rand_simple() * 0.14;
            },
            ExperimentalPhilosophyTopic::ExperimentalEthics => {
                self.conceptual_experiments = 0.95 + rand_simple() * 0.05;
                self.cross_cultural_studies = 0.90 + rand_simple() * 0.10;
                self.empirical_methodology = 0.85 + rand_simple() * 0.14;
            },
            ExperimentalPhilosophyTopic::MetaphysicsEmpirical => {
                self.intuition_analysis = 0.95 + rand_simple() * 0.05;
                self.conceptual_experiments = 0.90 + rand_simple() * 0.10;
                self.cross_cultural_studies = 0.85 + rand_simple() * 0.14;
            },
            ExperimentalPhilosophyTopic::LinguisticExperiments => {
                self.empirical_methodology = 0.95 + rand_simple() * 0.05;
                self.intuition_analysis = 0.90 + rand_simple() * 0.10;
                self.cross_cultural_studies = 0.85 + rand_simple() * 0.14;
            },
            ExperimentalPhilosophyTopic::CognitiveHistoriography => {
                self.conceptual_experiments = 0.95 + rand_simple() * 0.05;
                self.empirical_methodology = 0.90 + rand_simple() * 0.10;
                self.intuition_analysis = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.intuition_analysis == 0.0 {
            self.intuition_analysis = (self.empirical_methodology + self.conceptual_experiments) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_cross_cultural() {
        let mut system = ExperimentalPhilosophySystem::new(ExperimentalPhilosophyTopic::IntuitionsCrossCultural);
        system.analyze_system().unwrap();
        assert!(system.empirical_methodology > 0.8);
    }
}