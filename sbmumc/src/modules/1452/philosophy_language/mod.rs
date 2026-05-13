//! # SBMUMC Module 1452: Philosophy of Language
//!
//! Systems for philosophy of language and meaning.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LanguageTheory {
    ReferenceTheories,
    UseTheories,
    TruthTheories,
    SpeechActs,
    MeaningHolism,
    ContextualismLang,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyLanguageSystem {
    pub system_id: String,
    pub language_theory: LanguageTheory,
    pub meaning_theories: f64,
    pub reference_mechanisms: f64,
    pub use_theory_analysis: f64,
    pub communication_logic: f64,
}

impl PhilosophyLanguageSystem {
    pub fn new(language_theory: LanguageTheory) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            language_theory,
            meaning_theories: 0.0,
            reference_mechanisms: 0.0,
            use_theory_analysis: 0.0,
            communication_logic: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.language_theory {
            LanguageTheory::ReferenceTheories => {
                self.meaning_theories = 0.95 + rand_simple() * 0.05;
                self.reference_mechanisms = 0.90 + rand_simple() * 0.10;
                self.use_theory_analysis = 0.85 + rand_simple() * 0.14;
            },
            LanguageTheory::UseTheories => {
                self.communication_logic = 0.95 + rand_simple() * 0.05;
                self.meaning_theories = 0.90 + rand_simple() * 0.10;
                self.reference_mechanisms = 0.85 + rand_simple() * 0.14;
            },
            LanguageTheory::TruthTheories => {
                self.reference_mechanisms = 0.95 + rand_simple() * 0.05;
                self.communication_logic = 0.90 + rand_simple() * 0.10;
                self.meaning_theories = 0.85 + rand_simple() * 0.14;
            },
            LanguageTheory::SpeechActs => {
                self.use_theory_analysis = 0.95 + rand_simple() * 0.05;
                self.reference_mechanisms = 0.90 + rand_simple() * 0.10;
                self.communication_logic = 0.85 + rand_simple() * 0.14;
            },
            LanguageTheory::MeaningHolism => {
                self.meaning_theories = 0.95 + rand_simple() * 0.05;
                self.communication_logic = 0.90 + rand_simple() * 0.10;
                self.use_theory_analysis = 0.85 + rand_simple() * 0.14;
            },
            LanguageTheory::ContextualismLang => {
                self.reference_mechanisms = 0.95 + rand_simple() * 0.05;
                self.use_theory_analysis = 0.90 + rand_simple() * 0.10;
                self.meaning_theories = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.use_theory_analysis == 0.0 {
            self.use_theory_analysis = (self.meaning_theories + self.reference_mechanisms) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_reference() {
        let mut system = PhilosophyLanguageSystem::new(LanguageTheory::ReferenceTheories);
        system.analyze_system().unwrap();
        assert!(system.meaning_theories > 0.8);
    }
}