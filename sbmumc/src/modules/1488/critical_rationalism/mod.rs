//! # SBMUMC Module 1488: Critical Rationalism
//!
//! Systems for critical rationalism and falsificationism.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CriticalRationalismTopic {
    Falsificationism,
    CriticalDualism,
    RationalCriticism,
    ConjecturesRefutations,
    EpistemicHumility,
    EvolutionaryEpistemology,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalRationalismSystem {
    pub system_id: String,
    pub critical_rationalism_topic: CriticalRationalismTopic,
    pub falsifiability: f64,
    pub rational_criticism: f64,
    pub conjecture_hypothesis: f64,
    pub epistemic_growth: f64,
}

impl CriticalRationalismSystem {
    pub fn new(critical_rationalism_topic: CriticalRationalismTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            critical_rationalism_topic,
            falsifiability: 0.0,
            rational_criticism: 0.0,
            conjecture_hypothesis: 0.0,
            epistemic_growth: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.critical_rationalism_topic {
            CriticalRationalismTopic::Falsificationism => {
                self.falsifiability = 0.95 + rand_simple() * 0.05;
                self.rational_criticism = 0.90 + rand_simple() * 0.10;
                self.conjecture_hypothesis = 0.85 + rand_simple() * 0.14;
            },
            CriticalRationalismTopic::CriticalDualism => {
                self.epistemic_growth = 0.95 + rand_simple() * 0.05;
                self.falsifiability = 0.90 + rand_simple() * 0.10;
                self.rational_criticism = 0.85 + rand_simple() * 0.14;
            },
            CriticalRationalismTopic::RationalCriticism => {
                self.conjecture_hypothesis = 0.95 + rand_simple() * 0.05;
                self.epistemic_growth = 0.90 + rand_simple() * 0.10;
                self.falsifiability = 0.85 + rand_simple() * 0.14;
            },
            CriticalRationalismTopic::ConjecturesRefutations => {
                self.rational_criticism = 0.95 + rand_simple() * 0.05;
                self.conjecture_hypothesis = 0.90 + rand_simple() * 0.10;
                self.epistemic_growth = 0.85 + rand_simple() * 0.14;
            },
            CriticalRationalismTopic::EpistemicHumility => {
                self.falsifiability = 0.95 + rand_simple() * 0.05;
                self.rational_criticism = 0.90 + rand_simple() * 0.10;
                self.epistemic_growth = 0.85 + rand_simple() * 0.14;
            },
            CriticalRationalismTopic::EvolutionaryEpistemology => {
                self.conjecture_hypothesis = 0.95 + rand_simple() * 0.05;
                self.falsifiability = 0.90 + rand_simple() * 0.10;
                self.rational_criticism = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.conjecture_hypothesis == 0.0 {
            self.conjecture_hypothesis = (self.falsifiability + self.rational_criticism) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_falsificationism() {
        let mut system = CriticalRationalismSystem::new(CriticalRationalismTopic::Falsificationism);
        system.analyze_system().unwrap();
        assert!(system.falsifiability > 0.8);
    }
}