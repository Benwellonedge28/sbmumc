//! # SBMUMC Module 1077: Circular Economy
//!
//! Circular resource flows and waste reduction systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CircularStrategy {
    Reduce,
    Reuse,
    Recycle,
    Recover,
    Remanufacture,
    Redesign,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircularEconomySystem {
    pub system_id: String,
    pub circular_strategy: CircularStrategy,
    pub material_circularity_index: f64,
    pub waste_diversion_rate: f64,
    var resource_efficiency_gain: f64,
    pub economic_value_creation: f64,
    pub environmental_benefit_score: f64,
}

impl CircularEconomySystem {
    pub fn new(strategy: CircularStrategy) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            circular_strategy: strategy,
            material_circularity_index: 0.0,
            waste_diversion_rate: 0.0,
            var resource_efficiency_gain: 0.0,
            economic_value_creation: 0.0,
            environmental_benefit_score: 0.0,
        }
    }

    pub fn assess_system(&mut self) -> Result<()> {
        match self.circular_strategy {
            CircularStrategy::Reduce => {
                self.material_circularity_index = 0.3 + rand_simple() * 0.3;
                self.waste_diversion_rate = 0.5 + rand_simple() * 0.3;
                self.resource_efficiency_gain = 0.20 + rand_simple() * 0.30;
            },
            CircularStrategy::Recycle => {
                self.material_circularity_index = 0.4 + rand_simple() * 0.3;
                self.waste_diversion_rate = 0.6 + rand_simple() * 0.25;
                self.resource_efficiency_gain = 0.15 + rand_simple() * 0.25;
            },
            CircularStrategy::Remanufacture => {
                self.material_circularity_index = 0.6 + rand_simple() * 0.25;
                self.waste_diversion_rate = 0.75 + rand_simple() * 0.20;
                self.resource_efficiency_gain = 0.30 + rand_simple() * 0.35;
            },
            _ => {
                self.material_circularity_index = 0.35 + rand_simple() * 0.35;
                self.waste_diversion_rate = 0.5 + rand_simple() * 0.35;
                self.resource_efficiency_gain = 0.15 + rand_simple() * 0.30;
            }
        }

        self.economic_value_creation = self.material_circularity_index * 1e9 * (0.5 + rand_simple());
        self.environmental_benefit_score = self.waste_diversion_rate * (1.0 - self.material_circularity_index).recip_or(0.5);
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

pub fn compute_circularity_potential(sector: &str) -> Result<f64> {
    let potential = match sector {
        "Electronics" => 0.4,
        "Textiles" => 0.5,
        "Construction" => 0.6,
        _ => 0.35,
    };
    Ok(potential + rand_simple() * 0.3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remanufacturing_system() {
        let mut system = CircularEconomySystem::new(CircularStrategy::Remanufacture);
        system.assess_system().unwrap();
        assert!(system.material_circularity_index > 0.5);
    }
}

trait RecipOr<T> {
    fn recip_or(self, or: T) -> T;
}

impl<T: From<f64>> RecipOr<T> for f64 {
    fn recip_or(self, or: T) -> T {
        if self != 0.0 {
            T::from(1.0 / self)
        } else {
            or
        }
    }
}