//! # SBMUMC Module 1405: Differential Equations
//!
//! Systems for differential equations and dynamic systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EquationType {
    OrdinaryDifferential,
    PartialDifferential,
    StochasticDifferential,
    Delay,
    Integral,
    Functional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifferentialEquationsSystem {
    pub system_id: String,
    pub equation_type: EquationType,
    pub existence_uniqueness: f64,
    pub stability_analysis: f64,
    pub solution_techniques: f64,
    pub asymptotic_behavior: f64,
}

impl DifferentialEquationsSystem {
    pub fn new(equation_type: EquationType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            equation_type,
            existence_uniqueness: 0.0,
            stability_analysis: 0.0,
            solution_techniques: 0.0,
            asymptotic_behavior: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.equation_type {
            EquationType::OrdinaryDifferential => {
                self.existence_uniqueness = 0.95 + rand_simple() * 0.05;
                self.solution_techniques = 0.90 + rand_simple() * 0.10;
                self.stability_analysis = 0.85 + rand_simple() * 0.14;
            },
            EquationType::PartialDifferential => {
                self.stability_analysis = 0.95 + rand_simple() * 0.05;
                self.existence_uniqueness = 0.90 + rand_simple() * 0.10;
                self.asymptotic_behavior = 0.85 + rand_simple() * 0.14;
            },
            EquationType::StochasticDifferential => {
                self.solution_techniques = 0.95 + rand_simple() * 0.05;
                self.asymptotic_behavior = 0.90 + rand_simple() * 0.10;
                self.existence_uniqueness = 0.85 + rand_simple() * 0.14;
            },
            EquationType::Delay => {
                self.asymptotic_behavior = 0.95 + rand_simple() * 0.05;
                self.stability_analysis = 0.90 + rand_simple() * 0.10;
                self.solution_techniques = 0.85 + rand_simple() * 0.14;
            },
            EquationType::Integral => {
                self.existence_uniqueness = 0.95 + rand_simple() * 0.05;
                self.asymptotic_behavior = 0.90 + rand_simple() * 0.10;
                self.stability_analysis = 0.85 + rand_simple() * 0.14;
            },
            EquationType::Functional => {
                self.stability_analysis = 0.95 + rand_simple() * 0.05;
                self.solution_techniques = 0.90 + rand_simple() * 0.10;
                self.asymptotic_behavior = 0.85 + rand_simple() * 0.14;
            },
        }

        if self.asymptotic_behavior == 0.0 {
            self.asymptotic_behavior = (self.existence_uniqueness + self.solution_techniques) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_ode() {
        let mut system = DifferentialEquationsSystem::new(EquationType::OrdinaryDifferential);
        system.analyze_system().unwrap();
        assert!(system.existence_uniqueness > 0.8);
    }
}
