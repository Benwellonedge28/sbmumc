//! # SBMUMC Module 1446: Philosophy of Mind
//!
//! Systems for philosophy of mind and mental causation.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MentalPhenomena {
    Intentionality,
    Qualia,
    MentalCausation,
    Representation,
    Phenomenology,
    MentalUnity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyMindSystem {
    pub system_id: String,
    pub mental_phenomena: MentalPhenomena,
    pub mental_content: f64,
    pub phenomenal_experience: f64,
    pub causal_efficacy: f64,
    pub mental_representation: f64,
}

impl PhilosophyMindSystem {
    pub fn new(mental_phenomena: MentalPhenomena) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            mental_phenomena,
            mental_content: 0.0,
            phenomenal_experience: 0.0,
            causal_efficacy: 0.0,
            mental_representation: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.mental_phenomena {
            MentalPhenomena::Intentionality => {
                self.mental_content = 0.95 + rand_simple() * 0.05;
                self.phenomenal_experience = 0.90 + rand_simple() * 0.10;
                self.causal_efficacy = 0.85 + rand_simple() * 0.14;
            },
            MentalPhenomena::Qualia => {
                self.mental_representation = 0.95 + rand_simple() * 0.05;
                self.mental_content = 0.90 + rand_simple() * 0.10;
                self.phenomenal_experience = 0.85 + rand_simple() * 0.14;
            },
            MentalPhenomena::MentalCausation => {
                self.causal_efficacy = 0.95 + rand_simple() * 0.05;
                self.mental_representation = 0.90 + rand_simple() * 0.10;
                self.mental_content = 0.85 + rand_simple() * 0.14;
            },
            MentalPhenomena::Representation => {
                self.phenomenal_experience = 0.95 + rand_simple() * 0.05;
                self.causal_efficacy = 0.90 + rand_simple() * 0.10;
                self.mental_representation = 0.85 + rand_simple() * 0.14;
            },
            MentalPhenomena::Phenomenology => {
                self.mental_content = 0.95 + rand_simple() * 0.05;
                self.phenomenal_experience = 0.90 + rand_simple() * 0.10;
                self.causal_efficacy = 0.85 + rand_simple() * 0.14;
            },
            MentalPhenomena::MentalUnity => {
                self.mental_representation = 0.95 + rand_simple() * 0.05;
                self.causal_efficacy = 0.90 + rand_simple() * 0.10;
                self.mental_content = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.causal_efficacy == 0.0 {
            self.causal_efficacy = (self.mental_content + self.phenomenal_experience) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_intentionality() {
        let mut system = PhilosophyMindSystem::new(MentalPhenomena::Intentionality);
        system.analyze_system().unwrap();
        assert!(system.mental_content > 0.8);
    }
}