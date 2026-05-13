//! # SBMUMC Module 1469: History of Philosophy
//!
//! Systems for history of philosophy and philosophical traditions.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhilosophyHistoricalPeriod {
    AncientGreek,
    MedievalPhilosophy,
    EarlyModern,
    GermanIdealism,
    NineteenthCentury,
    TwentiethCentury,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyHistorySystem {
    pub system_id: String,
    pub philosophy_historical_period: PhilosophyHistoricalPeriod,
    pub philosophical_context: f64,
    pub conceptual_development: f64,
    pub influence_tracing: f64,
    pub text_interpretation: f64,
}

impl PhilosophyHistorySystem {
    pub fn new(philosophy_historical_period: PhilosophyHistoricalPeriod) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            philosophy_historical_period,
            philosophical_context: 0.0,
            conceptual_development: 0.0,
            influence_tracing: 0.0,
            text_interpretation: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.philosophy_historical_period {
            PhilosophyHistoricalPeriod::AncientGreek => {
                self.philosophical_context = 0.95 + rand_simple() * 0.05;
                self.conceptual_development = 0.90 + rand_simple() * 0.10;
                self.influence_tracing = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyHistoricalPeriod::MedievalPhilosophy => {
                self.text_interpretation = 0.95 + rand_simple() * 0.05;
                self.philosophical_context = 0.90 + rand_simple() * 0.10;
                self.conceptual_development = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyHistoricalPeriod::EarlyModern => {
                self.influence_tracing = 0.95 + rand_simple() * 0.05;
                self.text_interpretation = 0.90 + rand_simple() * 0.10;
                self.philosophical_context = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyHistoricalPeriod::GermanIdealism => {
                self.conceptual_development = 0.95 + rand_simple() * 0.05;
                self.influence_tracing = 0.90 + rand_simple() * 0.10;
                self.text_interpretation = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyHistoricalPeriod::NineteenthCentury => {
                self.philosophical_context = 0.95 + rand_simple() * 0.05;
                self.conceptual_development = 0.90 + rand_simple() * 0.10;
                self.text_interpretation = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyHistoricalPeriod::TwentiethCentury => {
                self.text_interpretation = 0.95 + rand_simple() * 0.05;
                self.philosophical_context = 0.90 + rand_simple() * 0.10;
                self.influence_tracing = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.influence_tracing == 0.0 {
            self.influence_tracing = (self.philosophical_context + self.conceptual_development) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_ancient_greek() {
        let mut system = PhilosophyHistorySystem::new(PhilosophyHistoricalPeriod::AncientGreek);
        system.analyze_system().unwrap();
        assert!(system.philosophical_context > 0.8);
    }
}