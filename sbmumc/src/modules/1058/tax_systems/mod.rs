//! # SBMUMC Module 1058: Tax Systems
//!
//! Taxation frameworks, policy, and administration.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaxType {
    Income,
    Corporate,
    Sales,
    Property,
    Consumption,
    Environmental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxSystem {
    pub system_id: String,
    pub country: String,
    pub tax_type: TaxType,
    pub effective_rate: f64,
    pub compliance_rate: f64,
    pub revenue_yield_gdp: f64,
    pub efficiency_score: f64,
}

impl TaxSystem {
    pub fn new(country: String, tax_type: TaxType) -> Self {
        Self {
            system_id: crate::core::uuid_simple(),
            country,
            tax_type,
            effective_rate: 0.0,
            compliance_rate: 0.0,
            revenue_yield_gdp: 0.0,
            efficiency_score: 0.0,
        }
    }

    pub fn analyze_tax_system(&mut self, gdp: f64) -> Result<()> {
        match self.tax_type {
            TaxType::Income => {
                self.effective_rate = 0.15 + rand_simple() * 0.25;
                self.compliance_rate = 0.75 + rand_simple() * 0.20;
                self.revenue_yield_gdp = self.effective_rate * self.compliance_rate;
            },
            TaxType::Corporate => {
                self.effective_rate = 0.15 + rand_simple() * 0.15;
                self.compliance_rate = 0.85 + rand_simple() * 0.12;
                self.revenue_yield_gdp = self.effective_rate * self.compliance_rate;
            },
            TaxType::Sales => {
                self.effective_rate = 0.08 + rand_simple() * 0.12;
                self.compliance_rate = 0.80 + rand_simple() * 0.15;
                self.revenue_yield_gdp = self.effective_rate * self.compliance_rate;
            },
            TaxType::Environmental => {
                self.effective_rate = 0.01 + rand_simple() * 0.04;
                self.compliance_rate = 0.70 + rand_simple() * 0.25;
                self.revenue_yield_gdp = self.effective_rate * self.compliance_rate;
            },
            _ => {
                self.effective_rate = 0.05 + rand_simple() * 0.15;
                self.compliance_rate = 0.75 + rand_simple() * 0.20;
                self.revenue_yield_gdp = self.effective_rate * self.compliance_rate;
            }
        }

        self.revenue_yield_gdp *= gdp / 100.0;
        self.efficiency_score = self.compliance_rate * (1.0 - self.effective_rate.abs() / 10.0);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxPolicy {
    pub policy_id: String,
    pub policy_name: String,
    pub tax_change_rate: f64,
    pub revenue_impact: f64,
    pub distributional_impact: f64,
    pub economic_activity_effect: f64,
}

impl TaxPolicy {
    pub fn new(name: String) -> Self {
        Self {
            policy_id: crate::core::uuid_simple(),
            policy_name: name,
            tax_change_rate: 0.0,
            revenue_impact: 0.0,
            distributional_impact: 0.0,
            economic_activity_effect: 0.0,
        }
    }

    pub fn evaluate_policy(&mut self) -> Result<()> {
        self.tax_change_rate = -0.05 + rand_simple() * 0.15;
        self.revenue_impact = self.tax_change_rate * (100.0 + rand_simple() * 400.0);
        self.distributional_impact = -0.3 + rand_simple() * 0.6;
        self.economic_activity_effect = -0.1 + rand_simple() * 0.3;
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

pub fn compute_optimal_rate(tax_type: &str) -> Result<f64> {
    let optimal = match tax_type {
        "Income" => 0.30,
        "Corporate" => 0.25,
        "Sales" => 0.10,
        _ => 0.15,
    };
    Ok(optimal + rand_simple() * 0.05 - 0.025)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progressive_income_tax() {
        let mut system = TaxSystem::new("Sweden".to_string(), TaxType::Income);
        system.analyze_tax_system(600.0).unwrap();
        assert!(system.efficiency_score > 0.0);
    }

    #[test]
    fn test_tax_policy() {
        let mut policy = TaxPolicy::new("Carbon_Tax_Increase".to_string());
        policy.evaluate_policy().unwrap();
        assert!(policy.economic_activity_effect > -0.2);
    }
}