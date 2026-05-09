//! # SBMUMC Module 1079: Experience Economy
//!
//! Economics of experiences and memorable events.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperienceType {
    Entertainment,
    Education,
    Escapism,
    Esthetic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceEconomyModel {
    pub model_id: String,
    pub experience_type: ExperienceType,
    pub experience_price_point: f64,
    pub customer_satisfaction: f64,
    var memorable_factor: f64,
    pub price_premium_percent: f64,
    pub repeat_customer_rate: f64,
}

impl ExperienceEconomyModel {
    pub fn new(experience_type: ExperienceType) -> Self {
        Self {
            model_id: crate::core::uuid_simple(),
            experience_type,
            experience_price_point: 0.0,
            customer_satisfaction: 0.0,
            var memorable_factor: 0.0,
            price_premium_percent: 0.0,
            repeat_customer_rate: 0.0,
        }
    }

    pub fn evaluate_experience(&mut self, base_cost: f64) -> Result<()> {
        match self.experience_type {
            ExperienceType::Entertainment => {
                self.experience_price_point = base_cost * (1.5 + rand_simple() * 1.0);
                self.customer_satisfaction = 0.75 + rand_simple() * 0.20;
            },
            ExperienceType::Education => {
                self.experience_price_point = base_cost * (2.0 + rand_simple() * 1.5);
                self.customer_satisfaction = 0.80 + rand_simple() * 0.15;
            },
            ExperienceType::Escapism => {
                self.experience_price_point = base_cost * (3.0 + rand_simple() * 2.0);
                self.customer_satisfaction = 0.85 + rand_simple() * 0.12;
            },
            ExperienceType::Esthetic => {
                self.experience_price_point = base_cost * (2.5 + rand_simple() * 2.5);
                self.customer_satisfaction = 0.82 + rand_simple() * 0.15;
            }
        }

        self.memorable_factor = self.customer_satisfaction * (0.8 + rand_simple() * 0.4);
        self.price_premium_percent = (self.experience_price_point / base_cost - 1.0) * 100.0;
        self.repeat_customer_rate = self.customer_satisfaction * (0.5 + rand_simple() * 0.4);
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

pub fn compute_experience_value_added(price: f64, memorable: f64) -> Result<f64> {
    Ok(price * memorable * 1.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escapism_experience() {
        let mut model = ExperienceEconomyModel::new(ExperienceType::Escapism);
        model.evaluate_experience(100.0).unwrap();
        assert!(model.price_premium_percent > 100.0);
    }
}