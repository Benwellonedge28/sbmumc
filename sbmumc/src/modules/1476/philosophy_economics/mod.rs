//! # SBMUMC Module 1476: Philosophy of Economics
//!
//! Systems for philosophy of economics and economic methodology.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhilosophyEconomicsTopic {
    MethodologicalIndividualism,
    RationalChoice,
    EconomicExplanation,
    WelfareEconomics,
    MarketFundamentalism,
    EconomicMethodology,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyEconomicsSystem {
    pub system_id: String,
    pub philosophy_economics_topic: PhilosophyEconomicsTopic,
    pub economic_methodology: f64,
    pub explanatory_models: f64,
    pub normative_foundations: f64,
    pub assumption_reality: f64,
}

impl PhilosophyEconomicsSystem {
    pub fn new(philosophy_economics_topic: PhilosophyEconomicsTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            philosophy_economics_topic,
            economic_methodology: 0.0,
            explanatory_models: 0.0,
            normative_foundations: 0.0,
            assumption_reality: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.philosophy_economics_topic {
            PhilosophyEconomicsTopic::MethodologicalIndividualism => {
                self.economic_methodology = 0.95 + rand_simple() * 0.05;
                self.explanatory_models = 0.90 + rand_simple() * 0.10;
                self.normative_foundations = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyEconomicsTopic::RationalChoice => {
                self.assumption_reality = 0.95 + rand_simple() * 0.05;
                self.economic_methodology = 0.90 + rand_simple() * 0.10;
                self.explanatory_models = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyEconomicsTopic::EconomicExplanation => {
                self.normative_foundations = 0.95 + rand_simple() * 0.05;
                self.assumption_reality = 0.90 + rand_simple() * 0.10;
                self.economic_methodology = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyEconomicsTopic::WelfareEconomics => {
                self.explanatory_models = 0.95 + rand_simple() * 0.05;
                self.normative_foundations = 0.90 + rand_simple() * 0.10;
                self.assumption_reality = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyEconomicsTopic::MarketFundamentalism => {
                self.economic_methodology = 0.95 + rand_simple() * 0.05;
                self.assumption_reality = 0.90 + rand_simple() * 0.10;
                self.explanatory_models = 0.85 + rand_simple() * 0.14;
            },
            PhilosophyEconomicsTopic::EconomicMethodology => {
                self.normative_foundations = 0.95 + rand_simple() * 0.05;
                self.explanatory_models = 0.90 + rand_simple() * 0.10;
                self.assumption_reality = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.explanatory_models == 0.0 {
            self.explanatory_models = (self.economic_methodology + self.normative_foundations) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_individualism() {
        let mut system = PhilosophyEconomicsSystem::new(PhilosophyEconomicsTopic::MethodologicalIndividualism);
        system.analyze_system().unwrap();
        assert!(system.economic_methodology > 0.8);
    }
}