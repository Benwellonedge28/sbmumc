//! # SBMUMC Module 975: Climate Finance
//! 
//! Financial mechanisms and frameworks for climate action.

use serde::{Deserialize, Serialize};
use crate::core::{SbmumcError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinanceMechanism {
    GreenBonds,
    ClimateFunds,
    CarbonPricing,
    BlendedFinance,
    ClimateInsurance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateInvestment {
    pub investment_id: String,
    pub mechanism: FinanceMechanism,
    pub project_name: String,
    pub amount_usd: f64,
    pub expected_return: f64,
    pub climate_impact_tco2: f64,
    pub risk_rating: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateFinancePortfolio {
    pub portfolio_id: String,
    pub investments: Vec<ClimateInvestment>,
    pub total_investment: f64,
    pub total_impact: f64,
    pub portfolio_roi: f64,
}

impl ClimateInvestment {
    pub fn new(mechanism: FinanceMechanism, project: &str) -> Self {
        Self {
            investment_id: format!("ci_{}", uuid_simple()),
            mechanism,
            project_name: project.to_string(),
            amount_usd: 0.0,
            expected_return: 0.0,
            climate_impact_tco2: 0.0,
            risk_rating: "Medium".to_string(),
        }
    }

    pub fn configure(&mut self, amount: f64, return_rate: f64, impact: f64, risk: &str) {
        self.amount_usd = amount;
        self.expected_return = return_rate;
        self.climate_impact_tco2 = impact;
        self.risk_rating = risk.to_string();
    }

    pub fn impact_per_dollar(&self) -> f64 {
        if self.amount_usd == 0.0 { 0.0 }
        else { self.climate_impact_tco2 / self.amount_usd }
    }
}

impl ClimateFinancePortfolio {
    pub fn new() -> Self {
        Self {
            portfolio_id: format!("cfport_{}", uuid_simple()),
            investments: Vec::new(),
            total_investment: 0.0,
            total_impact: 0.0,
            portfolio_roi: 0.0,
        }
    }

    pub fn add_investment(&mut self, investment: ClimateInvestment) {
        self.investments.push(investment);
        self.compute_metrics();
    }

    pub fn compute_metrics(&mut self) {
        self.total_investment = self.investments.iter()
            .map(|i| i.amount_usd)
            .sum();
        self.total_impact = self.investments.iter()
            .map(|i| i.climate_impact_tco2)
            .sum();
        let total_returns: f64 = self.investments.iter()
            .map(|i| i.amount_usd * i.expected_return)
            .sum();
        if self.total_investment > 0.0 {
            self.portfolio_roi = total_returns / self.total_investment;
        }
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climate_investment() {
        let mut investment = ClimateInvestment::new(
            FinanceMechanism::GreenBonds,
            "Solar Farm Project",
        );
        investment.configure(100000000.0, 0.08, 50000.0, "Low");
        assert!(investment.impact_per_dollar() > 0.0);
    }

    #[test]
    fn test_climate_finance_portfolio() {
        let mut portfolio = ClimateFinancePortfolio::new();
        portfolio.add_investment(ClimateInvestment::new(
            FinanceMechanism::ClimateFunds,
            "Reforestation Initiative",
        ));
        assert!(portfolio.total_investment >= 0.0);
    }
}
