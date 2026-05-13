//! # SBMUMC Module 1467: Global Justice
//!
//! Systems for global justice and international ethics.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GlobalJusticeTopic {
    DistributiveJustice,
    CosmopolitanismJustice,
    GlobalPoverty,
    InternationalLaw,
    HistoricalJustice,
    ClimateJustice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalJusticeSystem {
    pub system_id: String,
    pub global_justice_topic: GlobalJusticeTopic,
    pub justice_distribution: f64,
    pub moral_obligation: f64,
    pub global_fairness: f64,
    pub institutional_justice: f64,
}

impl GlobalJusticeSystem {
    pub fn new(global_justice_topic: GlobalJusticeTopic) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            global_justice_topic,
            justice_distribution: 0.0,
            moral_obligation: 0.0,
            global_fairness: 0.0,
            institutional_justice: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.global_justice_topic {
            GlobalJusticeTopic::DistributiveJustice => {
                self.justice_distribution = 0.95 + rand_simple() * 0.05;
                self.moral_obligation = 0.90 + rand_simple() * 0.10;
                self.global_fairness = 0.85 + rand_simple() * 0.14;
            },
            GlobalJusticeTopic::CosmopolitanismJustice => {
                self.institutional_justice = 0.95 + rand_simple() * 0.05;
                self.justice_distribution = 0.90 + rand_simple() * 0.10;
                self.moral_obligation = 0.85 + rand_simple() * 0.14;
            },
            GlobalJusticeTopic::GlobalPoverty => {
                self.global_fairness = 0.95 + rand_simple() * 0.05;
                self.institutional_justice = 0.90 + rand_simple() * 0.10;
                self.justice_distribution = 0.85 + rand_simple() * 0.14;
            },
            GlobalJusticeTopic::InternationalLaw => {
                self.moral_obligation = 0.95 + rand_simple() * 0.05;
                self.global_fairness = 0.90 + rand_simple() * 0.10;
                self.institutional_justice = 0.85 + rand_simple() * 0.14;
            },
            GlobalJusticeTopic::HistoricalJustice => {
                self.justice_distribution = 0.95 + rand_simple() * 0.05;
                self.global_fairness = 0.90 + rand_simple() * 0.10;
                self.moral_obligation = 0.85 + rand_simple() * 0.14;
            },
            GlobalJusticeTopic::ClimateJustice => {
                self.institutional_justice = 0.95 + rand_simple() * 0.05;
                self.moral_obligation = 0.90 + rand_simple() * 0.10;
                self.global_fairness = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.global_fairness == 0.0 {
            self.global_fairness = (self.justice_distribution + self.moral_obligation) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_distributive() {
        let mut system = GlobalJusticeSystem::new(GlobalJusticeTopic::DistributiveJustice);
        system.analyze_system().unwrap();
        assert!(system.justice_distribution > 0.8);
    }
}