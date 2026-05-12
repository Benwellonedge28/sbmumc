//! # SBMUMC Module 1378: Casting Directing
//!
//! Systems for casting and directing performers.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CastingContext {
    FeatureFilm,
    Television,
    Theater,
    Commercial,
    VoiceOver,
    Musical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastingDirectingSystem {
    pub system_id: String,
    pub casting_context: CastingContext,
    pub talent_identification: f64,
    pub character_fit: f64,
    pub ensemble_chemistry: f64,
    pub performance_coaching: f64,
}

impl CastingDirectingSystem {
    pub fn new(casting_context: CastingContext) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            casting_context,
            talent_identification: 0.0,
            character_fit: 0.0,
            ensemble_chemistry: 0.0,
            performance_coaching: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.casting_context {
            CastingContext::FeatureFilm => {
                self.character_fit = 0.95 + rand_simple() * 0.05;
                self.talent_identification = 0.90 + rand_simple() * 0.10;
                self.performance_coaching = 0.85 + rand_simple() * 0.14;
            },
            CastingContext::Television => {
                self.talent_identification = 0.95 + rand_simple() * 0.05;
                self.ensemble_chemistry = 0.90 + rand_simple() * 0.10;
                self.character_fit = 0.85 + rand_simple() * 0.14;
            },
            CastingContext::Theater => {
                self.character_fit = 0.95 + rand_simple() * 0.05;
                self.performance_coaching = 0.90 + rand_simple() * 0.10;
                self.ensemble_chemistry = 0.85 + rand_simple() * 0.14;
            },
            CastingContext::Commercial => {
                self.talent_identification = 0.95 + rand_simple() * 0.05;
                self.character_fit = 0.90 + rand_simple() * 0.10;
                self.ensemble_chemistry = 0.80 + rand_simple() * 0.18;
            },
            CastingContext::VoiceOver => {
                self.talent_identification = 0.95 + rand_simple() * 0.05;
                self.character_fit = 0.90 + rand_simple() * 0.10;
                self.performance_coaching = 0.85 + rand_simple() * 0.14;
            },
            CastingContext::Musical => {
                self.ensemble_chemistry = 0.95 + rand_simple() * 0.05;
                self.performance_coaching = 0.90 + rand_simple() * 0.10;
                self.character_fit = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.performance_coaching == 0.0 {
            self.performance_coaching = (self.character_fit + self.ensemble_chemistry) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_feature_film() {
        let mut system = CastingDirectingSystem::new(CastingContext::FeatureFilm);
        system.analyze_system().unwrap();
        assert!(system.character_fit > 0.8);
    }
}