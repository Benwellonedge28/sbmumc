//! # SBMUMC Module 1530: Psychic Development
//!
//! Systems for psychic development and abilities.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PsychicDevelopmentTopic {
    PsychicAwakening,
    Clairvoyance,
    Telepathy,
    PsychicSensing,
    PsychicAbilities,
    IntuitivePowers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychicDevelopmentSystem {
    pub system_id: String,
    pub psychic_development_topic: PsychicDevelopmentTopic,
    pub psychic_powers: f64,
    pub intuitive_ability: f64,
    pub extrasensory_perception: f64,
    pub mental_clarity: f64,
}

impl PsychicDevelopmentSystem {
    pub fn new(psychic_development_topic: PsychicDevelopmentTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            psychic_development_topic,
            psychic_powers: 0.0,
            intuitive_ability: 0.0,
            extrasensory_perception: 0.0,
            mental_clarity: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.psychic_development_topic {
            PsychicDevelopmentTopic::PsychicAwakening => {
                self.psychic_powers = 0.95 + rand_simple() * 0.05;
                self.intuitive_ability = 0.90 + rand_simple() * 0.10;
                self.extrasensory_perception = 0.85 + rand_simple() * 0.14;
            },
            PsychicDevelopmentTopic::Clairvoyance => {
                self.mental_clarity = 0.95 + rand_simple() * 0.05;
                self.extrasensory_perception = 0.90 + rand_simple() * 0.10;
                self.intuitive_ability = 0.85 + rand_simple() * 0.14;
            },
            PsychicDevelopmentTopic::Telepathy => {
                self.intuitive_ability = 0.95 + rand_simple() * 0.05;
                self.psychic_powers = 0.90 + rand_simple() * 0.10;
                self.mental_clarity = 0.85 + rand_simple() * 0.14;
            },
            PsychicDevelopmentTopic::PsychicSensing => {
                self.extrasensory_perception = 0.95 + rand_simple() * 0.05;
                self.mental_clarity = 0.90 + rand_simple() * 0.10;
                self.psychic_powers = 0.85 + rand_simple() * 0.14;
            },
            PsychicDevelopmentTopic::PsychicAbilities => {
                self.psychic_powers = 0.95 + rand_simple() * 0.05;
                self.intuitive_ability = 0.90 + rand_simple() * 0.10;
                self.mental_clarity = 0.85 + rand_simple() * 0.14;
            },
            PsychicDevelopmentTopic::IntuitivePowers => {
                self.mental_clarity = 0.95 + rand_simple() * 0.05;
                self.extrasensory_perception = 0.90 + rand_simple() * 0.10;
                self.intuitive_ability = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.extrasensory_perception == 0.0 {
            self.extrasensory_perception = (self.psychic_powers + self.intuitive_ability) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_psychic_awakening() {
        let mut system = PsychicDevelopmentSystem::new(PsychicDevelopmentTopic::PsychicAwakening);
        system.analyze_system().unwrap();
        assert!(system.psychic_powers > 0.8);
    }
}