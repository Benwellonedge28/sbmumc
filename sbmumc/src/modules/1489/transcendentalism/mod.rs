//! # SBMUMC Module 1489: Transcendentalism
//!
//! Systems for transcendentalism and transcendental arguments.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TranscendentalismTopic {
    KantianTranscendental,
    TranscendentalIdealism,
    ConditionsPossibility,
    SyntheticAPriori,
    TranscendentalDeduction,
    TranscendentalArguments,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscendentalismSystem {
    pub system_id: String,
    pub transcendentalism_topic: TranscendentalismTopic,
    pub transcendental_conditions: f64,
    pub conditions_possibility: f64,
    pub synthetic_apriori: f64,
    pub transcendental_structure: f64,
}

impl TranscendentalismSystem {
    pub fn new(transcendentalism_topic: TranscendentalismTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            transcendentalism_topic,
            transcendental_conditions: 0.0,
            conditions_possibility: 0.0,
            synthetic_apriori: 0.0,
            transcendental_structure: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.transcendentalism_topic {
            TranscendentalismTopic::KantianTranscendental => {
                self.transcendental_conditions = 0.95 + rand_simple() * 0.05;
                self.conditions_possibility = 0.90 + rand_simple() * 0.10;
                self.synthetic_apriori = 0.85 + rand_simple() * 0.14;
            },
            TranscendentalismTopic::TranscendentalIdealism => {
                self.transcendental_structure = 0.95 + rand_simple() * 0.05;
                self.transcendental_conditions = 0.90 + rand_simple() * 0.10;
                self.conditions_possibility = 0.85 + rand_simple() * 0.14;
            },
            TranscendentalismTopic::ConditionsPossibility => {
                self.synthetic_apriori = 0.95 + rand_simple() * 0.05;
                self.transcendental_structure = 0.90 + rand_simple() * 0.10;
                self.transcendental_conditions = 0.85 + rand_simple() * 0.14;
            },
            TranscendentalismTopic::SyntheticAPriori => {
                self.conditions_possibility = 0.95 + rand_simple() * 0.05;
                self.synthetic_apriori = 0.90 + rand_simple() * 0.10;
                self.transcendental_structure = 0.85 + rand_simple() * 0.14;
            },
            TranscendentalismTopic::TranscendentalDeduction => {
                self.transcendental_conditions = 0.95 + rand_simple() * 0.05;
                self.synthetic_apriori = 0.90 + rand_simple() * 0.10;
                self.conditions_possibility = 0.85 + rand_simple() * 0.14;
            },
            TranscendentalismTopic::TranscendentalArguments => {
                self.transcendental_structure = 0.95 + rand_simple() * 0.05;
                self.conditions_possibility = 0.90 + rand_simple() * 0.10;
                self.synthetic_apriori = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.synthetic_apriori == 0.0 {
            self.synthetic_apriori = (self.transcendental_conditions + self.conditions_possibility) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_kantian() {
        let mut system = TranscendentalismSystem::new(TranscendentalismTopic::KantianTranscendental);
        system.analyze_system().unwrap();
        assert!(system.transcendental_conditions > 0.8);
    }
}