//! # SBMUMC Module 1059: Fiscal Policy
//!
//! Government fiscal policy, budgets, and public finance.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FiscalStance {
    Expansionary,
    Neutral,
    Contractionary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiscalPolicy {
    pub policy_id: String,
    pub government: String,
    pub fiscal_stance: FiscalStance,
    pub spending_gdp_ratio: f64,
    pub revenue_gdp_ratio: f64,
    pub deficit_gdp_ratio: f64,
    pub multiplier_effect: f64,
}

impl FiscalPolicy {
    pub fn new(government: String) -> Self {
        Self {
            policy_id: crate::core::uuid_simple(),
            government,
            fiscal_stance: FiscalStance::Neutral,
            spending_gdp_ratio: 0.0,
            revenue_gdp_ratio: 0.0,
            deficit_gdp_ratio: 0.0,
            multiplier_effect: 0.0,
        }
    }

    pub fn assess_fiscal_policy(&mut self, gdp: f64) -> Result<()> {
        self.spending_gdp_ratio = 0.25 + rand_simple() * 0.20;
        self.revenue_gdp_ratio = 0.22 + rand_simple() * 0.18;
        self.deficit_gdp_ratio = self.spending_gdp_ratio - self.revenue_gdp_ratio;

        if self.deficit_gdp_ratio > 0.03 {
            self.fiscal_stance = FiscalStance::Expansionary;
        } else if self.deficit_gdp_ratio < -0.01 {
            self.fiscal_stance = FiscalStance::Contractionary;
        } else {
            self.fiscal_stance = FiscalStance::Neutral;
        }

        match self.fiscal_stance {
            FiscalStance::Expansionary => {
                self.multiplier_effect = 1.2 + rand_simple() * 0.6;
            },
            FiscalStance::Contractionary => {
                self.multiplier_effect = -0.8 - rand_simple() * 0.4;
            },
            FiscalStance::Neutral => {
                self.multiplier_effect = 0.0 + rand_simple() * 0.2;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernmentBudget {
    pub budget_id: String,
    pub year: u32,
    pub total_spending: f64,
    pub spending_allocation: Vec<(String, f64)>,
    pub budget_deficit: f64,
    pub debt_sustainability: f64,
}

impl GovernmentBudget {
    pub fn new(year: u32, spending: f64) -> Self {
        Self {
            budget_id: crate::core::uuid_simple(),
            year,
            total_spending: spending,
            spending_allocation: Vec::new(),
            budget_deficit: 0.0,
            debt_sustainability: 0.0,
        }
    }

    pub fn allocate_budget(&mut self, total_revenue: f64) -> Result<()> {
        self.spending_allocation = vec![
            ("Healthcare".to_string(), 0.25 + rand_simple() * 0.10),
            ("Education".to_string(), 0.15 + rand_simple() * 0.08),
            ("Defense".to_string(), 0.10 + rand_simple() * 0.10),
            ("Social_Security".to_string(), 0.20 + rand_simple() * 0.08),
            ("Infrastructure".to_string(), 0.08 + rand_simple() * 0.07),
            ("Other".to_string(), 0.10 + rand_simple() * 0.10),
        ];

        let mut total_pct = 0.0;
        for (_, pct) in &self.spending_allocation {
            total_pct += pct;
        }

        self.budget_deficit = self.total_spending - total_revenue;
        self.debt_sustainability = (total_revenue / self.total_spending) * (1.0 - total_pct / 6.0);
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

pub fn project_fiscal_impact(fiscal_stance: &str, gdp_growth: f64) -> Result<f64> {
    let multiplier = match fiscal_stance {
        "Expansionary" => 1.5,
        "Contractionary" => -1.2,
        _ => 0.0,
    };
    Ok(multiplier * gdp_growth / 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expansionary_fiscal() {
        let mut policy = FiscalPolicy::new("Federal_Government".to_string());
        policy.assess_fiscal_policy(25_000_000_000_000.0).unwrap();
        assert!(policy.deficit_gdp_ratio > 0.0 || policy.fiscal_stance == FiscalStance::Neutral);
    }

    #[test]
    fn test_budget_allocation() {
        let mut budget = GovernmentBudget::new(2025, 5e12);
        budget.allocate_budget(4.5e12).unwrap();
        assert!(!budget.spending_allocation.is_empty());
    }
}