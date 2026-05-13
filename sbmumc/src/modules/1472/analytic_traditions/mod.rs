//! # SBMUMC Module 1472: Analytic Philosophy
//!
//! Systems for analytic philosophy and linguistic philosophy.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalyticTradition {
    EarlyAnalytic,
    LogicalPositivism,
    OrdinaryLanguage,
    QuineanNaturalism,
    ModalAnalytic,
    EpistemologyNaturalized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticTraditionsSystem {
    pub system_id: String,
    pub analytic_tradition: AnalyticTradition,
    pub logical_analysis: f64,
    pub conceptual_clarity: f64,
    pub argument_reconstruction: f64,
    pub verification_methods: f64,
}

impl AnalyticTraditionsSystem {
    pub fn new(analytic_tradition: AnalyticTradition) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            analytic_tradition,
            logical_analysis: 0.0,
            conceptual_clarity: 0.0,
            argument_reconstruction: 0.0,
            verification_methods: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.analytic_tradition {
            AnalyticTradition::EarlyAnalytic => {
                self.logical_analysis = 0.95 + rand_simple() * 0.05;
                self.conceptual_clarity = 0.90 + rand_simple() * 0.10;
                self.argument_reconstruction = 0.85 + rand_simple() * 0.14;
            },
            AnalyticTradition::LogicalPositivism => {
                self.verification_methods = 0.95 + rand_simple() * 0.05;
                self.logical_analysis = 0.90 + rand_simple() * 0.10;
                self.conceptual_clarity = 0.85 + rand_simple() * 0.14;
            },
            AnalyticTradition::OrdinaryLanguage => {
                self.argument_reconstruction = 0.95 + rand_simple() * 0.05;
                self.verification_methods = 0.90 + rand_simple() * 0.10;
                self.logical_analysis = 0.85 + rand_simple() * 0.14;
            },
            AnalyticTradition::QuineanNaturalism => {
                self.conceptual_clarity = 0.95 + rand_simple() * 0.05;
                self.argument_reconstruction = 0.90 + rand_simple() * 0.10;
                self.verification_methods = 0.85 + rand_simple() * 0.14;
            },
            AnalyticTradition::ModalAnalytic => {
                self.logical_analysis = 0.95 + rand_simple() * 0.05;
                self.conceptual_clarity = 0.90 + rand_simple() * 0.10;
                self.verification_methods = 0.85 + rand_simple() * 0.14;
            },
            AnalyticTradition::EpistemologyNaturalized => {
                self.argument_reconstruction = 0.95 + rand_simple() * 0.05;
                self.logical_analysis = 0.90 + rand_simple() * 0.10;
                self.conceptual_clarity = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.argument_reconstruction == 0.0 {
            self.argument_reconstruction = (self.logical_analysis + self.conceptual_clarity) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_early_analytic() {
        let mut system = AnalyticTraditionsSystem::new(AnalyticTradition::EarlyAnalytic);
        system.analyze_system().unwrap();
        assert!(system.logical_analysis > 0.8);
    }
}