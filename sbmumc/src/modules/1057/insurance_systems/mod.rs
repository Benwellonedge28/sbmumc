//! # SBMUMC Module 1057: Insurance Systems
//!
//! Insurance products, risk pooling, and actuarial systems.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsuranceType {
    Life,
    Health,
    Property,
    Liability,
    Casualty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceProduct {
    pub product_id: String,
    pub insurance_type: InsuranceType,
    pub product_name: String,
    pub premium_rate: f64,
    pub coverage_amount: f64,
    pub claims_ratio: f64,
    pub customer_retention_rate: f64,
}

impl InsuranceProduct {
    pub fn new(insurance_type: InsuranceType, name: String) -> Self {
        Self {
            product_id: crate::core::uuid_simple(),
            insurance_type,
            product_name: name,
            premium_rate: 0.0,
            coverage_amount: 0.0,
            claims_ratio: 0.0,
            customer_retention_rate: 0.0,
        }
    }

    pub fn price_product(&mut self, coverage: f64) -> Result<()> {
        self.coverage_amount = coverage;

        match self.insurance_type {
            InsuranceType::Life => {
                self.premium_rate = coverage * 0.005 + rand_simple() * coverage * 0.003;
                self.claims_ratio = 0.7 + rand_simple() * 0.25;
            },
            InsuranceType::Health => {
                self.premium_rate = coverage * 0.02 + rand_simple() * coverage * 0.015;
                self.claims_ratio = 0.8 + rand_simple() * 0.15;
            },
            InsuranceType::Property => {
                self.premium_rate = coverage * 0.003 + rand_simple() * coverage * 0.004;
                self.claims_ratio = 0.5 + rand_simple() * 0.35;
            },
            _ => {
                self.premium_rate = coverage * 0.01 + rand_simple() * coverage * 0.01;
                self.claims_ratio = 0.6 + rand_simple() * 0.30;
            }
        }

        self.customer_retention_rate = 0.75 + rand_simple() * 0.20;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActuarialAssessment {
    pub assessment_id: String,
    pub risk_category: String,
    pub expected_loss: f64,
    pub loss_variance: f64,
    pub risk_charge: f64,
    pub capital_requirement: f64,
}

impl ActuarialAssessment {
    pub fn new(category: String) -> Self {
        Self {
            assessment_id: crate::core::uuid_simple(),
            risk_category: category,
            expected_loss: 0.0,
            loss_variance: 0.0,
            risk_charge: 0.0,
            capital_requirement: 0.0,
        }
    }

    pub fn compute_actuarial(&mut self) -> Result<()> {
        self.expected_loss = 1000.0 + rand_simple() * 9000.0;
        self.loss_variance = self.expected_loss * (0.5 + rand_simple() * 1.5);
        self.risk_charge = self.loss_variance.sqrt() * 2.0;
        self.capital_requirement = self.expected_loss + self.risk_charge;
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

pub fn compute_premium_rate(insurance_type: InsuranceType, risk: f64) -> Result<f64> {
    let base_rate = match insurance_type {
        InsuranceType::Life => 0.005,
        InsuranceType::Health => 0.02,
        InsuranceType::Property => 0.004,
        _ => 0.01,
    };
    Ok(base_rate * (1.0 + risk * 0.5))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_life_insurance() {
        let mut product = InsuranceProduct::new(
            InsuranceType::Life,
            "Term_Life_20_Year".to_string(),
        );
        product.price_product(500000.0).unwrap();
        assert!(product.premium_rate > 0.0);
    }

    #[test]
    fn test_actuarial_assessment() {
        let mut assessment = ActuarialAssessment::new("Auto_Collision".to_string());
        assessment.compute_actuarial().unwrap();
        assert!(assessment.capital_requirement > assessment.expected_loss);
    }
}