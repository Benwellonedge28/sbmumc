//! # SBMUMC Module 1218: Animal Husbandry
//!
//! Care and breeding of domestic animals for production.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HusbandryPractice {
    Breeding,
    Nutrition,
    Housing,
    HealthManagement,
    WelfareStandards,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimalHusbandrySystem {
    pub system_id: String,
    pub husbandry_practice: HusbandryPractice,
    pub productivity_score: f64,
    pub animal_welfare: f64,
    pub health_outcomes: f64,
    pub reproduction_efficiency: f64,
}

impl AnimalHusbandrySystem {
    pub fn new(husbandry_practice: HusbandryPractice) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            husbandry_practice,
            productivity_score: 0.0,
            animal_welfare: 0.0,
            health_outcomes: 0.0,
            reproduction_efficiency: 0.0,
        }
    }

    pub fn analyze_system(&mut self) -> Result<()> {
        match self.husbandry_practice {
            HusbandryPractice::Breeding => {
                self.reproduction_efficiency = 0.85 + rand_simple() * 0.14;
                self.productivity_score = 0.80 + rand_simple() * 0.18;
            },
            HusbandryPractice::Nutrition => {
                self.productivity_score = 0.85 + rand_simple() * 0.14;
                self.health_outcomes = 0.80 + rand_simple() * 0.18;
            },
            HusbandryPractice::Housing => {
                self.animal_welfare = 0.85 + rand_simple() * 0.14;
                self.health_outcomes = 0.75 + rand_simple() * 0.22;
            },
            HusbandryPractice::HealthManagement => {
                self.health_outcomes = 0.90 + rand_simple() * 0.10;
                self.animal_welfare = 0.75 + rand_simple() * 0.22;
            },
            HusbandryPractice::WelfareStandards => {
                self.animal_welfare = 0.90 + rand_simple() * 0.10;
                self.productivity_score = 0.70 + rand_simple() * 0.25;
            },
        }

        if self.reproduction_efficiency == 0.0 {
            self.reproduction_efficiency = (self.health_outcomes + self.animal_welfare) / 2.0 * (0.6 + rand_simple() * 0.3);
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
    fn test_breeding_practice() {
        let mut system = AnimalHusbandrySystem::new(HusbandryPractice::Breeding);
        system.analyze_system().unwrap();
        assert!(system.reproduction_efficiency > 0.6);
    }
}