//! # SBMUMC Module 1149: Early Childhood Education
//!
//! Development and learning approaches for young children.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EarlyChildhoodApproach {
    Montessori,
    ReggioEmilia,
    Waldorf,
    HighScope,
    BankStreet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyChildhoodFramework {
    pub framework_id: String,
    pub approach: EarlyChildhoodApproach,
    pub developmental_alignment: f64,
    pub play_based_learning: f64,
    pub social_emotional_development: f64,
    pub parental_engagement: f64,
}

impl EarlyChildhoodFramework {
    pub fn new(approach: EarlyChildhoodApproach) -> Self {
        Self {
            framework_id: crate::core::uuid_simple(),
            approach,
            developmental_alignment: 0.0,
            play_based_learning: 0.0,
            social_emotional_development: 0.0,
            parental_engagement: 0.0,
        }
    }

    pub fn analyze_framework(&mut self) -> Result<()> {
        match self.approach {
            EarlyChildhoodApproach::Montessori => {
                self.developmental_alignment = 0.85 + rand_simple() * 0.14;
                self.play_based_learning = 0.75 + rand_simple() * 0.22;
            },
            EarlyChildhoodApproach::ReggioEmilia => {
                self.developmental_alignment = 0.80 + rand_simple() * 0.18;
                self.play_based_learning = 0.85 + rand_simple() * 0.14;
            },
            EarlyChildhoodApproach::Waldorf => {
                self.developmental_alignment = 0.75 + rand_simple() * 0.22;
                self.play_based_learning = 0.80 + rand_simple() * 0.18;
            },
            _ => {
                self.developmental_alignment = 0.70 + rand_simple() * 0.25;
                self.play_based_learning = 0.65 + rand_simple() * 0.30;
            },
        }

        self.social_emotional_development = self.developmental_alignment * (0.8 + rand_simple() * 0.2);
        self.parental_engagement = 0.55 + rand_simple() * 0.40;
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
    fn test_montessori_approach() {
        let mut framework = EarlyChildhoodFramework::new(EarlyChildhoodApproach::Montessori);
        framework.analyze_framework().unwrap();
        assert!(framework.developmental_alignment > 0.7);
    }
}