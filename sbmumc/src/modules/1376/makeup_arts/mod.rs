//! # SBMUMC Module 1376: Makeup Arts
//!
//! Systems for makeup and prosthetics in entertainment.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MakeupApplication {
    CharacterMakeup,
    SpecialEffects,
    Prosthetics,
    PeriodMakeup,
    ProportionAlteration,
    CreatureDesign,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakeupArtsSystem {
    pub system_id: String,
    pub makeup_application: MakeupApplication,
    pub artistic_skill: f64,
    pub technical_excellence: f64,
    pub character_transformation: f64,
    pub skin_compatibility: f64,
}

impl MakeupArtsSystem {
    pub fn new(makeup_application: MakeupApplication) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            makeup_application,
            artistic_skill: 0.0,
            technical_excellence: 0.0,
            character_transformation: 0.0,
            skin_compatibility: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.makeup_application {
            MakeupApplication::CharacterMakeup => {
                self.character_transformation = 0.95 + rand_simple() * 0.05;
                self.artistic_skill = 0.90 + rand_simple() * 0.10;
                self.skin_compatibility = 0.85 + rand_simple() * 0.14;
            },
            MakeupApplication::SpecialEffects => {
                self.technical_excellence = 0.95 + rand_simple() * 0.05;
                self.character_transformation = 0.90 + rand_simple() * 0.10;
                self.artistic_skill = 0.85 + rand_simple() * 0.14;
            },
            MakeupApplication::Prosthetics => {
                self.technical_excellence = 0.95 + rand_simple() * 0.05;
                self.skin_compatibility = 0.90 + rand_simple() * 0.10;
                self.character_transformation = 0.85 + rand_simple() * 0.14;
            },
            MakeupApplication::PeriodMakeup => {
                self.artistic_skill = 0.95 + rand_simple() * 0.05;
                self.character_transformation = 0.90 + rand_simple() * 0.10;
                self.skin_compatibility = 0.85 + rand_simple() * 0.14;
            },
            MakeupApplication::ProportionAlteration => {
                self.technical_excellence = 0.95 + rand_simple() * 0.05;
                self.artistic_skill = 0.90 + rand_simple() * 0.10;
                self.character_transformation = 0.85 + rand_simple() * 0.14;
            },
            MakeupApplication::CreatureDesign => {
                self.character_transformation = 0.95 + rand_simple() * 0.05;
                self.technical_excellence = 0.90 + rand_simple() * 0.10;
                self.artistic_skill = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.skin_compatibility == 0.0 {
            self.skin_compatibility = (self.artistic_skill + self.technical_excellence) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_prosthetics() {
        let mut system = MakeupArtsSystem::new(MakeupApplication::Prosthetics);
        system.analyze_system().unwrap();
        assert!(system.technical_excellence > 0.8);
    }
}