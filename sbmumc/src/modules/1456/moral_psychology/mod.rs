//! # SBMUMC Module 1456: Moral Psychology
//!
//! Systems for moral psychology and ethical development.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MoralPsychologyTopic {
    MoralEmotions,
    MoralReasoning,
    MoralIdentity,
    Altruism,
    MoralDevelopment,
    MoralIntuition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralPsychologySystem {
    pub system_id: String,
    pub moral_psychology_topic: MoralPsychologyTopic,
    pub moral_sentiments: f64,
    pub ethical_cognition: f64,
    pub moral_character: f64,
    pub moral_action: f64,
}

impl MoralPsychologySystem {
    pub fn new(moral_psychology_topic: MoralPsychologyTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            moral_psychology_topic,
            moral_sentiments: 0.0,
            ethical_cognition: 0.0,
            moral_character: 0.0,
            moral_action: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.moral_psychology_topic {
            MoralPsychologyTopic::MoralEmotions => {
                self.moral_sentiments = 0.95 + rand_simple() * 0.05;
                self.ethical_cognition = 0.90 + rand_simple() * 0.10;
                self.moral_character = 0.85 + rand_simple() * 0.14;
            },
            MoralPsychologyTopic::MoralReasoning => {
                self.moral_action = 0.95 + rand_simple() * 0.05;
                self.moral_sentiments = 0.90 + rand_simple() * 0.10;
                self.ethical_cognition = 0.85 + rand_simple() * 0.14;
            },
            MoralPsychologyTopic::MoralIdentity => {
                self.moral_character = 0.95 + rand_simple() * 0.05;
                self.moral_action = 0.90 + rand_simple() * 0.10;
                self.moral_sentiments = 0.85 + rand_simple() * 0.14;
            },
            MoralPsychologyTopic::Altruism => {
                self.ethical_cognition = 0.95 + rand_simple() * 0.05;
                self.moral_character = 0.90 + rand_simple() * 0.10;
                self.moral_action = 0.85 + rand_simple() * 0.14;
            },
            MoralPsychologyTopic::MoralDevelopment => {
                self.moral_sentiments = 0.95 + rand_simple() * 0.05;
                self.moral_action = 0.90 + rand_simple() * 0.10;
                self.moral_character = 0.85 + rand_simple() * 0.14;
            },
            MoralPsychologyTopic::MoralIntuition => {
                self.ethical_cognition = 0.95 + rand_simple() * 0.05;
                self.moral_sentiments = 0.90 + rand_simple() * 0.10;
                self.moral_character = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.moral_character == 0.0 {
            self.moral_character = (self.moral_sentiments + self.ethical_cognition) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_emotions() {
        let mut system = MoralPsychologySystem::new(MoralPsychologyTopic::MoralEmotions);
        system.analyze_system().unwrap();
        assert!(system.moral_sentiments > 0.8);
    }
}